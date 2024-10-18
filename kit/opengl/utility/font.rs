use crate::{lib::*, *};
use GL::{Tex2d, RED};

derive_common_OBJ! {
pub struct Glyph {
	pub adv: f32,
	pub coord: Vec4,
	pub uv: Vec4,
}}

#[cfg_attr(feature = "adv_fs", derive(ser::Serialize, ser::Deserialize))]
#[derive(Default, Debug)]
pub struct Font {
	pub glyphs: HashMap<char, Glyph>,
	pub kerning: HashMap<char, HashMap<char, f32>>,
	pub midline: f32,
	tex: Option<Tex2d<RED, u8>>,
}
impl Font {
	pub fn tex(&self) -> &Tex2d<RED, u8> {
		self.tex.as_ref().unwrap_or_else(|| LocalStatic!(Tex2d<RED, u8>))
	}
	pub fn glyph(&self, c: char) -> Option<&Glyph> {
		self.glyphs.get(&c)
	}
	pub fn kern(&self, l: char, r: char) -> f32 {
		if self.kerning.is_empty() {
			return 0.;
		}
		(|| Some(*self.kerning.get(&l)?.get(&r)?))().unwrap_or(0.)
	}
	#[cfg(all(feature = "adv_fs", feature = "sdf"))]
	pub fn new_cached(name: &str, alphabet: impl AsRef<str>) -> Self {
		let alphabet = alphabet.as_ref();
		let alph_chksum = chksum::const_fnv1(alphabet.as_bytes()).to_string();
		let cache = format!("{name}.{alph_chksum}.font.z");
		if let Ok(d) = FS::Load::Archive(&cache) {
			if let Ok(font) = ser::SERDE::FromVec(&d) {
				return font;
			}
		}

		let font: Res<_> = (|| {
			let file = FS::Load::File(format!("res/{name}.ttf"))?;
			let font = Self::new(&file, alphabet)?;
			let _ = ser::SERDE::ToVec(&font).map(|v| FS::Save::Archive((cache, v, 3)));
			Ok(font)
		})();
		font.explain_err(|| format!("Cannot load font {name:?}")).warn()
	}
	#[cfg(feature = "sdf")]
	pub fn new(font_data: &[u8], alphabet: &str) -> Res<Self> {
		use {super::sdf::*, crate::math::*, rusttype as ttf};
		let (glyph_size, border, supersample) = (28, 2, 16);
		let alphabet = || alphabet.chars();
		let glyph_divisor = 2. / f32(glyph_size + border * 2);
		let divisor = glyph_divisor / f32(supersample);
		let scale = ttf::Scale::uniform(f32(glyph_size * supersample));

		let font = Res(ttf::Font::try_from_bytes(font_data))?;

		let kerning = alphabet()
			.filter_map(|c| {
				let kern = alphabet()
					.filter_map(|g| {
						let k = font.pair_kerning(scale, c, g) * divisor;
						Some((g, k)).filter(|_| k != 0.)
					})
					.collect::<HashMap<char, f32>>();
				Some((c, kern)).filter(|(_, k)| !k.is_empty())
			})
			.collect::<HashMap<_, _>>();
		DEBUG!("Font kerning: {kerning:?}");

		let mut glyphs = vec![];
		let mut sdf = SdfGenerator::default();
		let mut topline = 0.;
		let mut bottomline = 0.;
		let alphabet = alphabet()
			.map(|c| {
				let g = font.glyph(c).scaled(scale);
				let (adv, lsb) = {
					let m = g.h_metrics();
					(m.advance_width * divisor, m.left_side_bearing)
				};
				let g = g.positioned(ttf::point(0., 0.));
				let bb = g.pixel_bounding_box().map_or((0., 0., 0., 0.), |bb| {
					let (w, h, b) = {
						let (w, h) = (bb.max.x, bb.max.y).sub((bb.min.x, bb.min.y));
						ASSERT!(w != 0 && h != 0, "Corrupt font data");
						let b = border * supersample;
						ulVec3((w + b * 2, h + b * 2, b))
					};
					let (w, h, data) = {
						let mut data = vec![0; w * h];
						g.draw(|x, y, v| data[w * (b + usize(y)) + b + usize(x)] = u8((v * 255.).min(255.)));
						let sdf = sdf
							.generate::<RED, u8, RED>(&Tex2d::new((w, h), &data[..]), supersample, border)
							.Cast::<RED, u8>(supersample);
						let p = sdf.param;
						(p.w, p.h, sdf.Save::<RED, u8>(0))
					};
					let (x, y) = {
						let (x, y) = Vec2((bb.min.x, -bb.max.y));
						(x - lsb, y).mul(divisor)
					};
					let (x1, y1, x2, y2) = (x, y, x, y).sum((0., 0., f32(w), f32(h)).mul(glyph_divisor));
					glyphs.push((c, ImgBox { w, h, data }));
					topline = y2.max(topline);
					bottomline = y1.min(bottomline);
					(x1, y1, x2, y2)
				});
				(c, bb, adv)
			})
			.collect_vec();

		let (mut atlas, _rejects) = GL::atlas::pack_into_atlas::<_, _, RED, _>(glyphs, GL::MAX_TEXTURE_SIZE(), GL::MAX_TEXTURE_SIZE());
		ASSERT!(_rejects.is_empty(), "Cannot fit font into biggest gpu texture");

		let mut tex = None;
		let h = topline - bottomline;
		let midline = -bottomline / h;
		let glyphs = alphabet
			.into_iter()
			.map(|(c, coord, adv)| {
				let uv = atlas.remove(&c).map_or((0., 0., 0., 0.), |e| {
					tex = Some(e.atlas);
					e.region
				});
				let empty = uv.x() == uv.z() || uv.y() == uv.w();
				let adv = adv / h;
				let coord = coord.div(h).sum((0., midline, adv.or_def(empty), midline));
				(c, Glyph { adv, coord, uv })
			})
			.collect();

		let tex = Some(Rc::try_unwrap(Res(tex)?).valid());

		Ok(Self { glyphs, kerning, midline, tex })
	}
}

#[cfg(feature = "sdf")]
struct ImgBox {
	w: i32,
	h: i32,
	data: Box<[u8]>,
}
#[cfg(feature = "sdf")]
impl Eq for ImgBox {}
#[cfg(feature = "sdf")]
impl PartialEq for ImgBox {
	fn eq(&self, r: &Self) -> bool {
		if self.w != r.w && self.h != r.h {
			return false;
		}
		let diff = self.data.iter().zip(&r.data[..]).map(|(&l, &r)| (i32(l) - i32(r)).abs()).max().unwrap_or(0);
		diff < 5
	}
}
#[cfg(feature = "sdf")]
impl GL::atlas::Tile<u8> for ImgBox {
	fn w(&self) -> i32 {
		self.w
	}
	fn h(&self) -> i32 {
		self.h
	}
	fn data(&self) -> &[u8] {
		&self.data
	}
}
