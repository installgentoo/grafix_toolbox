#[macro_use]
pub mod shader;
#[macro_use]
pub mod uniforms;

mod args;
mod parsing;

use super::*;

pub use shader::*;
pub mod shader_use {
	pub use super::{inline_shd_use::*, uniforms::uniforms_use::*};
}
