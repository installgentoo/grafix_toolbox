pub mod pre {
	pub use super::StreamExtKit;
	pub use io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
	pub use tokio::{fs, io, sync::Notify};
	pub use tokio_stream::{Stream, StreamExt};
	pub mod stream {
		pub use futures_lite::stream::{Boxed, once_future as once, unfold};
		pub use tokio_stream::iter;
	}
	pub use futures_lite::future::FutureExt;
	pub type BoxFut<'s, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + 's>>;
}

#[allow(async_fn_in_trait)]
pub trait StreamExtKit: Stream {
	async fn count(self) -> usize;
	async fn for_each(self, f: impl FnMut(Self::Item));
}
impl<S: StreamExt> StreamExtKit for S {
	async fn count(self) -> usize {
		self.fold(0, |s, _| s + 1).await
	}
	async fn for_each(self, mut f: impl FnMut(Self::Item)) {
		self.fold((), |_, i| f(i)).await
	}
}

pub fn GLRuntime() -> &'static RT {
	InitGLRuntime::<WindowImpl>(None)
}
pub(in crate::kit) fn InitGLRuntime<W: Window>(w: Option<&mut W>) -> &'static RT {
	static S: OnceLock<RT> = OnceLock::new();
	S.get_or_init(|| {
		let w = w.unwrap_or_else(|| ERROR!("GLRuntime() accessed before Window()"));
		let init = w.gl_ctx_maker();
		rt::Builder::new_multi_thread()
			.worker_threads(1)
			.max_blocking_threads(1)
			.enable_time()
			.build()
			.explain_err(|| "Cannot create gl runtime")
			.fail()
			.tap(|s| {
				s.spawn(async { Box(init()).pipe(Box::leak) });
			})
			.pipe(RT)
	})
}

pub fn Runtime() -> &'static RT {
	static S: OnceLock<RT> = OnceLock::new();
	S.get_or_init(|| {
		rt::Builder::new_multi_thread()
			.worker_threads(3)
			.max_blocking_threads(2)
			.enable_time()
			.build()
			.expect("E| Cannot create async runtime")
			.pipe(RT)
	})
}
impl RT {
	pub fn spawn<T: SendS, F: Fut<T>>(&self, f: impl FnOnce() -> F + SendS) -> Task<T> {
		self.0.spawn(async { f().await }).pipe(Some).pipe(Task)
	}
	pub fn finish<T: SendS>(&self, mut t: Task<T>) -> T {
		self.finish_ref(&mut t)
	}
	pub fn cancel<T: SendS>(&self, mut t: Task<T>) {
		t.0.take().map(|h| {
			h.abort();
			let _ = self.block_on(h);
		});
	}
	pub fn finish_ref<T: SendS>(&self, t: &mut Task<T>) -> T {
		let t = t.0.take().valid();
		self.block_on(t).valid()
	}
	fn block_on<T: SendS>(&self, h: JoinHandle<T>) -> Res<T> {
		if let Ok(r) = rt::Handle::try_current() {
			WARN!("Blocking code polluted async");
			let (sn, rx) = oneshot::channel();
			r.spawn(async { sn.send(h.await) });
			task::block_in_place(|| rx.blocking_recv().valid()).res()
		} else {
			self.0.block_on(h).res()
		}
	}
}
pub struct RT(rt::Runtime);

pub struct Task<T>(Option<JoinHandle<T>>);
impl<T> Task<T> {
	pub fn new_uninit() -> Self {
		Self(None)
	}
	pub fn detach(mut self) {
		let _ = self.0.take();
	}
	pub fn is_ready(&self) -> bool {
		self.0.as_ref().is_some_and(|s| s.is_finished())
	}
}
impl<T> Drop for Task<T> {
	fn drop(&mut self) {
		self.0.take().map(|h| h.abort());
	}
}
impl<T> Debug for Task<T> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		let h = &self.0.as_ref().map(|h| h.id());
		f.debug_tuple("Task").field(h).field(&type_name::<T>()).finish()
	}
}

use tokio::{runtime as rt, sync::oneshot, task::JoinHandle};
use {crate::lib::*, GL::window::*};
