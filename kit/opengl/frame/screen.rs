use super::args::*;
use crate::uses::{adapters::*, *};
use GL::{spec::*, unigl::*, window::*, Frame};

impl Frame for Window {
	fn ClearColor(&self, args: impl ClearArgs) {
		let (attach, c) = args.get();
		GLCheck!(glClearFramebuff(0, gl::COLOR, attach, c.as_ptr() as *const f32));
	}
	fn ClearDepth<T>(&self, d: T)
	where
		f32: Cast<T>,
	{
		GLCheck!(glClearFramebuff(0, gl::DEPTH, 0, &f32(d) as *const f32));
	}
	fn size(&self) -> uVec2 {
		Window::_size()
	}
	fn aspect(&self) -> Vec2 {
		Window::_aspect()
	}
	fn pixel(&self) -> Vec2 {
		Window::_pixel()
	}
	fn bind(&mut self) -> Binding<Framebuff> {
		let (w, h) = self.size();
		self.Viewport((0, 0, w, h));
		self.Bind()
	}
}
impl Window {
	pub fn Viewport(&self, args: impl WINSize) {
		let (x, y, w, h) = args.get();
		GL::Viewport::Set((x, y, i32(w), i32(h)));
	}
	pub fn Bind(&self) -> Binding<Framebuff> {
		Binding::<Framebuff>::zero()
	}
}
