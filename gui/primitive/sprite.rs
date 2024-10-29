use super::*;

pub struct Sprite<'r, S> {
	pub pos: Vec2,
	pub size: Vec2,
	pub color: Color,
	pub tex: &'r VTex2d<S, u8>,
}
impl<S: TexSize> Sprite<'_, S> {
	pub fn compare(&self, crop: &Geom, r: &SpriteImpl<S>) -> State {
		let &Self { pos, size, color, tex: tex_new } = self;
		let xyzw = (State::XYZW | State::UV).or_def(geom_cmp(pos, size, crop, &r.base));
		let rgba = State::RGBA.or_def(color != r.base.color);
		let ord = State::MISMATCH.or_def(!ptr::eq(r.tex, tex_new) || (!rgba.is_empty() && ordering_cmp::<S, _>(color, r)));
		ord | xyzw | rgba
	}
	pub fn obj(self, crop: Geom) -> SpriteImpl<S> {
		let Self { pos, size, color, tex } = self;
		SpriteImpl { base: Base { pos, size, crop, color }, tex }
	}
}
pub struct SpriteImpl<S> {
	base: Base,
	tex: *const VTex2d<S, u8>,
}
impl<S: TexSize> SpriteImpl<S> {
	pub fn batchable(&self, r: &Self) -> bool {
		self.ordered() == r.ordered() && atlas_cmp(self.tex, r.tex)
	}
}
impl<S: TexSize> Primitive for SpriteImpl<S> {
	fn base(&self) -> &Base {
		&self.base
	}
	fn write_mesh(&self, to_clip: Vec2, BatchedObj { z, state, xyzw, rgba, uv }: BatchedObj) {
		if state.contains(State::XYZW | State::UV) {
			let ((x1, y1), (x2, y2), (u1, v1, u2, v2)) = <_>::to({
				let (to_clip, crop @ (p1, p2), &Base { pos, size, .. }) = (to_clip, self.base.bound_box(), self.base());
				let (xy1, xy2, uv) = (pos, pos.sum(size), unsafe { &*self.tex }.region);
				let uv = bound_uv(crop, (xy1, xy2), uv);

				(p1.mul(to_clip), p2.mul(to_clip), uv)
			});
			let O = f16::ZERO;

			if state.contains(State::XYZW) {
				xyzw[..16].copy_from_slice(&[x1, y1, z, O, x2, y1, z, O, x2, y2, z, O, x1, y2, z, O]);
			}

			if state.contains(State::UV) {
				uv[..8].copy_from_slice(&[u1, v1, u2, v1, u2, v2, u1, v2]);
			}
		}

		if state.contains(State::RGBA) {
			let (r, g, b, a) = vec4::to(self.base.color.mul(255).clmp(0, 255).round());

			rgba[..16].copy_from_slice(&[r, g, b, a, r, g, b, a, r, g, b, a, r, g, b, a]);
		}
	}
	fn batch_draw(&self, b: &VaoBinding<u16>, (offset, num): (u16, u16)) {
		let s = LeakyStatic!(Shader, { Shader::pure([vs_gui__pos_col_tex, ps_gui__col_tex]) });

		let t = unsafe { &*self.tex }.atlas.Bind(sampler());
		let _ = Uniforms!(s, ("tex", t));
		b.Draw((num, offset, gl::TRIANGLES));
	}

	fn ordered(&self) -> bool {
		S::TYPE == gl::RGBA || Primitive::ordered(self)
	}
}

pub fn sampler() -> &'static Sampler {
	LeakyStatic!(Rc<Sampler>, { Sampler::linear() })
}

SHADER!(
	vs_gui__pos_col_tex,
	r"layout(location = 0) in vec4 Position;
	layout(location = 1) in vec4 Color;
	layout(location = 2) in vec2 TexCoord;
	out vec4 glColor;
	out vec2 glUV;

	void main() {
		gl_Position = vec4(Position.xyz, 1);
		glColor = Color;
		glUV = TexCoord;
	}"
);
SHADER!(
	ps_gui__col_tex,
	r"in vec4 glColor;
	in vec2 glUV;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex;

	void main() { glFragColor = glColor * texture(tex, glUV); }"
);
