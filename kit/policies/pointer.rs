#[macro_export]
macro_rules! LazyStatic {
	($t: ty, $b: block) => {{
		static mut S: Option<$t> = None;
		unsafe {
			if S.is_some() {
			} else {
				S = Some($b);
			}
			S.as_mut().unwrap_unchecked()
		}
	}};
	($t: ty) => {
		LazyStatic!($t, { <$t>::default() })
	};
}

#[macro_export]
macro_rules! LocalStatic {
	($t: ty, $b: block) => {{
		use std::{cell::OnceCell, cell::UnsafeCell, mem::ManuallyDrop};
		thread_local!(static S: OnceCell<ManuallyDrop<UnsafeCell<$t>>> = Default::default());
		let r = S.with(|f| f.get_or_init(|| ManuallyDrop::new(UnsafeCell::new($b))).get());
		unsafe { &mut *r }
	}};
	($t: ty) => {
		LocalStatic!($t, { <$t>::default() })
	};
}

#[derive(Debug)]
pub struct TPtr<T: Send + Sync> {
	ptr: usize,
	t: Dummy<T>,
}
impl<T: Send + Sync> TPtr<T> {
	pub unsafe fn new(t: &mut T) -> Self {
		let ptr = t as *const T as usize;
		Self { ptr, t: Dummy }
	}
	pub fn get(&self) -> &'static T {
		unsafe { &*(self.ptr as *const T) }
	}
	pub fn get_mut(&mut self) -> &'static mut T {
		unsafe { &mut *(self.ptr as *mut T) }
	}
}
impl<T: Send + Sync> Copy for TPtr<T> {}
impl<T: Send + Sync> Clone for TPtr<T> {
	fn clone(&self) -> Self {
		*self
	}
}
use std::marker::PhantomData as Dummy;

#[macro_export]
macro_rules! typed_ptr {
	($n: expr) => {{
			unsafe { TPtr::new($n) }
	}};
	($($n: expr),+) => {{
			unsafe { ($(TPtr::new($n),)+) }
	}};
}
