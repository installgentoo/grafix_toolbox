pub use {fbo::*, frame::*, framebuff::*, screen::FrameInfo};

mod args;
mod fbo;
mod frame;
mod framebuff;
mod screen;

use {super::internal::*, crate::lib::*, crate::GL, args::*};
