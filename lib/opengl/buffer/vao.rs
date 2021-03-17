use super::{bindless::*, buffer::*, format::*, object::*, policy::*, state::*, types::*, vao_args::*};
use crate::uses::*;

#[derive(Default)]
pub struct Vao<D: IdxType> {
	o: Object<VertArrObj>,
	d: Dummy<D>,
}
impl<D: IdxType> Vao<D> {
	pub fn new() -> Self {
		Self { o: Object::new(), d: Dummy }
	}
	pub fn obj(&self) -> u32 {
		self.o.obj
	}
	pub fn Bind(&mut self) -> VaoBinding<D> {
		VaoBinding::new(self)
	}
}

pub struct VaoBinding<'l, D> {
	b: Binding<'l, VertArrObj>,
	d: Dummy<D>,
}
impl<'l, D: IdxType> VaoBinding<'l, D> {
	pub fn new(o: &'l mut Vao<D>) -> Self {
		let b = Binding::new(&mut o.o);
		Self { b, d: Dummy }
	}
}
impl<I: IdxType> Vao<I> {
	pub fn BindIdxs(&mut self, o: &IdxArr<I>) {
		GLCheck!(glVaoElementBuffer(self.obj(), o.obj));
		idxcheck_map().insert(self.obj());
	}
	pub fn AttribFmt<A: AttrType>(&mut self, o: &AttrArr<A>, args: impl AttrFmtArgs) {
		let (idx, size, norm, stride, offset) = args.get();
		ASSERT!((size > 0 && size < 5), "Attribute size {} isn't valid", size);
		let t_size = u32::to(type_size!(A));
		GLCheck!(glVertexAttribFormat(self.obj(), o.obj, idx, size, A::TYPE, to_glbool(norm), stride, offset, t_size));
	}
}
impl<'l, I: IdxType> VaoBinding<'l, I> {
	pub fn Draw(&self, args: impl DrawArgs) {
		ASSERT!(
			idxcheck_map().get(VertArrObj::bound_obj()).is_some(),
			"No Index buffer bound to VertArrObj {}",
			VertArrObj::bound_obj()
		);

		let (num, offset, mode) = args.get();
		GLCheck!(gl::DrawElements(mode, num, I::TYPE, (offset * type_size!(I)) as *const GLvoid));
	}
	pub fn DrawUnindexed(&self, args: impl DrawArgs) {
		let (num, offset, mode) = args.get();
		GLCheck!(gl::DrawArrays(mode, i32::to(offset), num));
	}
	pub fn DrawInstanced<T>(&self, n: T, args: impl DrawArgs)
	where
		i32: Cast<T>,
	{
		ASSERT!(
			idxcheck_map().get(VertArrObj::bound_obj()).is_some(),
			"No Index buffer bound to VertArrObj {}",
			VertArrObj::bound_obj()
		);

		let (num, offset, mode) = args.get();
		let offset = (offset * type_size!(I)) as *const GLvoid;
		GLCheck!(gl::DrawElementsInstanced(mode, num, I::TYPE, offset, i32::to(n)));
	}
}
