use super::{buffer::*, format::*, object::*, policy::*, state::*, types::*, universion::*, vao_args::*};
use crate::uses::*;

#[derive(Default)]
pub struct Vao<I> {
	o: Object<VertArrObj>,
	d: Dummy<I>,
}
impl<I: IdxType> Vao<I> {
	pub fn new() -> Self {
		Self { o: Def(), d: Dummy }
	}
	pub fn obj(&self) -> u32 {
		self.o.obj
	}
	pub fn Bind(&mut self) -> VaoBinding<I> {
		VaoBinding::new(self)
	}
	pub fn BindIdxs(&mut self, o: &IdxArr<I>) {
		GLCheck!(glVaoElementBuffer(self.obj(), o.obj));
		debug_assert!({
			*Index::crossbindcheck_map().entry(self.obj()).or_default() = vec![o.obj];
			true
		});
	}
	pub fn AttribFmt<A: AttrType>(&mut self, o: &AttrArr<A>, args: impl AttrFmtArgs) {
		let (idx, size, norm, stride, offset) = args.get();
		ASSERT!(size > 0 && size < 5, "Attribute size({size}) isn't valid");
		let t_size = u32(type_size!(A));
		GLCheck!(glVertexAttribFormat(self.obj(), o.obj, idx, size, A::TYPE, to_glbool(norm), stride, offset, t_size));
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
		Index::checkcrossbinds(VertArrObj::bound_obj());
		Attribute::checkcrossbinds(VertArrObj::bound_obj());
		let (num, offset, mode) = args.get();
		GLCheck!(gl::DrawElements(mode, num, I::TYPE, (offset * type_size!(I)) as *const GLvoid));
	}
	pub fn DrawUnindexed(&self, args: impl DrawArgs) {
		Attribute::checkcrossbinds(VertArrObj::bound_obj());
		let (num, offset, mode) = args.get();
		GLCheck!(gl::DrawArrays(mode, i32(offset), num));
	}
	pub fn DrawInstanced<T>(&self, n: T, args: impl DrawArgs)
	where
		i32: Cast<T>,
	{
		Index::checkcrossbinds(VertArrObj::bound_obj());
		Attribute::checkcrossbinds(VertArrObj::bound_obj());
		let (num, offset, mode) = args.get();
		let offset = (offset * type_size!(I)) as *const GLvoid;
		GLCheck!(gl::DrawElementsInstanced(mode, num, I::TYPE, offset, i32(n)));
	}
}
