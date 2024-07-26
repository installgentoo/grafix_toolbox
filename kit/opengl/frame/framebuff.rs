use super::{super::texture::*, *};

pub type Framebuffer = Object<Framebuff>;
pub type Renderbuffer = Object<Renderbuff>;

impl Framebuffer {
	pub fn attach(self, args: impl FramebuffArg) -> Self {
		args.apply(self.obj);
		self
	}
	pub fn Bind<S, F>(&self, tex: &Tex<S, F, impl TexType>) -> Binding<Framebuff> {
		let TexParam { w, h, .. } = tex.param;
		GL::Viewport::Set((0, 0, w, h));
		Binding::new(self)
	}
	pub fn Clear(&self, typ: GLenum, args: impl ClearArgs) {
		let (attach, c) = args.get();
		GL!(glClearFramebuff(self.obj, typ, attach, c.as_ptr()));
	}
}

pub trait FramebuffArg {
	fn apply(self, _: u32);
}
impl<S, F, T: TexType> FramebuffArg for (&Tex<S, F, T>, GLenum) {
	fn apply(self, obj: u32) {
		GL!(glFramebuffTex(obj, self.0.obj(), self.1));
	}
}
impl FramebuffArg for (&Renderbuffer, GLenum) {
	fn apply(self, obj: u32) {
		GL!(glFramebuffRenderbuff(obj, self.0.obj, self.1));
	}
}
