#[macro_use]
mod log;

#[cfg(feature = "rng")]
pub mod rand;
#[cfg(not(feature = "rng"))]
pub mod rand {}

pub mod chksum;
pub mod faster;
pub mod file;
pub mod logger;
pub mod n_iter;
pub mod range;
pub mod result;
pub mod serde;
pub mod slicing;
pub mod vec;

pub mod ext {
	pub trait Pipe: Sized {
		#[must_use]
		#[inline(always)]
		#[allow(async_fn_in_trait)]
		async fn tap_async(mut self, func: impl AsyncFnOnce(&mut Self)) -> Self {
			func(&mut self).await;
			self
		}
		#[must_use]
		#[inline(always)]
		fn tap(mut self, func: impl FnOnce(&mut Self)) -> Self {
			func(&mut self);
			self
		}
		#[inline(always)]
		fn pipe<R: Sized>(self, func: impl FnOnce(Self) -> R) -> R {
			func(self)
		}
		#[inline(always)]
		fn pipe_as<'a, T: 'a + ?Sized, R: 'a + Sized>(&'a self, func: impl FnOnce(&'a T) -> R) -> R
		where
			Self: std::ops::Deref<Target = T>,
		{
			func(std::ops::Deref::deref(self))
		}
	}
	impl<T: Sized> Pipe for T {}

	pub trait OrAssignment: Sized {
		fn or_def(self, filter: bool) -> Self;
		fn or_val(self, filter: bool, f: impl FnOnce() -> Self) -> Self; // TODO impl val/make for Sized + !&
		fn or_map(self, filter: impl FnOnce(&Self) -> bool, f: impl FnOnce(Self) -> Self) -> Self;
	}
	impl<S: Default> OrAssignment for S {
		#[inline(always)]
		fn or_def(self, filter: bool) -> Self {
			if filter {
				return self;
			}
			Self::default()
		}
		#[inline(always)]
		fn or_val(self, filter: bool, f: impl FnOnce() -> Self) -> Self {
			if filter {
				return self;
			}
			f()
		}
		#[inline(always)]
		fn or_map(self, filter: impl FnOnce(&Self) -> bool, f: impl FnOnce(Self) -> Self) -> Self {
			if filter(&self) {
				return self;
			}
			f(self)
		}
	}
}
