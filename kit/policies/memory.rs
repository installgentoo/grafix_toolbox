pub struct Ptr<T> {
	p: *mut T, // TODO Is Box still noalias?
}
impl<T> Drop for Ptr<T> {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.p) };
	}
}
impl<T> Ptr<T> {
	pub fn weak(&self) -> mem::ManuallyDrop<Self> {
		mem::ManuallyDrop::new(Ptr { p: self.p })
	}
	pub unsafe fn as_ref(&self) -> &'static T {
		unsafe { &*self.p }
	}
	pub fn ptr(&self) -> *mut T {
		self.p
	}
}
impl<T> From<Box<T>> for Ptr<T> {
	fn from(b: Box<T>) -> Self {
		Self { p: Box::into_raw(b) }
	}
}
impl<T: Default> Default for Ptr<T> {
	fn default() -> Self {
		Box::<T>::default().into()
	}
}
impl<T: Debug> Debug for Ptr<T> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		f.debug_tuple("Ptr").field(unsafe { &*self.p }).finish()
	}
}
unsafe impl<T> Send for Ptr<T> {}
unsafe impl<T> Sync for Ptr<T> {}

#[macro_export]
macro_rules! LazyStatic {
	($t: ty, $b: block) => {{
		use {std::sync::OnceLock, $crate::lib::Mutex};
		static S: OnceLock<Mutex<$t>> = OnceLock::new();
		S.get_or_init(|| Mutex::new($b)).lock()
	}};
	($t: ty) => {
		LazyStatic!($t, { <$t>::default() })
	};
}

#[macro_export]
macro_rules! LocalStatic {
	($t: ty, $b: block) => {{
		use std::{cell::OnceCell, cell::Cell};
		thread_local!(static S: OnceCell<Cell<$t>> = <_>::default());
		let r = S.with(|f| f.get_or_init(|| $b.into()).as_ptr());
		unsafe { &mut *r }
	}};
	($t: ty) => {
		LocalStatic!($t, { <$t>::default() })
	};
}

#[macro_export]
macro_rules! LeakyStatic {
	($t: ty, $b: block) => {{
		use std::mem::ManuallyDrop;
		LocalStatic!(ManuallyDrop<$t>, { ManuallyDrop::new($b) }) as &mut $t
	}};
	($t: ty) => {
		LeakyStatic!($t, { <$t>::default() })
	};
}

use crate::lib::*;
