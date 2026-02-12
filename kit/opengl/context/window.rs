use super::*;

use win_impl::CtxDrop;
pub use win_impl::WindowImpl;

pub fn Window(a: impl WINSize, t: &str) -> impl Window {
	WindowImpl::get(a, t).fail()
}

pub trait Window: 'static {
	fn info(&self) -> &FrameInfo;
	fn clipboard(&self) -> String;
	fn set_clipboard(&mut self, str: &str);
	fn set_vsync(&mut self, enabled: bool);
	fn resize(&mut self, size: uVec2);

	fn gl_ctx_maker(&mut self) -> impl SendS + FnOnce() -> CtxDrop;
	fn poll_events(&mut self) -> Vec<event::Event>;
	fn swap(&mut self);
}

type WArgs = (i32, i32, u32, u32);
pub trait WINSize {
	fn get(self) -> WArgs;
}
impl<A, B, C, D> WINSize for (A, B, C, D)
where
	i32: Cast<A> + Cast<B>,
	u32: Cast<C> + Cast<D>,
{
	fn get(self) -> WArgs {
		WArgs::to(self)
	}
}
impl<A, B> WINSize for (A, B)
where
	u32: Cast<A> + Cast<B>,
{
	fn get(self) -> WArgs {
		(0, 0, u32(self.0), u32(self.1))
	}
}
