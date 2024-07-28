use super::{super::texture::*, *};

pub type Framebuffer = Object<Framebuff>;
pub type Renderbuffer = Object<Renderbuff>;

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

pub struct RenderTgt<S, F> {
	pub fbo: Fbo<S, F>,
	pub depth: Renderbuffer,
}
impl<S: TexSize, F: TexFmt> RenderTgt<S, F> {
	pub fn new(args: impl FboArgs) -> Self {
		let mut fbo = Fbo::new(args);
		let TexParam { w, h, .. } = fbo.tex.param;
		let depth = Renderbuffer::new();
		GL!(glRenderbuffStorage(depth.obj, 1, gl::DEPTH_COMPONENT, w, h));
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
		GL!(glClearFramebuff(self.fbo.fb.obj, gl::DEPTH, 0, &f32(d) as *const f32));
	}
	fn size(&self) -> uVec2 {
		self.fbo.size()
	}
	fn bind(&self) -> Binding<Framebuff> {
		self.fbo.bind()
	}
}
