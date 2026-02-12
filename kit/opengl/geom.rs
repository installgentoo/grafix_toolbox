pub use {camera::*, mesh::*, model::*, vao::*};

pub type AnyMesh = Box<dyn AnyMeshT>;
impl Default for AnyMesh {
	fn default() -> Self {
		Mesh::make_sphere(0.1, 6)
	}
}

pub struct Mesh<I> {
	pub geom: Geometry<I>,
	pub draw: (I, GLenum),
}

pub mod Screen {
	struct Model {
		vao: Vao<u8>,
		_xyuv: AttrArr<i8>,
	}
	pub fn Draw() {
		GLSave!(DEPTH_TEST);
		GLDisable!(DEPTH_TEST);
		LeakyStatic!(Model, {
			#[rustfmt::skip]
			let _xyuv = AttrArr::new(&[ -1, -1, 0, 0,  3, -1, 2, 0,  -1, 3, 0, 2 ][..]);
			let mut vao = Vao::default();
			vao.AttribFmt(&_xyuv, (0, 4));
			Model { vao, _xyuv }
		})
		.vao
		.Bind()
		.DrawUnindexed(3);
		GLRestore!(DEPTH_TEST);
	}
	use super::*;
}

pub mod Skybox {
	pub fn Draw() {
		LeakyStatic!(Geometry<u8>, {
			#[rustfmt::skip]
			let idx = &[0, 1, 3,  3, 1, 2,
						4, 5, 7,  7, 5, 6,
						0, 1, 4,  4, 1, 5,
						3, 2, 7,  7, 2, 6,
						2, 1, 6,  6, 1, 5,
						3, 7, 0,  0, 7, 4_u8][..];
			#[rustfmt::skip]
			let xyz = &[-1,  1, 1,   1,  1, 1,   1,  1, -1,  -1,  1, -1,
						-1, -1, 1,   1, -1, 1,   1, -1, -1,  -1, -1, -1_i8][..];
			Geometry::new(idx, (3, xyz))
		})
		.Draw(36);
	}
	use super::*;
}

mod args;
mod camera;
mod mesh;
mod model;
mod vao;

use crate::lib::*;
use {GL::buffer::*, GL::spec::*, args::*};

SHADER!(
	vs_mesh__2d_screen,
	r"layout(location = 0) in vec4 Position;
	out vec2 glUV;

	void main() {
		gl_Position = vec4(Position.xy, 0, 1);
		glUV = Position.zw;
	}"
);

SHADER!(
	ps_mesh__2d_screen,
	r"in vec2 glUV;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D iTex;

	void main() { glFragColor = texture(iTex, glUV); }"
);
