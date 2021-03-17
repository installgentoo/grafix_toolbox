use super::obj::*;
use super::sprite::{gui__pos_col_tex_vs, sampler};
use super::sprite9::{sprite9_idxs, write_sprite9};
use crate::uses::{math::*, *};
use crate::GL::{shader::*, window::*, VTex2d, VaoBinding, RGBA};

pub struct Frame9<'a> {
	pub pos: Vec2,
	pub size: Vec2,
	pub corner: f32,
	pub color: Color,
	pub theme: &'a VTex2d<RGBA, u8>,
}
impl<'a> Frame9<'a> {
	#[inline(always)]
	pub fn compare(&self, crop: &Crop, r: &Frame9Impl) -> State {
		let &Self { pos, size, corner, color, theme } = self;
		let xyzw = (State::XYZW | State::UV).or_def(geom_cmp(pos, size, crop, &r.base) || corner != r.corner);
		let rgba = State::RGBA.or_def(color != r.base.color);
		let tex = State::UV.or_def(!ptr::eq(theme, r.tex));
		let ord = State::MISMATCH.or_def(!tex.is_empty() && atlas_cmp(theme, r.tex));
		ord | xyzw | rgba | tex
	}
	pub fn obj(self, crop: Crop) -> Frame9Impl {
		let Self { pos, size, corner, color, theme } = self;
		Frame9Impl {
			base: Base { pos, size, crop, color },
			corner,
			tex: theme,
		}
	}
}
pub struct Frame9Impl {
	base: Base,
	corner: f32,
	tex: *const VTex2d<RGBA, u8>,
}
impl Frame9Impl {
	pub fn batchable(&self, r: &Self) -> bool {
		self.tex == r.tex
	}
}
impl Object for Frame9Impl {
	fn base(&self) -> &Base {
		&self.base
	}
	fn write_mesh(&self, range: BatchRange) {
		let (crop, &Base { pos, size, color, .. }) = (self.base.bound_box(), self.base());
		let c = size.x().min(size.y()) * self.corner.min(0.5).max(0.);
		write_sprite9((Window::aspect(), pos, size, (c, c), crop, (0., 0., 1., 1.), color), range);
	}
	fn batch_draw(&self, b: &VaoBinding<u16>, (offset, num): (u16, u16)) {
		let s = UnsafeOnce!(Shader, { EXPECT!(Shader::new((gui__pos_col_tex_vs, gui__frame_ps))) });

		let tex = unsafe { &*self.tex };
		let t = tex.tex.Bind(sampler());
		let (x, y, w, _) = tex.region;
		let _ = Uniforms!(s, ("src", &t), ("theme_coords", (x, y, w - x)));
		b.Draw((num, offset, gl::TRIANGLES));
	}

	fn vert_count(&self) -> u32 {
		16
	}
	fn gen_idxs(&self, (start, size): (u16, u16)) -> Vec<u16> {
		sprite9_idxs((start, size))
	}
}

SHADER!(
	gui__frame_ps,
	r"#version 330 core
in vec4 glColor;
in vec2 glTexCoord;
layout(location = 0)out vec4 glFragColor;
uniform sampler2D src;
uniform vec3 theme_coords;

void main()
{
float d = min(0.9, length(glTexCoord));
vec4 c = texture(src, theme_coords.xy + vec2(d * theme_coords.z, 0.));
glFragColor = glColor * c;
}"
);
