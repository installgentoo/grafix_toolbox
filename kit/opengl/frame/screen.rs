use super::{Frame, GL::window::*, *};
use crate::math::*;

impl Frame for Window {
	fn ClearColor(&self, args: impl ClearArgs) {
		let (attach, c) = args.get();
		GLCheck!(glClearFramebuff(0, gl::COLOR, attach, c.as_ptr()));
	}
	fn ClearDepth<T>(&self, d: T)
	where
		f32: Cast<T>,
	{
		GLCheck!(glClearFramebuff(0, gl::DEPTH, 0, &f32(d) as *const f32));
	}
	fn size(&self) -> uVec2 {
		self.info().size
	}
	fn aspect(&self) -> Vec2 {
		self.info().aspect
	}
	fn pixel(&self) -> f32 {
		self.info().pixel
	}
	fn bind(&self) -> Binding<Framebuff> {
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

pub struct FrameInfo {
	pub size: uVec2,
	pub aspect: Vec2,
	pub pixel: f32,
}
impl FrameInfo {
	pub fn new((w, h): uVec2) -> Self {
		let size = (w, h);
		let (w, h, min) = Vec3((w, h, w.min(h)));
		let aspect = (w, h).div(min);
		let pixel = 2. / min;
		Self { size, aspect, pixel }
	}
}
