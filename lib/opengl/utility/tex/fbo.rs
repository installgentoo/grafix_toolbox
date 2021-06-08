use crate::uses::*;
use crate::GL::{spec::*, tex::*, Framebuffer};

pub struct Fbo<S, F> {
	pub fb: Framebuffer,
	pub tex: Tex2d<S, F>,
}
impl<S: TexSize, F: TexFmt> Fbo<S, F> {
	pub fn new(args: impl FboArgs) -> Self {
		let (w, h) = args.get();
		let tex = Tex2d::<S, F>::new_empty((w, h));
		let fb = Framebuffer::new().attach((&tex, gl::COLOR_ATTACHMENT0));
		Self { fb, tex }
	}
	pub fn bind(&mut self) -> Binding<Framebuff> {
		self.fb.Bind(&self.tex)
	}
	pub fn clear(&self) {
		self.fb.ClearColor(0);
	}
}

pub struct Slab<S, F> {
	pub src: Fbo<S, F>,
	pub tgt: Fbo<S, F>,
}
impl<S: TexSize, F: TexFmt> Slab<S, F> {
	pub fn new(args: impl FboArgs) -> Self {
		Self {
			src: Fbo::new(args),
			tgt: Fbo::new(args),
		}
	}
	pub fn swap(&mut self) {
		mem::swap(&mut self.src, &mut self.tgt);
	}
}

pub trait FboArgs: Copy {
	fn get(self) -> iVec2;
}
impl<W: Copy, H: Copy> FboArgs for (W, H)
where
	i32: Cast<W> + Cast<H>,
{
	fn get(self) -> iVec2 {
		iVec2::to(self)
	}
}
