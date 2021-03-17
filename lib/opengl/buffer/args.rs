use super::{object::*, state::*, types::*};
use crate::uses::{slicing::*, *};

type Args = (*const GLvoid, usize, GLenum);
pub trait AllocArgs<T> {
	fn geta(self) -> Args;
}
impl<T> AllocArgs<T> for Args {
	fn geta(self) -> Self {
		self
	}
}
impl<S: Sliceable<T>, T> AllocArgs<T> for (S, GLenum) {
	fn geta(self) -> Args {
		let slice = self.0.slice();
		(slice.as_ptr() as *const GLvoid, slice.len(), self.1)
	}
}
impl<T> AllocArgs<T> for &[T] {
	fn geta(self) -> Args {
		(self, gl::DYNAMIC_STORAGE_BIT | gl::MAP_READ_BIT | gl::MAP_WRITE_BIT).geta()
	}
}
impl<T> AllocArgs<T> for &Vec<T> {
	fn geta(self) -> Args {
		(self[..]).geta()
	}
}
/*impl<T, const N: usize> AllocArgs<T> for [T; N] {//TODO add const generic impl + same for UpdateArgs
	fn geta(self) -> Args {
		(self.as_ptr(), self.len(), _)
	}
}*/

type UArgs = (*const GLvoid, usize, usize);
pub trait UpdateArgs<T> {
	fn getu(self) -> UArgs;
}
impl<T> UpdateArgs<T> for UArgs {
	fn getu(self) -> Self {
		self
	}
}
impl<S: Sliceable<T>, T, O> UpdateArgs<T> for (S, O)
where
	usize: Cast<O>,
{
	fn getu(self) -> UArgs {
		let slice = self.0.slice();
		(slice.as_ptr() as *const GLvoid, slice.len(), usize::to(self.1))
	}
}
impl<T> UpdateArgs<T> for &[T] {
	fn getu(self) -> UArgs {
		(self, 0).getu()
	}
}
impl<T> UpdateArgs<T> for &Vec<T> {
	fn getu(self) -> UArgs {
		(self[..]).getu()
	}
}

type RArgs = (isize, isize, GLenum);
pub trait MappingArgs {
	fn get(self) -> RArgs;
}
impl<O, L> MappingArgs for (O, L, GLenum)
where
	isize: Cast<O> + Cast<L>,
{
	fn get(self) -> RArgs {
		(isize::to(self.0), isize::to(self.1), self.2)
	}
}
impl MappingArgs for GLenum {
	fn get(self) -> RArgs {
		(0, 0, self).get()
	}
}

pub fn get_mapping_args<T: State, D: Copy>(o: &ArrObject<T, D>, args: impl MappingArgs) -> (isize, usize, GLenum) {
	let (offset, len, access) = args.get();
	let len = len.or_val(len >= 1, isize::to(o.len) - offset);
	ASSERT!(isize::to(o.len) >= offset + len && len > 0, "Buffer {}({}) mapped out of bounds", o.obj, type_name!(T));
	let tsize = isize::to(type_size!(D));
	(offset * tsize, usize::to(len * tsize), access)
}
