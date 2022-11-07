#![warn(clippy::all)]
#![allow(
	non_snake_case,
	non_camel_case_types,
	clippy::float_cmp,
	clippy::from_over_into,
	clippy::many_single_char_names,
	clippy::missing_safety_doc,
	clippy::module_inception,
	clippy::option_map_unit_fn,
	clippy::type_complexity,
	clippy::unit_arg,
	clippy::upper_case_acronyms
)]

#[macro_use]
mod kit;
pub mod glsl;

#[cfg(feature = "gui")]
pub mod gui;

pub use kit::*;
