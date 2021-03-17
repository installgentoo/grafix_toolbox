mod args;
mod format;
mod framebuff;
#[macro_use]
mod sampler;
#[macro_use]
mod sampl_pool;
mod tex_type;
mod texbuff;
mod texture;

use super::*;

pub mod chans {
	pub use super::format::{RED, RG, RGB, RGBA};
}
pub mod spec {
	pub use super::format::{TexFmt, TexSize};
	pub use super::tex_type::*;
}
pub use {framebuff::*, sampl_pool::*, sampler::*, texbuff::*, texture::*};
