use super::{policy, state::*, universion::*};
use crate::uses::{adapters::*, *};

pub fn Viewport(args: impl WINSize) {
	let (x, y, w, h) = args.get();
	GL::Viewport::Set((x, y, i32::to(w), i32::to(h)));
}

pub fn BindScreenFbo() {
	policy::Framebuff::Lock(0);
	policy::Framebuff::Bind(0);
}

pub fn ClearScreen(args: impl ColorDepthArg) {
	let (rgba, d) = args.get();
	ClearColor(rgba);
	ClearDepth(d);
}

pub fn ClearColor(rgba: [f32; 4]) {
	GLCheck!(glClearFramebuff(0, gl::COLOR, 0, rgba.as_ptr() as *const f32));
}

pub fn ClearDepth(d: f32) {
	GLCheck!(glClearFramebuff(0, gl::DEPTH, 0, &d as *const f32));
}

type Args = ([f32; 4], f32);
pub trait ColorDepthArg {
	fn get(self) -> Args;
}
impl<R, G, B, A, D> ColorDepthArg for ((R, G, B, A), D)
where
	f32: Cast<D>,
	Vec4: Cast<(R, G, B, A)>,
{
	fn get(self) -> Args {
		let (r, g, b, a) = Vec4::to(self.0);
		([r, g, b, a], f32::to(self.1))
	}
}
impl<R, G, B, A> ColorDepthArg for (R, G, B, A)
where
	Vec4: Cast<(R, G, B, A)>,
{
	fn get(self) -> Args {
		(self, 1.).get()
	}
}
impl<C: Copy, D> ColorDepthArg for (C, D)
where
	f32: Cast<C> + Cast<D>,
{
	fn get(self) -> Args {
		let v = self.0;
		((v, v, v, v), self.1).get()
	}
}
impl<C: Copy> ColorDepthArg for C
where
	f32: Cast<C>,
{
	fn get(self) -> Args {
		(self, 1.).get()
	}
}
