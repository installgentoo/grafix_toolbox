pub use offhand::*;

pub mod event;
pub mod window;

fn load_gl(loader: impl FnMut(STR) -> *const std::ffi::c_void) {
	gl::load_with(loader);

	let version = unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) }.to_str().fail();
	PRINT!("Initialized OpenGL, {version}");
	*GL::macro_uses::gl_was_initialized() = true;
	if GL::unigl::IS_DEBUG {
		GL::EnableDebugContext(GL::DebugLevel::All);
	}
	crate::GL!(gl::Disable(gl::DITHER));
}

#[derive(Debug)]
pub struct Fence {
	obj: gl::types::GLsync,
}
impl Fence {
	pub fn new() -> Self {
		let obj = GL!(gl::FenceSync(gl::SYNC_GPU_COMMANDS_COMPLETE, 0));
		DEBUG!("Created GL Fence {obj:?}");
		GL!(gl::Flush());
		Self { obj }
	}
	pub fn Block(&self) {
		while !self.BlockFor(16000000) {}
	}
	pub fn BlockFor(&self, nanoseconds: u64) -> bool {
		(gl::TIMEOUT_EXPIRED != GL!(gl::ClientWaitSync(self.obj, 0, nanoseconds))).tap(|_| DEBUG!("Synced GL Fence {:?}", self.obj))
	}
}
impl Default for Fence {
	fn default() -> Self {
		Self::new()
	}
}
impl Drop for Fence {
	fn drop(&mut self) {
		DEBUG!("Deleting GL Fence {:?}", self.obj);
		GL!(gl::DeleteSync(self.obj));
	}
}
unsafe impl Send for Fence {}

mod offhand;

//mod glfw_impl;
mod sdl2_impl;
use sdl2_impl as win_impl;

use crate::{GL::FrameInfo, lib::*, math::*};
