use {super::state::*, crate::stdlib::*};

#[derive(Debug)]
pub struct Obj<T: State> {
	t: Dummy<Cell<T>>,
	pub obj: u32,
}
impl<T: State> Obj<T> {
	pub fn new() -> Self {
		let obj = T::New();
		Self { t: Dummy, obj }
	}
}
impl<T: State> Drop for Obj<T> {
	fn drop(&mut self) {
		T::Drop(self.obj);
	}
}
impl<T: State> Default for Obj<T> {
	fn default() -> Self {
		Self::new()
	}
}
impl<T: State> Eq for Obj<T> {}
impl<T: State> PartialEq for Obj<T> {
	fn eq(&self, r: &Self) -> bool {
		self.obj == r.obj
	}
}

pub struct Bind<'l, T: State>(Dummy<&'l *const T>);
impl<T: State> Bind<'_, T> {
	pub fn new(o: &Obj<T>) -> Self {
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
impl<T: State> Drop for Bind<'_, T> {
	fn drop(&mut self) {
		T::Unlock();
	}
}

#[derive(Debug)]
pub struct ArrObj<T: State, D> {
	t: Dummy<(Cell<T>, D)>,
	pub obj: u32,
	pub len: usize,
}
impl<T: State, D> ArrObj<T, D> {
	pub fn new_empty(len: usize) -> Self {
		Self { t: Dummy, obj: T::New(), len }
	}
	pub fn size(&self) -> usize {
		self.len * type_size::<D>()
	}
}
impl<T: State, D> Drop for ArrObj<T, D> {
	fn drop(&mut self) {
		T::Drop(self.obj);
	}
}
impl<T: State, D> Default for ArrObj<T, D> {
	fn default() -> Self {
		Self::new_empty(0)
	}
}
impl<T: State, D> Eq for ArrObj<T, D> {}
impl<T: State, D> PartialEq for ArrObj<T, D> {
	fn eq(&self, r: &Self) -> bool {
		self.obj == r.obj
	}
}
pub trait ArrObjLease {}
impl<T: State, D> ArrObjLease for ArrObj<T, D> {}
impl ArrObjLease for () {}
