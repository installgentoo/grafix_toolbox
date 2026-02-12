pub use {format::*, mapping::*};

pub type AttrArr<D> = ArrObj<Attribute, D>;
pub type IdxArr<D> = ArrObj<Index, D>;

pub type UniformArr = ShdArrObj<Uniform, f32>;
pub type ShdStorageArr = ShdArrObj<ShdStorage, f32>;

#[derive(Debug)]
pub struct ShdArrObj<T: ShdBuffType, D> {
	pub array: ArrObj<T, D>,
	loc: Cell<u32>,
}
impl<T: ShdBuffType, D> ShdArrObj<T, D> {
	pub fn new(args: impl AllocArgs<D>) -> Self {
		ArrObj::new(args).into()
	}
}
impl<T: ShdBuffType, D> Drop for ShdArrObj<T, D> {
	fn drop(&mut self) {
		UniformState::<T>::drop(self.array.obj);
	}
}
impl<T: ShdBuffType, D> From<ArrObj<T, D>> for ShdArrObj<T, D> {
	fn from(array: ArrObj<T, D>) -> Self {
		let (size, max) = (array.size(), T::max_size());
		if size > max {
			FAIL!("GL {} buffer({}|{size}) exceeds maximum size {max}", type_name::<T>(), array.obj);
		}
		Self { array, loc: Def() }
	}
}
impl UniformArr {
	pub fn Bind(&self) -> ShdArrBind<Uniform> {
		let loc = self.loc.take();
		let (b, l) = ShdArrBind::<Uniform>::new(self, loc);
		self.loc.set(l);
		b
	}
}
impl ShdStorageArr {
	pub fn Bind(&self, loc: u32) -> Option<ShdArrBind<ShdStorage>> {
		ShdArrBind::<ShdStorage>::new(self, loc)
	}
}

pub struct ShdArrBind<'l, T: ShdBuffType> {
	t: Dummy<&'l T>,
	pub l: u32,
}
impl<'l> ShdArrBind<'l, Uniform> {
	fn new(o: &'l UniformArr, hint: u32) -> (Self, u32) {
		let l = UniformState::<Uniform>::Bind(o.array.obj, hint);
		(Self { t: Dummy, l }, l)
	}
}
impl<'l> ShdArrBind<'l, ShdStorage> {
	fn new(o: &'l ShdStorageArr, loc: u32) -> Option<Self> {
		if !UniformState::<ShdStorage>::BindLocation(o.array.obj, loc) {
			None?
		}
		Self { t: Dummy, l: loc }.into()
	}
}
impl<T: ShdBuffType> Clone for ShdArrBind<'_, T> {
	fn clone(&self) -> Self {
		let Self { t, l } = *self;
		UniformState::<T>::Clone(l);
		Self { t, l }
	}
}
impl<T: ShdBuffType> Drop for ShdArrBind<'_, T> {
	fn drop(&mut self) {
		UniformState::<T>::Unbind(self.l);
	}
}

mod args;
mod format;
mod mapping;

use {super::internal::*, crate::lib::*, args::*};
