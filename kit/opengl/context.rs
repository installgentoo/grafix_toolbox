pub use offhand::*;

pub mod event;
pub mod window;

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
		while gl::TIMEOUT_EXPIRED == GL!(gl::ClientWaitSync(self.obj, 0, 16000000)) {}
	}
	pub fn BlockFor(&self, nanoseconds: u64) {
		GL!(gl::ClientWaitSync(self.obj, 0, nanoseconds));
		DEBUG!("Synced GL Fence {:?}", self.obj);
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

use crate::lib::*;
