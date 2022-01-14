use crate::uses::*;

#[macro_export]
macro_rules! UnsafeOnce {
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
macro_rules! UnsafeLocal {
	($t: ty, $b: block) => {{
		use std::cell::UnsafeCell;
		thread_local!(static S: UnsafeCell<Option<$t>> = UnsafeCell::new(None));
		let mut r = None;
		unsafe {
			S.with(|f| {
				let f = f.get();
				if (*f).is_some() {
				} else {
					*f = Some($b);
				}
				r = Some(f)
			});
			(*r.unwrap_unchecked()).as_mut().unwrap_unchecked()
		}
	}};
}

#[macro_export]
macro_rules! FnStatic {
	($n: ident: $t: ty, $b: block) => {
		fn $n() -> &'static mut $t {
			UnsafeOnce!($t, { Def() })
		}
		*$n() = $b;
	};
}

#[macro_export]
macro_rules! FnLocal {
	($n: ident: $t: ty, $b: block) => {
		fn $n() -> &'static mut $t {
			UnsafeLocal!($t, { Def() })
		}
		*$n() = $b;
	};
}

#[derive(Debug)]
pub struct static_ptr<T: Send + Sync> {
	t: Dummy<T>,
	ptr: usize,
}
impl<T: Send + Sync> static_ptr<T> {
	pub unsafe fn new(t: &T) -> Self {
		let ptr = t as *const T as usize;
		Self { ptr, t: Dummy }
	}
	pub fn get(&self) -> &'static T {
		unsafe { &*(self.ptr as *const T) }
	}
	#[allow(unused_mut)]
	pub fn get_mut(&mut self) -> &'static mut T {
		unsafe { &mut *(self.ptr as *mut T) }
	}
}
impl<T: Send + Sync> Copy for static_ptr<T> {}
impl<T: Send + Sync> Clone for static_ptr<T> {
	fn clone(&self) -> Self {
		Self { ptr: self.ptr, t: Dummy }
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
