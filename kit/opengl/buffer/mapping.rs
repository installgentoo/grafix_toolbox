use super::*;

impl<T: Buffer, D> ArrObject<T, D> {
	pub fn new(args: impl AllocArgs<D>) -> Self {
		let (ptr, size, usage) = args.geta();
		let o = Self::new_empty(size);
		GLCheck!(glBufferStorage(T::TYPE, o.obj, isize(o.size()), ptr, usage));
		o
	}
	pub fn Update(&mut self, args: impl UpdateArgs<D>) {
		let (ptr, size, offset) = args.getu();
		ASSERT!(self.len >= offset + size, "Buffer {}({}) updated out of bounds", self.obj, type_name::<T>());
		let s = type_size::<D>();
		GLCheck!(glBufferSubData(T::TYPE, self.obj, isize(offset * s), isize(size * s), ptr));
	}
	pub fn Map(&mut self) -> Mapping<T, D> {
		self.MapRange(0)
	}
	pub fn MapMut(&mut self) -> MappingMut<T, D> {
		self.MapRangeMut(0)
	}
	pub fn MapRange(&mut self, args: impl MappingArgs) -> Mapping<T, D> {
		let (offset, len, access) = get_mapping_args(self, args);
		Mapping::new(self, offset, len, access | gl::MAP_READ_BIT)
	}
	pub fn MapRangeMut(&mut self, args: impl MappingArgs) -> MappingMut<T, D> {
		let (offset, len, access) = get_mapping_args(self, args);
		MappingMut::new(self, offset, len, access | gl::MAP_WRITE_BIT)
	}
}

pub struct Mapping<'l, T: Buffer, D> {
	o: &'l ArrObject<T, D>,
	size: usize,
	pub raw_mem: *const D,
}
impl<'l, T: Buffer, D> Mapping<'l, T, D> {
	pub fn new(o: &'l ArrObject<T, D>, offset: isize, len: usize, access: GLbitfield) -> Self {
		let raw_mem = GLCheck!(glMapBufferRange(T::TYPE, o.obj, offset, isize(len), access)) as *const D;
		Self { o, size: len / type_size::<D>(), raw_mem }
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
		Self { o, size: len / type_size::<D>(), raw_mem }
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
