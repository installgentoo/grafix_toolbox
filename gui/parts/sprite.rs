use super::obj::*;
use crate::GL::{atlas::VTex2d, shader::*, tex::*, VaoBinding};
use crate::{lib::*, math::*};

pub struct Sprite<'r, S> {
	pub pos: Vec2,
	pub size: Vec2,
	pub color: Color,
	pub tex: &'r VTex2d<S, u8>,
}
impl<S: TexSize> Sprite<'_, S> {
	#[inline(always)]
	pub fn compare(&self, crop: &Crop, r: &SpriteImpl<S>) -> State {
		let &Self { pos, size, color, tex } = self;
		let xyzw = (State::XYZW | State::UV).or_def(geom_cmp(pos, size, crop, &r.base));
		let rgba = State::RGBA.or_def(color != r.base.color);
		let _tex = State::UV.or_def(!ptr::eq(tex, r.tex));
		let ord = State::MISMATCH.or_def((!_tex.is_empty() && atlas_cmp(tex, r.tex)) || (!rgba.is_empty() && ordering_cmp::<S, _>(color, r)));
		ord | xyzw | rgba | _tex
	}
	pub fn obj(self, crop: Crop) -> SpriteImpl<S> {
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
impl<S: TexSize> Object for SpriteImpl<S> {
	fn base(&self) -> &Base {
		&self.base
	}
	fn write_mesh(&self, aspect: Vec2, BatchedObj { z, state, xyzw, rgba, uv }: BatchedObj) {
		if state.contains(State::XYZW | State::UV) {
			let ((x1, y1), (x2, y2), (u1, v1, u2, v2)) = <_>::to({
				let (aspect, (crop1, crop2), &Base { pos, size, .. }) = (aspect, self.base.bound_box(), self.base());
				let (xy1, xy2, uv) = (pos, pos.sum(size), unsafe { &*self.tex }.region);
				let uv = bound_uv((crop1, crop2), (xy1, xy2), uv);

				(crop1.mul(aspect), crop2.mul(aspect), uv)
			});
			const O: f16 = f16::ZERO;

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
		let s = LocalStatic!(Shader, { Shader::pure([vs_gui__pos_col_tex, ps_gui__col_tex]) });

		let t = unsafe { &*self.tex }.tex.Bind(sampler());
		let _ = Uniforms!(s, ("tex", &t));
		b.Draw((num, offset, gl::TRIANGLES));
	}

	fn ordered(&self) -> bool {
		S::TYPE == gl::RGBA || Object::ordered(self)
	}
}

pub fn sampler() -> &'static Sampler {
	LocalStatic!(Rc<Sampler>, { Sampler::linear() })
}

SHADER!(
	vs_gui__pos_col_tex,
	r"layout(location = 0) in vec4 Position;
	layout(location = 1) in vec4 Color;
	layout(location = 2) in vec2 TexCoord;
	out vec4 glColor;
	out vec2 glTexUV;

	void main() {
		gl_Position = vec4(Position.xyz, 1);
		glColor = Color;
		glTexUV = TexCoord;
	}"
);
SHADER!(
	ps_gui__col_tex,
	r"in vec4 glColor;
	in vec2 glTexUV;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex;

	void main() { glFragColor = glColor * texture(tex, glTexUV); }"
);
