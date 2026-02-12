use super::{GL::window::*, *};
use crate::math::*;

impl<W: Window> Frame for W {
	fn ClearColor(&self, args: impl ClearArgs) {
		let (attach, c) = args.get();
		GL!(glClearFramebuff(0, gl::COLOR, attach, c.as_ptr()));
	}
	fn ClearDepth<A>(&self, d: A)
	where
		f32: Cast<A>,
	{
		GL!(glClearFramebuff(0, gl::DEPTH, 0, &f32(d) as *const f32));
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
	fn bind(&self) -> Bind<FramebuffT> {
		let (w, h) = vec2(self.size());
		GL::Viewport::Set((0, 0, w, h));
		Bind::<FramebuffT>::zero()
	}
}

pub struct FrameInfo {
	pub size: uVec2,
	pub aspect: Vec2,
	pub pixel: f32,
}
impl FrameInfo {
	pub fn new(size: uVec2) -> Self {
		let s = Vec2(size);
		let aspect = s.div(s.min_comp());
		let pixel = 2. / s.min_comp();
		Self { size, aspect, pixel }
	}
}
