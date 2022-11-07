use super::{consts::Get, funcs::*, types::*};
use crate::uses::logging;

FUNC!(gl, Viewport, i32, i32, i32, i32);
FUNC!(gl, BlendFunc, GLenum, GLenum);
FUNC!(gl, BlendFuncSeparate, GLenum, GLenum, GLenum, GLenum);
FUNC!(gl, BlendEquation, GLenum);
FUNC!(gl, DepthFunc, GLenum);
FUNC!(ext, PixelStorePack, i32);
FUNC!(ext, PixelStoreUnpack, i32);

mod ext {
	pub unsafe fn PixelStorePack(v: i32) {
		gl::PixelStorei(gl::PACK_ALIGNMENT, v)
	}
	pub unsafe fn PixelStoreUnpack(v: i32) {
		gl::PixelStorei(gl::UNPACK_ALIGNMENT, v)
	}
}

SWITCH!(DEPTH_WRITEMASK, gl::DepthMask(gl::TRUE), gl::DepthMask(gl::FALSE), DEFAULT_TRUE);

SWITCH!(DITHER, DEFAULT_TRUE);
SWITCH!(MULTISAMPLE, DEFAULT_TRUE);

SWITCH!(DEBUG_OUTPUT);
SWITCH!(DEBUG_OUTPUT_SYNCHRONOUS);

SWITCH!(DEPTH_TEST);
SWITCH!(BLEND);
SWITCH!(CULL_FACE);
SWITCH!(TEXTURE_CUBE_MAP_SEAMLESS);

#[macro_export]
macro_rules! GLEnable {
($f: ty) => {{ use crate::uses::GL::states::*; <$f>::Enable(); }};
($f: ty, $($n: ty),+) => {{
	use crate::uses::GL::states::*;
	<$f>::Enable();
	GLEnable!($($n),+);
}};
}

#[macro_export]
macro_rules! GLDisable {
($f: ty) => {{ use crate::uses::GL::states::*; <$f>::Disable(); }};
($f: ty, $($n: ty),+) => {{
	use crate::uses::GL::states::*;
	<$f>::Disable();
	GLDisable!($($n),+);
}};
}

#[macro_export]
macro_rules! GLSave {
($f: ty) => {{ use crate::uses::GL::states::*; <$f>::Save(); }};
($f: ty, $($n: ty),+) => {{
	use crate::uses::GL::states::*;
	<$f>::Save();
	GLSave!($($n),+);
}};
}

#[macro_export]
macro_rules! GLRestore {
($f: ty) => {{ use crate::uses::GL::states::*; <$f>::Restore(); }};
($f: ty, $($n: ty),+) => {{
	use crate::uses::GL::states::*;
	<$f>::Restore();
	GLRestore!($($n),+);
}};
}
