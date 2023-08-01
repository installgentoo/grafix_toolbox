use crate::uses::*;

#[derive(Debug)]
pub struct Fence {
	obj: gl::types::GLsync,
}
impl Fence {
	pub fn new() -> Self {
		let obj = GLCheck!(gl::FenceSync(gl::SYNC_GPU_COMMANDS_COMPLETE, 0));
		DEBUG!("Created GL Fence {obj:?}");
		GLCheck!(gl::Flush());
		Self { obj }
	}
	pub fn Block(&self) {
		loop {
			let state = GLCheck!(gl::ClientWaitSync(self.obj, 0, 16000000));
			if state != gl::TIMEOUT_EXPIRED {
				return;
			}
		}
	}
	pub fn BlockFor(&self, nanoseconds: u64) {
		GLCheck!(gl::ClientWaitSync(self.obj, 0, nanoseconds));
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
		GLCheck!(gl::DeleteSync(self.obj));
	}
}

unsafe impl Send for Fence {}
