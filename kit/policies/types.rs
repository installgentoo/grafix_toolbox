pub mod cached;
pub mod cached_str;
pub mod lazy;
pub mod memoized;
pub mod prefetch;

pub mod ext {
	#[derive(Debug)]
	pub struct TPtr<T> {
		ptr: usize,
		t: Dummy<T>,
	}
	impl<T: Send + Sync> TPtr<T> {
		pub unsafe fn new(t: &mut T) -> Self {
			let ptr = t as *mut T as usize;
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
}

#[macro_export]
macro_rules! typed_ptr {
	($n: expr) => {{
			unsafe { TPtr::new($n) }
	}};
	($($n: expr),+) => {{
			unsafe { ($(TPtr::new($n),)+) }
	}};
}

#[macro_export]
macro_rules! LazyStatic {
	($t: ty, $b: block) => {{
		use std::sync::{Mutex, OnceLock};
		static S: OnceLock<Mutex<$t>> = OnceLock::new();
		S.get_or_init(|| Mutex::new($b)).lock().fail()
	}};
	($t: ty) => {
		LazyStatic!($t, { <$t>::default() })
	};
}

#[macro_export]
macro_rules! LocalStatic {
	($t: ty, $b: block) => {{
		use std::{cell::OnceCell, cell::Cell};
		thread_local!(static S: OnceCell<Cell<$t>> = Default::default());
		let r = S.with(|f| f.get_or_init(|| Cell::new($b)).as_ptr());
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
