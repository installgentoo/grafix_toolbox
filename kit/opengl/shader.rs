pub use shader_ext::*;

#[macro_use]
pub mod uniform;

#[macro_export]
macro_rules! SHADER {
	($n: ident, $($body: expr),+) => {
		#[allow(non_upper_case_globals)]
		pub const $n: $crate::GL::macro_uses::InlineShader = $crate::GL::macro_uses::InlineShader(stringify!($n), &[$($body,)+]);
	};
}
pub struct InlineShader(pub STR, pub &'static [STR]);
impl From<I> for Str {
	fn from(v: I) -> Self {
		let InlineShader(v, v_t) = v;
		ShaderManager::inline_source(v, v_t);
		v.into()
	}
}
type I = InlineShader;

mod args;
mod compiler;
mod object;
mod parsing;
mod shader_ext;

use crate::{lazy::*, lib::*, slicing::*, sync::*, GL::window::*};
use {super::internal::*, std::ffi::CString};
