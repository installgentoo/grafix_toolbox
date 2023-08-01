#![warn(clippy::all)]
#![allow(
	non_snake_case,
	non_camel_case_types,
	clippy::len_without_is_empty,
	clippy::missing_safety_doc,
	clippy::module_inception,
	clippy::type_complexity
)]

#[macro_use]
mod kit;
pub mod glsl;

#[cfg(feature = "gui")]
pub mod gui;

pub use kit::*;
