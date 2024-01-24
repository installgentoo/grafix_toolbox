use super::{Renderbuffer, *};
use crate::{math::*, GL::tex::*};

pub trait Frame {
	fn aspect(&self) -> Vec2 {
		let (w, h) = self.size();
		let (w, h, min) = Vec3((w, h, w.min(h)));
		(min, min).div((w, h))
	}
	fn pixel(&self) -> Vec2 {
		(1., 1.).div(self.size())
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
	fn bind(&mut self) -> Binding<Framebuff>;
}

pub struct RenderTgt<S, F> {
	pub fbo: Fbo<S, F>,
	pub depth: Renderbuffer,
}
impl<S: TexSize, F: TexFmt> RenderTgt<S, F> {
	pub fn new(args: impl FboArgs) -> Self {
		let mut fbo = Fbo::new(args);
		let TexParam { w, h, .. } = fbo.tex.param;
		let depth = Renderbuffer::new();
		GLCheck!(glRenderbuffStorage(depth.obj, 1, gl::DEPTH_COMPONENT, w, h));
		fbo.fb = fbo.fb.attach((&depth, gl::DEPTH_ATTACHMENT));
		Self { fbo, depth }
	}
}
impl<S: TexSize, F: TexFmt> Frame for RenderTgt<S, F> {
	fn ClearColor(&self, args: impl ClearArgs) {
		self.fbo.ClearColor(args);
	}
	fn ClearDepth<T>(&self, d: T)
	where
		f32: Cast<T>,
	{
		GLCheck!(glClearFramebuff(self.fbo.fb.obj, gl::DEPTH, 0, &f32(d) as *const f32));
	}
	fn size(&self) -> uVec2 {
		self.fbo.size()
	}
	fn bind(&mut self) -> Binding<Framebuff> {
		self.fbo.bind()
	}
}
