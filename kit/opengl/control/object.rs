use super::state::*;
use crate::stdlib::*;

#[derive(Debug)]
pub struct Object<T: State> {
	t: Dummy<T>,
	pub obj: u32,
}
impl<T: State> Object<T> {
	pub fn new() -> Self {
		let obj = T::New();
		Self { t: Dummy, obj }
	}
}
impl<T: State> Drop for Object<T> {
	fn drop(&mut self) {
		T::Drop(self.obj);
	}
}
impl<T: State> Default for Object<T> {
	fn default() -> Self {
		Self::new()
	}
}

pub struct Binding<'l, T: State>(Dummy<&'l T>);
impl<T: State> Binding<'_, T> {
	pub fn new(o: &mut Object<T>) -> Self {
		T::Lock(o.obj);
		T::Bind(o.obj);
		Self(Dummy)
	}
	pub fn zero() -> Self {
		T::Lock(0);
		T::Bind(0);
		Self(Dummy)
	}
}
impl<T: State> Drop for Binding<'_, T> {
	fn drop(&mut self) {
		T::Unlock();
	}
}

#[derive(Debug)]
pub struct ArrObject<T: State, D> {
	t: Dummy<T>,
	d: Dummy<D>,
	pub obj: u32,
	pub len: usize,
}
impl<T: State, D> ArrObject<T, D> {
	pub fn new_empty(len: usize) -> Self {
		let (t, d, obj) = (Dummy, Dummy, T::New());
		Self { t, d, obj, len }
	}
	pub fn size(&self) -> usize {
		self.len * type_size::<D>()
	}
}
impl<T: State, D> Drop for ArrObject<T, D> {
	fn drop(&mut self) {
		T::Drop(self.obj);
	}
}
impl<T: State, D> Default for ArrObject<T, D> {
	fn default() -> Self {
		Self::new_empty(0)
	}
}
