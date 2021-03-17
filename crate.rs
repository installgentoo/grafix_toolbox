#![warn(clippy::all)]
#![allow(
	dead_code,
	non_snake_case,
	non_camel_case_types,
/*	unused_macros,
	clippy::range_plus_one,
	clippy::many_single_char_names,
	clippy::too_many_arguments,
	clippy::cast_lossless,
	clippy::new_without_default*/
)]

#[macro_use]
mod lib;
pub use lib::*;

pub mod glsl;

pub mod gui;
