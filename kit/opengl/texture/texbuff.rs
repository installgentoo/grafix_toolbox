use super::{format::*, spec::*, *};

pub struct TexBuffer<'a, S, F, B: State> {
	tex: Object<Texture<GL_TEXTURE_BUFFER>>,
	unit: Cell<u32>,
	b: Dummy<&'a ArrObject<B, F>>,
	f: Dummy<F>,
	s: Dummy<S>,
}
impl<S: TexSize, F: TexFmt, B: Buffer> TexBuffer<'_, S, F, B> {
	pub fn new(buf: &ArrObject<B, F>) -> Self {
		let tex = Object::new();
		let fmt = get_internal_fmt::<S, F>();
		ASSERT!(
			GL::MAX_TEXTURE_BUFFER_SIZE() >= i32(buf.len),
			"Buffer {} for buffer texture {} exceeds maximum size",
			buf.obj,
			tex.obj
		);
		GL!(glTextureBuffer(GL_TEXTURE_BUFFER::TYPE, tex.obj, fmt, buf.obj));
		let (b, f, s) = (Dummy, Dummy, Dummy);
		let unit = Cell::new(0);
		Self { tex, unit, b, f, s }
	}
	pub fn Bind(&self) -> TexBuffBinding {
		let unit = self.unit.take();
		let (b, u) = TexBuffBinding::new(&self.tex, unit);
		self.unit.set(u);
		b
	}
}

pub struct TexBuffBinding<'l> {
	t: Dummy<&'l GL_TEXTURE_BUFFER>,
	pub u: u32,
}
impl TexBuffBinding<'_> {
	pub fn new(o: &Object<Texture<GL_TEXTURE_BUFFER>>, hint: u32) -> (Self, u32) {
		let u = TexState::BindAny::<GL_TEXTURE_BUFFER>(o.obj, hint);
		(Self { t: Dummy, u }, u)
	}
}
impl Drop for TexBuffBinding<'_> {
	fn drop(&mut self) {
		TexState::Unbind(self.u);
	}
}
