use super::{args::*, bindless::*, format::*, mapping::*, object::*, policy::*, state::*};
use crate::uses::*;

pub type AttrArr<D> = ArrObject<Attribute, D>;
pub type IdxArr<D> = ArrObject<Index, D>;

impl<D: AttrType> ArrObject<Attribute, D> {
	pub fn new(args: impl AllocArgs<D>) -> Self {
		Self::allocate(args)
	}
}
impl<D: IdxType> ArrObject<Index, D> {
	pub fn new(args: impl AllocArgs<D>) -> Self {
		Self::allocate(args)
	}
}
impl<T: State + Buffer, D: Copy> ArrObject<T, D> {
	pub fn allocate(args: impl AllocArgs<D>) -> Self {
		let (ptr, size, usage) = args.geta();
		let o = Self::new_empty(size);
		GLCheck!(glBufferStorage(T::TYPE, o.obj, isize::to(size * type_size!(D)), ptr, usage));
		o
	}
	pub fn Update(&mut self, args: impl UpdateArgs<D>) {
		let (ptr, size, offset) = args.getu();
		ASSERT!(self.len >= offset + size, "Buffer {}({}) updated out of bounds", self.obj, type_name!(T));
		let s = type_size!(D);
		GLCheck!(glBufferSubData(T::TYPE, self.obj, isize::to(offset * s), isize::to(size * s), ptr));
	}
	//TODO Async Mappings
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
