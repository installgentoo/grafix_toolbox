pub use program::*;

pub mod program;
#[macro_use]
pub mod uniform;

#[macro_export]
macro_rules! SHADER {
	($n: ident, $($body: expr),+) => {
		#[allow(non_upper_case_globals)]
		pub const $n: $crate::GL::macro_uses::InlineShader = $crate::GL::macro_uses::InlineShader(stringify!($n), const_format::concatcp!($($body,)+));
	};
}

pub struct InlineShader(pub STR, pub STR);

type I = InlineShader;

mod args;
mod compiler;
mod object;
mod parsing;

use crate::{lazy::*, lib::*, slicing::*, sync::*, FS, GL, GL::window::*};
use {super::internal::*, std::ffi::CString};
