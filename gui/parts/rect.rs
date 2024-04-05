use super::obj::*;
use crate::GL::{shader::*, VaoBinding};
use crate::{lib::*, math::*};

pub struct Rect {
	pub pos: Vec2,
	pub size: Vec2,
	pub color: Color,
}
impl Rect {
	#[inline(always)]
	pub fn compare(&self, crop: &Crop, r: &RectImpl) -> State {
		let &Self { pos, size, color } = self;
		let xyzw = State::XYZW.or_def(geom_cmp(pos, size, crop, &r.base));
		let rgba = State::RGBA.or_def(color != r.base.color);
		let ord = State::MISMATCH.or_def(!rgba.is_empty() && (!opaque(color) != r.ordered()));
		ord | xyzw | rgba
	}
	pub fn obj(self, crop: Crop) -> RectImpl {
		let Self { pos, size, color } = self;
		RectImpl { base: Base { pos, size, crop, color } }
	}
}
pub struct RectImpl {
	base: Base,
}
impl RectImpl {
	pub fn batchable(&self, r: &Self) -> bool {
		self.ordered() == r.ordered()
	}
}
impl Object for RectImpl {
	fn base(&self) -> &Base {
		&self.base
	}
	fn write_mesh(&self, to_clip: Vec2, BatchedObj { z, state, xyzw, rgba, .. }: BatchedObj) {
		if state.contains(State::XYZW) {
			let ((x1, y1), (x2, y2)) = <_>::to({
				let (to_clip, (crop1, crop2)) = (to_clip, self.base.bound_box());
				(crop1.mul(to_clip), crop2.mul(to_clip))
			});
			const O: f16 = f16::ZERO;

			xyzw[..16].copy_from_slice(&[x1, y1, z, O, x2, y1, z, O, x2, y2, z, O, x1, y2, z, O]);
		}

		if state.contains(State::RGBA) {
			let (r, g, b, a) = vec4::to(self.base.color.mul(255).clmp(0, 255).round());

			rgba[..16].copy_from_slice(&[r, g, b, a, r, g, b, a, r, g, b, a, r, g, b, a]);
		}
	}
	fn batch_draw(&self, b: &VaoBinding<u16>, (offset, num): (u16, u16)) {
		let s = LocalStatic!(Shader, { Shader::pure([vs_gui__pos_col, ps_gui__col]) });

		let _ = s.Bind();
		b.Draw((num, offset, gl::TRIANGLES));
	}

	fn ordered(&self) -> bool {
		!opaque(self.base().color)
	}
}

SHADER!(
	vs_gui__pos_col,
	r"layout(location = 0) in vec4 Position;
	layout(location = 1) in vec4 Color;
	out vec4 glColor;

	void main() {
		gl_Position = vec4(Position.xyz, 1);
		glColor = Color;
	}"
);
SHADER!(
	ps_gui__col,
	r"in vec4 glColor;
	layout(location = 0) out vec4 glFragColor;

	void main() { glFragColor = glColor; }"
);
