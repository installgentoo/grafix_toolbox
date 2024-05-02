#![warn(clippy::all)]
#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc, clippy::module_inception)]

pub use kit::*;

#[macro_use]
mod kit;
pub mod glsl;

#[cfg(feature = "gui")]
pub mod gui;
