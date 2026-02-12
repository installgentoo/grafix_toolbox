use super::*;

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
		(*self, gl::DYNAMIC_STORAGE_BIT | gl::MAP_WRITE_BIT).geta()
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
impl<R: uRange> MappingArgs for (R, GLenum) {
	fn get(self) -> RArgs {
		let (b, e) = self.0.get_range();
		(b, e, self.1)
	}
}
impl<R: uRange> MappingArgs for R {
	fn get(self) -> RArgs {
		(self, 0).get()
	}
}

pub fn get_mapping_args<T: State, D>(o: &ArrObj<T, D>, args: impl MappingArgs) -> (isize, usize, GLenum) {
	let (start, end, access) = args.get();
	let end = end.or_val(end != usize::MAX, || o.len);
	ASSERT!(start < end && end <= o.len, "GL {} buffer {} mapping {end} oob, len {}", type_name::<T>(), o.obj, o.len);
	let tsize = type_size::<D>();
	(isize(start * tsize), (end - start) * tsize, access)
}
