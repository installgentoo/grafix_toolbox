use super::{buffer::*, format::*, object::*, policy::*, types::*, universion::*, vao_args::*};
use crate::uses::*;

#[derive(Default)]
pub struct Vao<I> {
	o: Object<VertArrObj>,
	d: Dummy<I>,
}
impl<I: IdxType> Vao<I> {
	pub fn new() -> Self {
		Self { o: Object::new(), d: Dummy }
	}
	pub fn obj(&self) -> u32 {
		self.o.obj
	}
	pub fn Bind(&mut self) -> VaoBinding<I> {
		VaoBinding::new(self)
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
		let t_size = u32(type_size!(A));
		GLCheck!(glVertexAttribFormat(self.obj(), o.obj, idx, size, A::TYPE, to_glbool(norm), stride, offset, t_size));
	}
}

pub struct VaoBinding<'l, I> {
	_b: Binding<'l, VertArrObj>,
	d: Dummy<I>,
}
impl<I: IdxType> VaoBinding<'_, I> {
	pub fn new(o: &mut Vao<I>) -> Self {
		let _b = Binding::new(&mut o.o);
		Self { _b, d: Dummy }
	}
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
		GLCheck!(gl::DrawArrays(mode, i32(offset), num));
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
		GLCheck!(gl::DrawElementsInstanced(mode, num, I::TYPE, offset, i32(n)));
	}
}

pub fn idxcheck_map() -> &'static mut HashSet<u32> {
	UnsafeOnce!(HashSet<u32>, { HashSet::new() })
}

#[allow(unused_imports)]
use super::state::*;
