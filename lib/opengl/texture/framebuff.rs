use super::{bindless::*, format::*, object::*, policy::*, texture::*, types::*};
use crate::uses::*;

pub type Framebuffer = Object<Framebuff>;
pub type Renderbuffer = Object<Renderbuff>;

impl Framebuffer {
	pub fn attach(self, args: impl FramebuffArg) -> Self {
		args.apply(self.obj);
		self
	}
	pub fn Bind<T: TexType, S: TexSize, F: TexFmt>(&mut self, tex: &Tex<T, S, F>) -> Binding<Framebuff> {
		let TexParam { w, h, .. } = tex.param;
		GL::Viewport::Set((0, 0, w, h));
		Binding::new(self)
	}
	pub fn ClearColor(&self, args: impl ClearArgs) {
		let (attach, c) = args.get();
		GLCheck!(glClearFramebuff(self.obj, gl::COLOR, attach, c.as_ptr() as *const f32));
	}
	pub fn ClearDepth<T>(&self, d: T)
	where
		f32: Cast<T>,
	{
		GLCheck!(glClearFramebuff(self.obj, gl::DEPTH, 0, &f32::to(d) as *const f32));
	}
}

pub trait FramebuffArg {
	fn apply(self, _: u32);
}
impl<T: TexType, S: TexSize, F: TexFmt> FramebuffArg for (&Tex<T, S, F>, GLenum) {
	fn apply(self, obj: u32) {
		GLCheck!(glFramebuffTex(obj, self.0.obj(), self.1));
	}
}
impl FramebuffArg for (&Renderbuffer, GLenum) {
	fn apply(self, obj: u32) {
		GLCheck!(glFramebuffRenderbuff(obj, self.0.obj, self.1));
	}
}

type Args = (i32, [f32; 4]);
pub trait ClearArgs {
	fn get(self) -> Args;
}
impl<R, G, B, A> ClearArgs for (u32, (R, G, B, A))
where
	Vec4: Cast<(R, G, B, A)>,
{
	fn get(self) -> Args {
		let (r, g, b, a) = Vec4::to(self.1);
		(i32::to(self.0), [r, g, b, a])
	}
}
impl<R, G, B, A> ClearArgs for (R, G, B, A)
where
	Vec4: Cast<(R, G, B, A)>,
{
	fn get(self) -> Args {
		(0, self).get()
	}
}
impl<C: Copy> ClearArgs for (u32, C)
where
	f32: Cast<C>,
{
	fn get(self) -> Args {
		let v = self.1;
		(self.0, (v, v, v, v)).get()
	}
}
impl<C: Copy> ClearArgs for C
where
	f32: Cast<C>,
{
	fn get(self) -> Args {
		(0, self).get()
	}
}
