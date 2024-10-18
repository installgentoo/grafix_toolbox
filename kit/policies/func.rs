#[macro_use]
mod logging_def;

#[cfg(feature = "rng")]
pub mod rand;
#[cfg(not(feature = "rng"))]
pub mod rand {}

pub mod chksum;
pub mod file;
pub mod index;
pub mod logging;
pub mod n_iter;
pub mod result;
pub mod slicing;
pub mod vec;

pub mod ext {
	pub trait InspectCell<T> {
		fn inspect<R>(&self, f: impl Fn(&T) -> R) -> R;
	}
	impl<T> InspectCell<T> for std::cell::Cell<T> {
		fn inspect<R>(&self, f: impl Fn(&T) -> R) -> R {
			let s = unsafe { &*self.as_ptr() };
			f(s)
		}
	}

	pub trait UnwrapValid<T> {
		fn valid(self) -> T;
	}
	impl<T> UnwrapValid<T> for Option<T> {
		fn valid(self) -> T {
			#[cfg(debug_assertions)]
			{
				self.expect("E| Not valid: None")
			}
			#[cfg(not(debug_assertions))]
			{
				unsafe { self.unwrap_unchecked() }
			}
		}
	}
	impl<T, E: std::fmt::Debug> UnwrapValid<T> for Result<T, E> {
		fn valid(self) -> T {
			#[cfg(debug_assertions)]
			{
				self.expect("E| Not valid: Err")
			}
			#[cfg(not(debug_assertions))]
			{
				unsafe { self.unwrap_unchecked() }
			}
		}
	}

	pub trait OrAssignment {
		fn or_def(self, filter: bool) -> Self;
		fn or_val(self, filter: bool, val: Self) -> Self;
	}
	impl<T: Default> OrAssignment for T {
		fn or_def(self, filter: bool) -> Self {
			if filter {
				self
			} else {
				Self::default()
			}
		}
		fn or_val(self, filter: bool, v: Self) -> Self {
			if filter {
				self
			} else {
				v
			}
		}
	}
}
