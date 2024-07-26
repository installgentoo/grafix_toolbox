use super::*;
use std::ops::{RangeFrom, RangeFull, RangeTo};

type Args = (*const GLvoid, usize, GLenum);
pub trait AllocArgs<T> {
	fn geta(&self) -> Args;
}
impl<T> AllocArgs<T> for Args {
	fn geta(&self) -> Self {
		*self
	}
}
impl<S: AsRef<[T]>, T> AllocArgs<T> for (S, GLenum) {
	fn geta(&self) -> Args {
		let slice = self.0.as_ref();
		(slice.as_ptr() as *const GLvoid, slice.len(), self.1)
	}
}
impl<T> AllocArgs<T> for &[T] {
	fn geta(&self) -> Args {
		(*self, gl::DYNAMIC_STORAGE_BIT | gl::MAP_READ_BIT | gl::MAP_WRITE_BIT).geta()
	}
}

type UArgs = (*const GLvoid, usize, usize);
pub trait UpdateArgs<T> {
	fn getu(&self) -> UArgs;
}
impl<T> UpdateArgs<T> for UArgs {
	fn getu(&self) -> Self {
		*self
	}
}
impl<S: AsRef<[T]>, T, O: Copy> UpdateArgs<T> for (S, O)
where
	usize: Cast<O>,
{
	fn getu(&self) -> UArgs {
		let slice = self.0.as_ref();
		(slice.as_ptr() as *const GLvoid, slice.len(), usize(self.1))
	}
}
impl<T> UpdateArgs<T> for &[T] {
	fn getu(&self) -> UArgs {
		(*self, 0).getu()
	}
}

type RArgs = (usize, usize, GLenum);
pub trait MappingArgs {
	fn get(self) -> RArgs;
}
impl<I> MappingArgs for (Range<I>, GLenum)
where
	usize: Cast<I>,
{
	fn get(self) -> RArgs {
		(usize(self.0.start), usize(self.0.end), self.1)
	}
}
impl<I> MappingArgs for (RangeTo<I>, GLenum)
where
	usize: Cast<I>,
{
	fn get(self) -> RArgs {
		(0..usize(self.0.end), self.1).get()
	}
}
impl<I> MappingArgs for (RangeFrom<I>, GLenum)
where
	usize: Cast<I>,
{
	fn get(self) -> RArgs {
		(usize(self.0.start)..0, self.1).get()
	}
}
impl MappingArgs for (RangeFull, GLenum) {
	fn get(self) -> RArgs {
		(0..0, self.1).get()
	}
}
impl<I> MappingArgs for Range<I>
where
	usize: Cast<I>,
{
	fn get(self) -> RArgs {
		(self, 0).get()
	}
}
impl<I> MappingArgs for RangeTo<I>
where
	usize: Cast<I>,
{
	fn get(self) -> RArgs {
		(self, 0).get()
	}
}
impl<I> MappingArgs for RangeFrom<I>
where
	usize: Cast<I>,
{
	fn get(self) -> RArgs {
		(self, 0).get()
	}
}
impl MappingArgs for RangeFull {
	fn get(self) -> RArgs {
		(self, 0).get()
	}
}

pub fn get_mapping_args<T: State, D>(o: &ArrObject<T, D>, args: impl MappingArgs) -> (isize, usize, GLenum) {
	let (start, end, access) = args.get();
	let end = end.or_val(end != 0, o.len);
	ASSERT!(start < end, "Buffer {}({}) access with malformed range", o.obj, type_name::<T>());
	ASSERT!(end <= o.len, "Buffer {}({}) mapped out of bounds", o.obj, type_name::<T>());
	let tsize = type_size::<D>();
	(isize(start * tsize), (end - start) * tsize, access)
}
