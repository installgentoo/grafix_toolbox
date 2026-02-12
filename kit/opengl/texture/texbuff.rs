use super::{format::*, *};

pub struct TexBuffer<'a, S, F, B: State> {
	t: Dummy<&'a (S, F, B)>,
	tex: Obj<TextureT<GL_TEXTURE_BUFFER>>,
	unit: Cell<u32>,
}
impl<S: TexSize, F: TexFmt, B: Buffer> TexBuffer<'_, S, F, B> {
	pub fn new(buf: &ArrObj<B, F>) -> Self {
		let tex = Obj::new();
		let fmt = get_internal_fmt::<S, F>();
		ASSERT!(
			GL::MAX_TEXTURE_BUFFER_SIZE() >= i32(buf.len),
			"Buffer {} for buffer texture {} exceeds maximum size",
			buf.obj,
			tex.obj
		);
		GL!(glTextureBuffer(GL_TEXTURE_BUFFER::TYPE, tex.obj, fmt, buf.obj));
		Self { t: Dummy, tex, unit: 0.into() }
	}
	pub fn Bind(&self) -> TexBuffBind {
		let unit = self.unit.take();
		let (b, u) = TexBuffBind::new(&self.tex, unit);
		self.unit.set(u);
		b
	}
}

pub struct TexBuffBind<'l> {
	l: Dummy<&'l GL_TEXTURE_BUFFER>,
	pub u: u32,
}
impl TexBuffBind<'_> {
	fn new(o: &Obj<TextureT<GL_TEXTURE_BUFFER>>, hint: u32) -> (Self, u32) {
		let u = TexState::BindAny::<GL_TEXTURE_BUFFER>(o.obj, hint);
		(Self { l: Dummy, u }, u)
	}
}
impl Drop for TexBuffBind<'_> {
	fn drop(&mut self) {
		TexState::Unbind(self.u);
	}
}
