pub use {camera::*, mesh::*, model::*};

pub mod Screen {
	use super::*;
	struct Model {
		vao: Vao<u8>,
		_xyuv: AttrArr<i8>,
	}
	pub fn Draw() {
		GLSave!(DEPTH_TEST);
		GLDisable!(DEPTH_TEST);
		LocalStatic!(Model, {
			#[rustfmt::skip]
			let _xyuv = AttrArr::new(&[ -1, -1, 0, 0,  3, -1, 2, 0,  -1, 3, 0, 2 ][..]);
			let mut vao = Vao::new();
			vao.AttribFmt(&_xyuv, (0, 4));
			Model { vao, _xyuv }
		})
		.vao
		.Bind()
		.DrawUnindexed(3);
		GLRestore!(DEPTH_TEST);
	}
}

pub mod Skybox {
	use super::*;
	struct Model {
		vao: Vao<u8>,
		_idx: IdxArr<u8>,
		_xyz: AttrArr<i8>,
	}
	pub fn Draw() {
		LocalStatic!(Model, {
			#[rustfmt::skip]
			let _idx = IdxArr::new(&[ 0, 1, 3,  3, 1, 2,
									  4, 5, 7,  7, 5, 6,
									  0, 1, 4,  4, 1, 5,
									  3, 2, 7,  7, 2, 6,
									  2, 1, 6,  6, 1, 5,
									  3, 7, 0,  0, 7, 4, ][..]);
			#[rustfmt::skip]
			let _xyz = AttrArr::new(&[ -1,  1, 1,   1,  1, 1,   1,  1, -1,  -1,  1, -1,
									   -1, -1, 1,   1, -1, 1,   1, -1, -1,  -1, -1, -1 ][..]);
			let mut vao = Vao::new();
			vao.BindIdxs(&_idx);
			vao.AttribFmt(&_xyz, (0, 3));
			Model { vao, _idx, _xyz }
		})
		.vao
		.Bind()
		.Draw(36);
	}
}

mod camera;
mod mesh;
mod model;

use crate::{lib::*, *};
use GL::buffer::*;

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
	uniform sampler2D tex;

	void main() { glFragColor = texture(tex, glUV); }"
);
