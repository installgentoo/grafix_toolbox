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
}

#[macro_export]
macro_rules! LocalStatic {
	($t: ty, $b: block) => {{
		use std::{mem::*, cell::*};
		thread_local!(static S: OnceCell<ManuallyDrop<UnsafeCell<$t>>> = Default::default());
		let r = S.with(|f| f.get_or_init(|| ManuallyDrop::new(UnsafeCell::new($b))).get());
		unsafe { &mut *r }
	}};
}

#[derive(Debug)]
pub struct static_ptr<T: Send + Sync> {
	t: std::marker::PhantomData<T>,
	ptr: usize,
}
impl<T: Send + Sync> static_ptr<T> {
	pub unsafe fn new(t: &mut T) -> Self {
		let ptr = t as *const T as usize;
		Self { ptr, t: std::marker::PhantomData }
	}
	pub fn get(&self) -> &'static T {
		unsafe { &*(self.ptr as *const T) }
	}
	pub fn get_mut(&mut self) -> &'static mut T {
		unsafe { &mut *(self.ptr as *mut T) }
	}
}
impl<T: Send + Sync> Copy for static_ptr<T> {}
impl<T: Send + Sync> Clone for static_ptr<T> {
	fn clone(&self) -> Self {
		Self {
			ptr: self.ptr,
			t: std::marker::PhantomData,
		}
	}
}

#[macro_export]
macro_rules! StaticPtr {
	($n: expr) => {{
			unsafe { static_ptr::new($n) }
	}};
	($($n: expr),+) => {{
			unsafe { ($(static_ptr::new($n),)+) }
	}};
}
