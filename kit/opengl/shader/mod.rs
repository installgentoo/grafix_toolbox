pub use program::*;

pub mod program;
#[macro_use]
pub mod uniform;

#[macro_export]
macro_rules! SHADER {
	($n: ident, $($body: expr),+) => {
		#[allow(non_upper_case_globals)]
		pub const $n: $crate::GL::macro_uses::InlineShader = $crate::GL::macro_uses::InlineShader(stringify!($n), const_format::concatcp!($crate::GL::unigl::GLSL_VERSION, $($body,)+));
	};
}

pub struct InlineShader(pub STR, pub STR);

type CompileArgs = Box<[Str]>;
pub trait ShaderArgs {
	fn get(self) -> CompileArgs;
}
impl<A1: Into<Str>, A2: Into<Str>> ShaderArgs for (A1, A2) {
	fn get(self) -> CompileArgs {
		let (a1, a2) = self;
		[a1.into(), a2.into()].into()
	}
}
impl<A1: Into<Str>, A2: Into<Str>, A3: Into<Str>> ShaderArgs for (A1, A2, A3) {
	fn get(self) -> CompileArgs {
		let (a1, a2, a3) = self;
		[a1.into(), a2.into(), a3.into()].into()
	}
}
impl<const N: usize> ShaderArgs for [I; N] {
	fn get(self) -> CompileArgs {
		self.into_iter().map(|a| a.into()).collect_box()
	}
}

type I = InlineShader;

mod compiler;
mod object;
mod parsing;

use crate::{lazy::*, lib::*, slicing::*, sync::*, GL};
use {super::internal::*, std::ffi::CString};
