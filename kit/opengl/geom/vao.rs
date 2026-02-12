use super::{super::internal::*, *};

#[derive(Default, Debug)]
pub struct Vao<I> {
	i: Dummy<I>,
	o: Obj<VertArrT>,
}
impl<I: IdxType> Vao<I> {
	pub fn obj(&self) -> u32 {
		self.o.obj
	}
	pub fn Bind(&self) -> VaoBind<I> {
		VaoBind::new(self)
	}
	pub fn BindIdxs(&mut self, o: &IdxArr<I>) {
		GL!(glVaoElementBuffer(self.obj(), o.obj));
		debug_assert!({
			*Index::crossbindcheck_map().entry(self.obj()).or_default() = vec![o.obj];
			true
		});
	}
	pub fn AttribFmt<A: AttrType>(&mut self, o: &AttrArr<A>, args: impl AttrFmtArgs) {
		let (idx, size, norm, stride, offset) = args.get();
		ASSERT!(size > 0 && size < 5, "Attribute size({size}) isn't valid");
		let t_size = u32(type_size::<A>());
		GL!(glVertexAttribFormat(self.obj(), o.obj, idx, size, A::TYPE, to_glbool(norm), stride, offset, t_size));
		debug_assert!({
			let attrs = Attribute::crossbindcheck_map().entry(self.obj()).or_default();
			if attrs.len() < usize(idx + 1) {
				attrs.resize(usize(idx + 1), u32::MAX);
			}
			attrs[usize(idx)] = o.obj;
			true
		});
	}
}
impl<I> Drop for Vao<I> {
	fn drop(&mut self) {
		debug_assert!({
			Index::crossbindcheck_map().remove(&self.o.obj);
			Attribute::crossbindcheck_map().remove(&self.o.obj);
			true
		})
	}
}

pub struct VaoBind<'l, I> {
	i: Dummy<I>,
	_b: Bind<'l, VertArrT>,
}
impl<I: IdxType> VaoBind<'_, I> {
	fn new(o: &Vao<I>) -> Self {
		let _b = Bind::new(&o.o);
		Self { i: Dummy, _b }
	}
	pub fn Draw(&self, args: impl DrawArgs) {
		VertArrT::bound_obj().pipe_as(Index::checkcrossbinds);
		VertArrT::bound_obj().pipe_as(Attribute::checkcrossbinds);
		let (num, offset, mode) = args.get();
		GL!(gl::DrawElements(mode, num, I::TYPE, (offset * type_size::<I>()) as *const GLvoid));
	}
	pub fn DrawUnindexed(&self, args: impl DrawArgs) {
		VertArrT::bound_obj().pipe_as(Attribute::checkcrossbinds);
		let (num, offset, mode) = args.get();
		GL!(gl::DrawArrays(mode, i32(offset), num));
	}
	pub fn DrawInstanced<A>(&self, n: A, args: impl DrawArgs)
	where
		i32: Cast<A>,
	{
		VertArrT::bound_obj().pipe_as(Index::checkcrossbinds);
		VertArrT::bound_obj().pipe_as(Attribute::checkcrossbinds);
		let (num, offset, mode) = args.get();
		let offset = (offset * type_size::<I>()) as *const GLvoid;
		GL!(gl::DrawElementsInstanced(mode, num, I::TYPE, offset, i32(n)));
	}
}
