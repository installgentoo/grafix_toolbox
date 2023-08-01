use crate::uses::*;
use GL::{atlas::*, Tex2d, RED};

derive_common_OBJ! {
pub struct Glyph {
	pub adv: f32,
	pub coord: Vec4,
	pub uv: Vec4,
}}
impl Glyph {
	pub fn is_empty(&self) -> bool {
		self.uv.0 == self.uv.2
	}
}

#[cfg_attr(feature = "adv_fs", derive(Serialize, Deserialize))]
#[derive(Default)]
pub struct Font {
	pub glyphs: HashMap<char, Glyph>,
	pub kerning: HashMap<char, HashMap<char, f32>>,
	pub midline: f32,
	tex: Option<Tex2d<RED, u8>>,
}
impl Font {
	pub fn tex(&self) -> &Tex2d<RED, u8> {
		self.tex.as_ref().unwrap_or_else(|| UnsafeOnce!(Tex2d<RED, u8>, { Def() }))
	}
	pub fn char(&self, c: char) -> &Glyph {
		let g = &self.glyphs;
		g.get(&c).unwrap_or_else(|| {
			DEBUG!("No character {c:?} in font");
			static E: Glyph = Glyph {
				adv: 0.,
				coord: (0., 0., 0., 0.),
				uv: (0., 0., 0., 0.),
			};
			&E
		})
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
		let cache = &format!("{name}.{alph_chksum}.font.z");
		if let Ok(d) = FS::Load::Archive(cache) {
			if let Ok(font) = SERDE::FromVec(&d) {
				return font;
			}
		}

		let font: Res<_> = (|| {
			let file = FS::Load::File(format!("res/{name}.ttf"))?;
			let font = Self::new(file, alphabet)?;
			let _ = SERDE::ToVec(&font).map(|v| FS::Save::Archive((cache, v)));
			Ok(font)
		})();
		OR_DEFAULT!(font, "Could not load font {name}: {}")
	}
	#[cfg(feature = "sdf")]
	pub fn new(font_data: impl Borrow<Vec<u8>>, alphabet: impl AsRef<str>) -> Res<Self> {
		use {super::sdf::*, math::*, rusttype as ttf};
		let alphabet = alphabet.as_ref();
		let (glyph_size, border, supersample) = (28, 2, 16);
		let alphabet = || alphabet.chars();
		let glyph_divisor = 2. / f32(glyph_size + border * 2);
		let divisor = glyph_divisor / f32(supersample);
		let scale = ttf::Scale::uniform(f32(glyph_size * supersample));

		let font = Res(ttf::Font::try_from_bytes(font_data.borrow()))?;

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
		let mut sdf = SdfGenerator::new();
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
						let sdf = sdf.generate(Tex2d::<RED, u8>::new((w, h), &data), supersample, border * 2);
						let p = sdf.param;
						(p.w, p.h, sdf.Save::<RED, u8>(0))
					};
					let (x, y) = {
						let (x, y, b) = Vec3((bb.min.x, -bb.max.y, b));
						(x - lsb, y).sub(b).mul(divisor)
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

		let (mut atlas, _rejects) = pack_into_atlas::<_, _, RED, _>(glyphs, GL::MAX_TEXTURE_SIZE(), GL::MAX_TEXTURE_SIZE());
		ASSERT!(_rejects.is_empty(), "Couldn't fit font into system texture size");

		let mut tex = None;
		let h = topline - bottomline;
		let midline = -bottomline / h;
		let glyphs = alphabet
			.into_iter()
			.map(|(c, coord, adv)| {
				let uv = atlas.remove(&c).map_or((0., 0., 0., 0.), |e| {
					tex = Some(e.tex);
					e.region
				});
				let empty = uv.x() == uv.z() || uv.y() == uv.w();
				let adv = adv / h;
				let coord = coord.div(h).sum((0., midline, adv.or_def(empty), midline));
				(c, Glyph { adv, coord, uv })
			})
			.collect();

		let tex = Some(Rc::try_unwrap(Res(tex)?).unwrap());

		Ok(Self { glyphs, kerning, midline, tex })
	}
}

struct ImgBox {
	w: i32,
	h: i32,
	data: Vec<u8>,
}
impl Eq for ImgBox {}
impl PartialEq for ImgBox {
	fn eq(&self, r: &Self) -> bool {
		if self.w != r.w && self.h != r.h {
			return false;
		}
		let diff = self.data.iter().zip(&r.data).map(|(&l, &r)| (i32(l) - i32(r)).abs()).max().unwrap_or(0);
		diff < 5
	}
}
impl Tile<u8> for ImgBox {
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
