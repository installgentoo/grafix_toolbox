use super::*;
use GL::font::*;

pub struct Text<'r, 'a> {
	pub pos: Vec2,
	pub scale: f32,
	pub color: Color,
	pub text: &'a str,
	pub font: &'r Font,
}
impl<'a> Text<'_, 'a> {
	pub fn size(line: &str, f: &Font, scale: f32) -> Vec2 {
		Self::size_and_len(line, f, scale).0
	}
	pub fn lsb(l: &str, f: &Font, scale: f32) -> f32 {
		l.chars().next().map_or(0., |c| f.glyph(c).lsb) * scale
	}
	pub fn substr(line: &'a str, f: &Font, scale: f32, max_width: f32) -> (Vec2, (&'a str, &'a str)) {
		let (x, i) = line
			.char_indices()
			.scan((0., None), |(x, last_c), (i, c)| {
				let (adv, (_, _, x2, _)) = f.adv_coord(last_c, c);
				*last_c = Some(c);
				Some(*x + x2).filter(|x| x * scale <= max_width).map(|r| {
					*x += adv;
					(r, i)
				})
			})
			.fuse()
			.last()
			.map(|(x, i)| (x, i + line[i..].utf8_slice(..1).len()))
			.unwrap_or_default();

		((x * scale, scale), line.split_at(i))
	}
	pub fn adv_at(line: &str, f: &Font, scale: f32, at_glyph: usize) -> f32 {
		line.chars()
			.take(at_glyph)
			.scan((0., None), |(x, last_c), c| {
				let (adv, _) = f.adv_coord(last_c, c);
				(*x, *last_c) = (*x + adv, Some(c));
				Some(*x)
			})
			.last()
			.map_or(0., |x| x * scale)
	}
	fn size_and_len(line: &str, f: &Font, scale: f32) -> (Vec2, u32) {
		let (len, size) = line
			.chars()
			.scan((0., None), |(x, last_c), c| {
				let (adv, (_, _, x2, _)) = f.adv_coord(last_c, c);
				let x2 = *x + x2;
				(*x, *last_c) = (*x + adv, Some(c));
				Some(x2)
			})
			.enumerate()
			.last()
			.map(|(n, x2)| (n + 1, (x2, 1.).mul(scale)))
			.unwrap_or((0, (0., scale)));
		(size, u32(len))
	}
	pub fn compare(&self, crop: &Geom, r: &TextImpl) -> State {
		let Self { pos, scale, color, text, font } = *self;
		let line = *text != *r.line;
		let xyzw = (State::XYZW | State::UV).or_def(pos != r.base.pos || scale != r.scale || *crop != r.base.crop || line);
		let rgba = State::RGBA.or_def(color != r.base.color);
		let ord = State::MISMATCH.or_def(line || !ptr::eq(font, r.font));
		ord | xyzw | rgba
	}
	pub fn obj(self, crop: Geom) -> TextImpl {
		let Self { pos, scale, color, text, font } = self;
		let (size, len) = Self::size_and_len(text, font, scale);
		TextImpl {
			base: Base { pos, size, crop, color },
			scale,
			len,
			line: text.into(),
			font,
		}
	}
}

pub struct TextImpl {
	base: Base,
	scale: f32,
	len: u32,
	line: Str,
	font: *const Font,
}
impl TextImpl {
	pub fn batchable(&self, r: &Self) -> bool {
		ptr::eq(self.font, r.font)
	}
}
impl Primitive for TextImpl {
	fn base(&self) -> &Base {
		&self.base
	}
	fn write_mesh(&self, aspect: Vec2, BatchedObj { z, state, mut xyzw, mut rgba, mut uv }: BatchedObj) {
		if self.line.is_empty() {
			return;
		}

		let Self {
			base: Base { pos, color, crop: (p1, p2), .. },
			scale,
			len,
			ref line,
			font,
		} = *self;

		if state.contains(State::XYZW) {
			let font = unsafe { &*font };

			let (mut x, mut last_c) = (0., None);
			for c in line.chars() {
				let (adv, (x1, y1, x2, y2), u) = font.adv_coord_uv(&last_c, c);
				let is_empty = y1 == y2;
				last_c = Some(c);

				let ((x1, y1), (x2, y2), (u1, v1, u2, v2)) = <_>::to({
					let xy1 = pos.sum((x + x1, y1).mul(scale));
					let xy2 = pos.sum((x + x2, y2).mul(scale));
					let uv = bound_uv((p1, p2), (xy1, xy2), u);
					let xy1 = xy1.clmp(p1, p2).mul(aspect);
					let xy2 = xy2.clmp(p1, p2).mul(aspect);
					(xy1, xy2, uv).or_def(!is_empty)
				});
				let O = f16::ZERO;

				xyzw[..16].copy_from_slice(&[x1, y1, z, O, x2, y1, z, O, x2, y2, z, O, x1, y2, z, O]);
				xyzw = &mut xyzw[16..];
				uv[..8].copy_from_slice(&[u1, v1, u2, v1, u2, v2, u1, v2]);
				uv = &mut uv[8..];

				x += adv;
			}
		}

		if state.contains(State::RGBA) {
			let (r, g, b, a) = vec4(color.mul(255).clmp(0, 255).round());
			let col = &[r, g, b, a];

			for _ in 0..4 * len {
				rgba[..4].copy_from_slice(col);
				rgba = &mut rgba[4..];
			}
		}
	}
	fn batch_draw(&self, b: &VaoBind<u16>, (offset, num): (u16, u16)) {
		let s = LeakyStatic!(Shader, { Shader::pure([vs_gui__pos_col_tex, ps_gui_sdftext]) });

		let t = unsafe { &*self.font }.tex().Bind(sampler());
		let _ = Uniforms!(s, ("iTex", t));
		b.Draw((num, offset, gl::TRIANGLES));
	}

	fn vert_count(&self) -> u32 {
		self.len * 4
	}
}

SHADER!(
	ps_gui_sdftext,
	r"in vec4 glColor;
	in vec2 glUV;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D iTex;

	void main() {
		vec2 dx = vec2(.33333333 * dFdxFine(glUV.x), 0);
		float sz = textureSize(iTex, 0).x;
		float dsdf = sz * dx.x * .2;

		float l = texture(iTex, glUV - dx).r;
		float c = texture(iTex, glUV).r;
		float r = texture(iTex, glUV + dx).r;

		vec3 p = smoothstep(vec3(.5 - dsdf), vec3(.5 + dsdf), vec3(l, c, r)) * 2;

		vec4 correction = vec4(p.rgbg);
		glFragColor = glColor * correction;
	}"
);
