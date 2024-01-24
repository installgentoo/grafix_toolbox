pub use {fbo::*, frame::*, framebuff::*};

mod args;
mod fbo;
mod frame;
mod framebuff;
mod screen;

use {super::internal::*, crate::lib::*, crate::GL, args::*};
