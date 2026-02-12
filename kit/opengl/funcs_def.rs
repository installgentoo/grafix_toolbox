use super::control::{consts::Get, funcs::*};
use crate::lib::*;

CONST!(MAX_TEXTURE_IMAGE_UNITS, i32);
CONST!(MAX_COMBINED_TEXTURE_IMAGE_UNITS, i32);
CONST!(MAX_TEXTURE_BUFFER_SIZE, i32);
CONST!(MAX_TEXTURE_SIZE, i32);
CONST!(MAX_UNIFORM_BUFFER_BINDINGS, i32);
CONST!(MAX_SHADER_STORAGE_BUFFER_BINDINGS, i32);
CONST!(MAX_UNIFORM_BLOCK_SIZE, i32);
CONST!(MAX_SHADER_STORAGE_BLOCK_SIZE, i32);

FUNC!(gl, Viewport, i32, i32, i32, i32);
FUNC!(gl, BlendFunc, GLenum, GLenum);
FUNC!(gl, BlendFuncSeparate, GLenum, GLenum, GLenum, GLenum);
FUNC!(gl, BlendEquation, GLenum);
FUNC!(gl, DepthFunc, GLenum);
FUNC!(ext, PixelStorePack, i32);
FUNC!(ext, PixelStoreUnpack, i32);

mod ext {
	pub fn PixelStorePack(v: i32) {
		unsafe { gl::PixelStorei(gl::PACK_ALIGNMENT, v) }
	}
	pub fn PixelStoreUnpack(v: i32) {
		unsafe { gl::PixelStorei(gl::UNPACK_ALIGNMENT, v) }
	}
}

SWITCH!(DEPTH_WRITEMASK, gl::DepthMask(gl::TRUE), gl::DepthMask(gl::FALSE), DEFAULT_TRUE);
pub use DEPTH_WRITEMASK as DEPTH_WRITABLE;

SWITCH!(MULTISAMPLE, DEFAULT_TRUE);

SWITCH!(DEBUG_OUTPUT);
SWITCH!(DEBUG_OUTPUT_SYNCHRONOUS);

SWITCH!(DEPTH_TEST);
SWITCH!(BLEND);
SWITCH!(CULL_FACE);
SWITCH!(TEXTURE_CUBE_MAP_SEAMLESS);
SWITCH!(PROGRAM_POINT_SIZE);
SWITCH!(SAMPLE_SHADING);

#[macro_export]
macro_rules! GLEnable {
	($f: ty) => {{ use $crate::GL::states::*; <$f>::Enable(); }};
	($f: ty, $($n: ty),+) => {{
		use $crate::GL::states::*;
		<$f>::Enable();
		GLEnable!($($n),+);
	}};
}

#[macro_export]
macro_rules! GLDisable {
	($f: ty) => {{ use $crate::GL::states::*; <$f>::Disable(); }};
	($f: ty, $($n: ty),+) => {{
		use $crate::GL::states::*;
		<$f>::Disable();
		GLDisable!($($n),+);
	}};
}

#[macro_export]
macro_rules! GLSave {
	($f: ty) => {{ use $crate::GL::states::*; <$f>::Save(); }};
	($f: ty, $($n: ty),+) => {{
		use $crate::GL::states::*;
		<$f>::Save();
		GLSave!($($n),+);
	}};
}

#[macro_export]
macro_rules! GLRestore {
	($f: ty) => {{ use $crate::GL::states::*; <$f>::Restore(); }};
	($f: ty, $($n: ty),+) => {{
		use $crate::GL::states::*;
		<$f>::Restore();
		GLRestore!($($n),+);
	}};
}
