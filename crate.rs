#![warn(clippy::all)]
#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc, clippy::module_inception, clippy::option_map_unit_fn)]

#[macro_use]
mod kit;
pub mod glsl;

#[cfg(feature = "gui")]
pub mod gui;

pub use kit::*;
