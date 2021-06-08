use super::state::*;
use crate::uses::Dummy;

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

pub struct Binding<'l, T: State> {
	t: Dummy<&'l T>,
}
impl<'l, T: State> Binding<'l, T> {
	pub fn new(o: &'l mut Object<T>) -> Self {
		T::Lock(o.obj);
		T::Bind(o.obj);
		Self { t: Dummy }
	}
}
impl<'l, T: State> Drop for Binding<'l, T> {
	fn drop(&mut self) {
		T::Unlock();
	}
}

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
