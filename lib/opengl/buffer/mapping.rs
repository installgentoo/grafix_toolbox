use super::{bindless::*, object::*, policy::*, state::*, types::*};
use crate::uses::*;

pub struct Mapping<'l, T: State + Buffer, D: Copy> {
	o: &'l ArrObject<T, D>,
	size: usize,
	pub raw_mem: *const D,
}
impl<'l, T: State + Buffer, D: Copy> Mapping<'l, T, D> {
	pub fn new(o: &'l mut ArrObject<T, D>, offset: isize, len: usize, access: GLbitfield) -> Self {
		let raw_mem = GLCheck!(glMapBufferRange(T::TYPE, o.obj, offset, isize::to(len), access)) as *const D;
		Self {
			o,
			size: len / type_size!(D),
			raw_mem,
		}
	}
	pub fn mem(&self) -> &'l [D] {
		unsafe { slice::from_raw_parts(self.raw_mem, self.size) }
	}
}
impl<'l, T: State + Buffer, D: Copy> Drop for Mapping<'l, T, D> {
	fn drop(&mut self) {
		let _valid = GLCheck!(glUnmapBuffer(T::TYPE, self.o.obj));
		ASSERT!(_valid == gl::TRUE, "Buffer memory was corrupted by OS");
	}
}

pub struct MappingMut<'l, T: State + Buffer, D: Copy> {
	o: &'l ArrObject<T, D>,
	size: usize,
	pub raw_mem: *mut D,
}
impl<'l, T: State + Buffer, D: Copy> MappingMut<'l, T, D> {
	pub fn new(o: &'l mut ArrObject<T, D>, offset: isize, len: usize, access: GLbitfield) -> Self {
		let raw_mem = GLCheck!(glMapBufferRange(T::TYPE, o.obj, offset, isize::to(len), access)) as *mut D;
		Self {
			o,
			size: len / type_size!(D),
			raw_mem,
		}
	}
	pub fn mem(&self) -> &'l mut [D] {
		unsafe { slice::from_raw_parts_mut(self.raw_mem, self.size) }
	}
}
impl<'l, T: State + Buffer, D: Copy> Drop for MappingMut<'l, T, D> {
	fn drop(&mut self) {
		let _valid = GLCheck!(glUnmapBuffer(T::TYPE, self.o.obj));
		ASSERT!(_valid == gl::TRUE, "Buffer memory was corrupted by OS");
	}
}
