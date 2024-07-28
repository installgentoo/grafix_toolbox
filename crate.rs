#![warn(clippy::all)]
#![allow(non_snake_case, non_camel_case_types, clippy::missing_safety_doc)]

pub use kit::*;

#[macro_use]
mod kit;

#[cfg(feature = "gui")]
pub mod gui;
