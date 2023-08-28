use super::obj::*;
use super::sprite::{gui__pos_col_tex_vs, sampler};
use crate::uses::{math::*, *};
use GL::{font::*, shader::*, VaoBinding};

pub struct Text<'r, 'a> {
	pub pos: Vec2,
	pub scale: f32,
	pub color: Color,
	pub text: &'a str,
	pub font: &'r Font,
}
impl<'a> Text<'_, 'a> {
	pub fn size(text: &'a str, font: &Font, scale: f32) -> Vec2 {
		Self::size_and_len(text, font, scale).0
	}
	pub fn substr(text: &'a str, font: &Font, scale: f32, max_width: f32) -> (Vec2, (&'a str, &'a str)) {
		let (size, i) = text
			.char_indices()
			.scan((-0.5 * font.char(text.chars().next().unwrap_or(' ')).coord.x(), 0 as char), |(x, last_c), (i, c)| {
				*x += font.kern(*last_c, c);
				*last_c = c;
				let ch = font.char(c);
				Some((*x, ch)).filter(|(x, ch)| (x + ch.coord.z()) * scale <= max_width).map(|r| {
					*x += ch.adv;
					(r, i)
				})
			})
			.fuse()
			.last()
			.map_or(((0., 0.), 0), |((x, g), i)| ((x + g.coord.z(), 1.).mul(scale), i));
		let i = text[i..].char_indices().nth(1).map_or(text.len(), |(l, _)| i + l);
		(size, (&text[..i], &text[i..]))
	}
	pub fn adv_at(text: &str, font: &Font, scale: f32, at_glyph: usize) -> f32 {
		text.chars()
			.take(at_glyph)
			.scan((0., 0 as char), |(x, last_c), c| {
				*x += font.kern(*last_c, c);
				*last_c = c;
				Some(*x).map(|r| {
					*x += font.char(c).adv;
					r
				})
			})
			.last()
			.map_or(0., |x| x * scale)
	}
	pub fn char_at(text: &str, font: &Font, scale: f32, at_glyph: usize) -> Glyph {
		text.chars().take(at_glyph + 1).last().map_or_else(Glyph::default, |c| {
			let Glyph { adv, coord, uv } = *font.char(c);
			let coord = coord.mul(scale);
			Glyph { adv: adv * scale, coord, uv }
		})
	}
	fn size_and_len(text: &'a str, font: &Font, scale: f32) -> (Vec2, u32) {
		let mut len = 0;
		let size = text
			.chars()
			.scan((-0.5 * font.char(text.chars().next().unwrap_or(' ')).coord.x(), 0 as char), |(x, last_c), c| {
				*x += font.kern(*last_c, c);
				*last_c = c;
				let ch = font.char(c);
				Some((*x, ch)).map(|r| {
					Some(ch).filter(|ch| !ch.is_empty()).map(|_| len += 1);
					*x += ch.adv;
					r
				})
			})
			.fuse()
			.last()
			.map_or((0., 0.), |(x, g)| (x + g.coord.z(), 1.).mul(scale));
		(size, len)
	}
	#[inline(always)]
	pub fn compare(&self, crop: &Crop, r: &TextImpl) -> State {
		let &Self { pos, scale, color, text, font } = self;
		let text = *text != *r.text;
		let xyzw = (State::XYZW | State::UV).or_def(pos != r.base.pos || scale != r.scale || *crop != r.base.crop || text);
		let rgba = State::RGBA.or_def(color != r.base.color);
		let ord = State::MISMATCH.or_def(text && !ptr::eq(font, r.font));
		ord | xyzw | rgba
	}
	pub fn obj(self, crop: Crop) -> TextImpl {
		let Self { pos, scale, color, text, font } = self;
		let (size, len) = Self::size_and_len(text, font, scale);
		TextImpl {
			base: Base { pos, size, crop, color },
			scale,
			len,
			text: text.into(),
			font: unsafe { mem::transmute(font) },
		}
	}
}

pub struct TextImpl {
	base: Base,
	scale: f32,
	len: u32,
	text: Box<str>,
	font: &'static Font,
}
impl TextImpl {
	pub fn batchable(&self, r: &Self) -> bool {
		ptr::eq(self.font, r.font)
	}
}
impl Object for TextImpl {
	fn base(&self) -> &Base {
		&self.base
	}
	fn write_mesh(&self, aspect: Vec2, (z, state, mut xyzw, mut rgba, mut uv): BatchRange) {
		if self.text.is_empty() {
			return;
		}

		let Self { base, scale, len, text, font } = self;
		let &Base {
			pos, color, crop: (crop1, crop2), ..
		} = base;

		if state.contains(State::XYZW) {
			let (aspect, s) = (aspect, *scale);

			let (mut x, mut last_c) = (-0.5 * font.char(text.chars().next().unwrap_or(' ')).coord.x(), 0 as char);
			for c in text.chars() {
				x += font.kern(last_c, c);
				last_c = c;
				let ch = font.char(c);

				if !ch.is_empty() {
					let ((x1, y1), (x2, y2), (u1, v1, u2, v2)) = <_>::to({
						let &Glyph { coord: (x1, y1, x2, y2), uv, .. } = ch;

						let xy1 = pos.sum((x + x1, y1).mul(s));
						let xy2 = pos.sum((x + x2, y2).mul(s));
						let uv = bound_uv((crop1, crop2), (xy1, xy2), uv);
						let xy1 = xy1.clmp(crop1, crop2).mul(aspect);
						let xy2 = xy2.clmp(crop1, crop2).mul(aspect);
						(xy1, xy2, uv)
					});
					const O: f16 = f16::ZERO;

					xyzw[..16].copy_from_slice(&[x1, y1, z, O, x2, y1, z, O, x2, y2, z, O, x1, y2, z, O]);
					xyzw = &mut xyzw[16..];
					uv[..8].copy_from_slice(&[u1, v1, u2, v1, u2, v2, u1, v2]);
					uv = &mut uv[8..];
				}
				x += ch.adv;
			}
		}

		if state.contains(State::RGBA) {
			let (r, g, b, a) = vec4::to(color.mul(255).clmp(0, 255).round());
			let col = &[r, g, b, a];

			for _ in 0..4 * *len {
				rgba[..4].copy_from_slice(col);
				rgba = &mut rgba[4..];
			}
		}
	}
	fn batch_draw(&self, b: &VaoBinding<u16>, (offset, num): (u16, u16)) {
		let s = LocalStatic!(Shader, { Shader::pure((gui__pos_col_tex_vs, gui_sdftext_ps)) });

		let t = self.font.tex().Bind(sampler());
		let _ = Uniforms!(s, ("tex", &t));
		b.Draw((num, offset, gl::TRIANGLES));
	}

	fn vert_count(&self) -> u32 {
		self.len * 4
	}
}

SHADER!(
	gui_sdftext_ps,
	r"in vec4 glColor;
	in vec2 glTexCoord;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex;

	void main() {
		ivec2 sz = textureSize(tex, 0);

		float dx = dFdx(glTexCoord.x) * sz.x;
		float dy = dFdy(glTexCoord.y) * sz.y;

		float toPixels = 8 * inversesqrt(dx * dx + dy * dy);

		vec2 step = vec2(dFdx(glTexCoord.x) * .5, 0);

		float pix_l = texture(tex, glTexCoord.xy - step).r - .5;
		float pix_r = texture(tex, glTexCoord.xy + step).r - .5;
		float pix_n = texture(tex, glTexCoord.xy + step * 2).r - .5;

		float pix = clamp((texture(tex, glTexCoord.xy).r - .5) * toPixels * 8 + .5, 0, 1);

		pix_l = clamp(pix_l * toPixels + 1, 0, 1);
		pix_r = clamp(pix_r * toPixels + 1, 0, 1);
		pix_n = clamp(pix_n * toPixels + 1, 0, 1);

		pix = (pix_l + pix_r + pix) / 3;

		vec4 correction = vec4(vec3(pix_l, pix_r, pix_n), pix);

		/* // Antialias
		float center = texture(tex, glTexCoord.xy).r;
		float dscale = .354; // half of 1/sqrt2
		float friends = .5; // scale value to apply to neighbours
		vec2 duv = dscale * (dFdx(v_uv) + dFdy(v_uv));
		vec4 box = vec4(v_uv-duv, v_uv+duv);
		vec4 c = samp( box.xy ) + samp( box.zw ) + samp( box.xw ) + samp( box.zy );
		float sum = 4; // 4 neighbouring samples
		rgbaOut = fontColor * (center + friends * c) / (1. + sum*friends); */

		glFragColor = glColor * correction;
	}"
);
