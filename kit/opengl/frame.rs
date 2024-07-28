pub use {fbo::*, framebuff::*, screen::FrameInfo};

pub trait Frame {
	fn aspect(&self) -> Vec2 {
		let (w, h) = self.size();
		let (w, h, min) = Vec3((w, h, w.min(h)));
		(min, min).div((w, h))
	}
	fn to_clip(&self) -> Vec2 {
		(1., 1.).div(self.aspect())
	}
	fn pixel(&self) -> f32 {
		let (w, h) = self.size();
		2. / f32(w.min(h))
	}
	fn pixel_vec2(&self) -> Vec2 {
		let p = self.pixel();
		(p, p)
	}
	fn clear(&self, args: impl ColorDepthArg) {
		let (rgba, d) = args.getc();
		self.ClearColor((0, rgba));
		self.ClearDepth(d);
	}
	fn ClearColor(&self, _: impl ClearArgs);
	fn ClearDepth<T>(&self, _: T)
	where
		f32: Cast<T>,
	{
	}
	fn size(&self) -> uVec2;
	fn bind(&self) -> Binding<Framebuff>;
}

mod args;
mod fbo;
mod framebuff;
mod screen;

use crate::{lib::*, math::*, GL::tex::*};
use {super::internal::*, args::*};
