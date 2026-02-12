use crate::{lib::*, math::*};
use GL::*;

#[derive_as_obj(Default)]
pub struct Glyph {
	pub adv: f32,
	pub lsb: f32,
	pub coord: Vec4,
	pub uv: Vec4,
}

#[derive_as_ser(Default, Debug)]
pub struct Font {
	glyphs: HashMap<char, Glyph>,
	kerning: HashMap<char, HashMap<char, f32>>,
	tex: Option<Tex2d<RED, u8>>,
}
impl Font {
	pub fn tex(&self) -> &Tex2d<RED, u8> {
		self.tex.as_ref().unwrap_or_else(|| LeakyStatic!(Tex2d<RED, u8>, { Tex2d::none() }))
	}
	pub fn glyph(&self, c: char) -> &Glyph {
		self.glyphs.get(&c).unwrap_or_else(|| {
			WARN!("No glyph {c:?} in Font");
			self.glyphs.get(&' ').fail()
		})
	}
	pub fn adv_coord(&self, prev: &Option<char>, c: char) -> (f32, Vec4) {
		let (a, c, _) = self.adv_coord_uv(prev, c);
		(a, c)
	}
	pub fn adv_coord_uv(&self, prev: &Option<char>, c: char) -> (f32, Vec4, Vec4) {
		let g = self.glyph(c);
		let k = prev.map_or(0., |p| self.kern(p, c));
		let x = k;
		(g.adv + k, g.coord.sum((x, 0, x, 0)), g.uv)
	}
	pub fn kern(&self, l: char, r: char) -> f32 {
		(|| Some(*self.kerning.get(&l)?.get(&r)?))().unwrap_or(0.)
	}
	#[cfg(all(feature = "adv_fs", feature = "sdf"))]
	pub fn new_cached(name: &str, alphabet: impl AsRef<str>) -> Self {
		let alphabet = alphabet.as_ref();
		let alph_chksum = chksum::const_fnv1(alphabet.as_bytes()).to_string();
		let cache = format!("{name}.{alph_chksum}.font.z");
		if let Ok(font) = FS::Load::Archive(&cache).and_then(ser::from_vec) {
			return font;
		}

		(|| -> Res<_> {
			format!("res/{name}.ttf")
				.pipe(FS::Load::File)?
				.pipe_as(|data| Self::new(data, alphabet))?
				.tap(|font| ser::to_vec(font).map(|v| FS::Save::Archive((cache, v, 3))).warn())
				.pipe(Ok)
		})()
		.explain_err(|| format!("Cannot load font {name:?}"))
		.warn()
	}
	#[cfg(feature = "sdf")]
	pub fn new(font_data: &[u8], alphabet: &str) -> Res<Self> {
		use {super::sdf::*, crate::math::*, rusttype as ttf};
		let (g_size, border, supersample) = (18, 2, 8);
		let alphabet = || alphabet.chars();
		let g_size = f32(g_size * supersample).pipe(ttf::Scale::uniform);
		let border_sdf = border * supersample * supersample;
		let border_keep = border * supersample;

		let font = ttf::Font::try_from_bytes(font_data).res()?;

		let (mut sdf, mut glyphs) = (Def::<SdfGenerator>(), vec![]);
		let (min_y, div) = {
			let v = font.v_metrics(g_size);
			(v.descent, 1. / (v.ascent - v.descent).abs())
		};
		let geometry = alphabet()
			.map(|c| {
				let g = font.glyph(c).scaled(g_size);
				let (adv, lsb) = {
					let m = g.h_metrics();
					(m.advance_width, m.left_side_bearing)
				};
				let g = g.positioned(ttf::point(0., min_y));
				let bb = g.pixel_bounding_box().map_or(Vec4(0), |bb| {
					let bb = (bb.min.x, -bb.max.y, bb.max.x, -bb.min.y);
					let s = bb.zw().sub(bb.xy()).abs();
					let s_real = {
						ASSERT!(s.gt(0).all(), "Corrupt font data");
						let b = border_sdf;
						let (w, h) = ulVec2(s.sum(b * 2));
						let data = vec![0; w * h].tap(|d| g.draw(|x, y, v| d[w * (h - usize(b + i32(y))) + usize(b + i32(x))] = u8((v * 255.).min(255.))));
						let sdf = sdf.generate::<RED, u8, RED>(&Tex2d::new((w, h), &data[..]), b);
						let b = b - border_keep;
						let sdf = sdf.Cut(iVec4((0, 0, w, h)).sum((b, b, -b, -b)));
						let uImage::<RED> { w, h, data, .. } = sdf.Cast::<RED, u8>(supersample).into();
						glyphs.push((c, ImgBox { w, h, data }));
						Vec2((w, h).mul(supersample))
					};
					let o = Vec2(s).sub(s_real).mul(0.5);
					let ((x, y), (w, h)) = (Vec2(bb.xy()).sum(o), s_real);
					let _half_texel @ b = 0.5 * f32(supersample);
					(x, y, x + w, y + h).sum((b, b, -b, -b))
				});
				(c, bb, (adv, lsb))
			})
			.collect_vec();

		let (mut atlas, _rejects) = GL::atlas::pack_into_atlas::<_, _, RED, _>(glyphs, GL::MAX_TEXTURE_SIZE(), GL::MAX_TEXTURE_SIZE());
		ASSERT!(_rejects.is_empty(), "GPU cannot fit font texture");

		let kerning = alphabet()
			.filter_map(|c| {
				let kern = alphabet()
					.filter_map(|g| {
						let k = font.pair_kerning(g_size, c, g) * div;
						Some((g, k)).filter(|_| !k.eps_eq(0.))
					})
					.collect::<HashMap<_, _>>();
				Some((c, kern)).filter(|(_, k)| !k.is_empty())
			})
			.collect();
		DEBUG!("Font kerning: {kerning:?}");

		let mut tex = None;
		let glyphs = geometry
			.into_iter()
			.map(|(c, coord, adv_lsb)| {
				let (adv, lsb) = adv_lsb.mul(div);
				let Some(e) = atlas.remove(&c) else {
					return (c, Glyph { adv, coord: (0., 0., adv, 0.), ..Def() });
				};
				let uv = e.region.sum((0.5, 0.5, -0.5, -0.5).div(e.atlas.whdl().xyxy()));
				tex = Some(e.atlas);
				let coord = coord.sub((-lsb, 0, -lsb, 0)).mul(div);
				(c, Glyph { adv, lsb, coord, uv })
			})
			.collect();

		debug_assert!({
			#[cfg(feature = "png")]
			{
				let _i: uImage<RED> = (&**tex.as_valid()).into();
				_i.save(format!("{}.png", chksum::ref_UUID(&_i)));
			}
			true
		});

		let tex = tex.ok_or("Font has no atlas")?.pipe(Rc::into_inner);

		Self { glyphs, kerning, tex }.pipe(Ok)
	}
}
unsafe impl Sync for Font {}

#[cfg(feature = "sdf")]
struct ImgBox {
	w: u32,
	h: u32,
	data: Box<[u8]>,
}
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
		i32(self.w)
	}
	fn h(&self) -> i32 {
		i32(self.h)
	}
	fn data(&self) -> &[u8] {
		&self.data
	}
}
