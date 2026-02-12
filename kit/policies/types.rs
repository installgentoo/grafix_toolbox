pub mod lazy {
	pub use super::{arc_slice::*, cached::*, cached_str::*, feed::*, lazy_cell::*, memoized::MemRes, memoized::Memoized, prefetch::*, ver_vec::*};
}

pub type STR = &'static str;
pub type Str = Box<str>;
pub type Astr = Arc<str>;

#[inline(always)]
pub fn Arc<T>(v: T) -> Arc<T> {
	Arc::new(v)
}
#[inline(always)]
pub fn Box<T>(v: T) -> Box<T> {
	Box::new(v)
}
#[inline(always)]
pub fn Cell<T>(v: T) -> Cell<T> {
	Cell::new(v)
}
#[inline(always)]
pub fn Def<T: Default>() -> T {
	<_>::default()
}

pub trait MutateCell<'i, T: 'i> {
	fn mutate<R: Default>(&self, with: impl FnOnce(&'i mut T) -> R) -> R;
}
impl<'i, T: 'i> MutateCell<'i, T> for Cell<T> {
	#[inline(always)]
	fn mutate<R: Default>(&self, with: impl FnOnce(&'i mut T) -> R) -> R {
		with(unsafe { &mut *self.as_ptr() })
	}
}

pub trait InspectCell<'s, T> {
	fn bind(&'s self) -> &'s T;
}
impl<'s, T> InspectCell<'s, T> for &'s Cell<T> {
	#[inline(always)]
	fn bind(&'s self) -> &'s T {
		unsafe { &*self.as_ptr() }
	}
}
impl<'s, T> InspectCell<'s, T> for &'s mut Cell<T> {
	#[inline(always)]
	fn bind(&'s self) -> &'s T {
		unsafe { &*self.as_ptr() }
	}
}

mod arc_slice;
mod cached;
mod cached_str;
mod feed;
mod lazy_cell;
mod memoized;
mod prefetch;
mod ver_vec;

use std::{cell::Cell, sync::Arc};
