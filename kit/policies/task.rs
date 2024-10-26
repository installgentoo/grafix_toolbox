pub mod pre {
	pub use futures_lite::{future, stream, Future, FutureExt, Stream, StreamExt};
	pub use io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
	pub use tokio::{fs, io, sync::Notify};
}

pub fn Runtime() -> &'static RT {
	static S: OnceLock<RT> = OnceLock::new();
	S.get_or_init(|| {
		RT(rt::Builder::new_multi_thread()
			.worker_threads(1)
			.enable_time()
			.build()
			.expect("E| Cannot create async runtime"))
	})
}
impl RT {
	pub fn spawn<F: Future + SendStat>(&self, future: F) -> Task<F::Output>
	where
		F::Output: SendStat,
	{
		Task(Some(self.0.spawn(future)))
	}
	pub fn finish<T>(&self, mut t: Task<T>) -> T {
		self.finish_ref(&mut t)
	}
	pub fn finish_ref<T>(&self, t: &mut Task<T>) -> T {
		let t = t.0.take().valid();
		self.0.block_on(t).valid()
	}
}
pub struct RT(rt::Runtime);

pub struct Task<T>(Option<tokio::task::JoinHandle<T>>);
impl<T> Task<T> {
	pub fn new_uninit() -> Self {
		Self(None)
	}
	pub fn is_ready(&self) -> bool {
		self.0.as_ref().map_or(false, |s| s.is_finished())
	}
}
impl<T> Drop for Task<T> {
	fn drop(&mut self) {
		if let Some(h) = self.0.take() {
			h.abort()
		}
	}
}

use {crate::lib::*, pre::*, std::sync::OnceLock, tokio::runtime as rt};
