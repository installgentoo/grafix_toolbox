use super::*;

pub struct Mapping<'l, T: Buffer, D> {
	o: &'l ArrObject<T, D>,
	size: usize,
	pub raw_mem: *const D,
}
impl<'l, T: Buffer, D> Mapping<'l, T, D> {
	pub fn new(o: &'l mut ArrObject<T, D>, offset: isize, len: usize, access: GLbitfield) -> Self {
		let raw_mem = GLCheck!(glMapBufferRange(T::TYPE, o.obj, offset, isize(len), access)) as *const D;
		Self { o, size: len / type_size!(D), raw_mem }
	}
	pub fn mem(&self) -> &'l [D] {
		unsafe { slice::from_raw_parts(self.raw_mem, self.size) }
	}
}
impl<T: Buffer, D> Drop for Mapping<'_, T, D> {
	fn drop(&mut self) {
		let _valid = GLCheck!(glUnmapBuffer(T::TYPE, self.o.obj));
		ASSERT!(_valid == gl::TRUE, "Buffer memory was corrupted by OS");
	}
}

pub struct MappingMut<'l, T: Buffer, D> {
	o: &'l ArrObject<T, D>,
	size: usize,
	pub raw_mem: *mut D,
}
impl<'l, T: Buffer, D> MappingMut<'l, T, D> {
	pub fn new(o: &'l mut ArrObject<T, D>, offset: isize, len: usize, access: GLbitfield) -> Self {
		let raw_mem = GLCheck!(glMapBufferRange(T::TYPE, o.obj, offset, isize(len), access)) as *mut D;
		Self { o, size: len / type_size!(D), raw_mem }
	}
	pub fn mem(&self) -> &'l mut [D] {
		unsafe { slice::from_raw_parts_mut(self.raw_mem, self.size) }
	}
}
impl<T: Buffer, D> Drop for MappingMut<'_, T, D> {
	fn drop(&mut self) {
		let _valid = GLCheck!(glUnmapBuffer(T::TYPE, self.o.obj));
		ASSERT!(_valid == gl::TRUE, "Buffer memory was corrupted by OS");
	}
}
