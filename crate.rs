#![warn(clippy::all)]
#![allow(
	non_snake_case,
	non_camel_case_types,
	mismatched_lifetime_syntaxes,
	clippy::unit_arg,
	clippy::let_and_return,
	clippy::option_map_unit_fn,
	clippy::mut_from_ref,
	clippy::missing_transmute_annotations,
	clippy::missing_safety_doc
)]

pub use kit::*;

#[macro_use]
mod kit;

#[cfg(feature = "gui")]
pub mod gui;
