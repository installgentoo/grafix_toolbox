use super::{super::logging, *};
use std::cell::UnsafeCell;

pub trait Fetcher<K, T> {
	fn get(&self, _: K) -> &T;
	fn take(&self, _: K) -> T;
}

pub struct Prefetched<'a, K, T, M: Fetcher<K, T>>(UnsafeCell<mSelf<'a, K, T, M>>);
enum mSelf<'a, K, T, M: Fetcher<K, T>> {
	Started(Box<Option<(K, &'a M)>>),
	Done(&'a T),
}

impl<'a, K, T, M: Fetcher<K, T>> Prefetched<'a, K, T, M> {
	pub fn new(k: K, m: &'a M) -> Self {
		Self(UnsafeCell::new(Started(Box(Some((k, m))))))
	}
	pub fn get(&self) -> &T {
		let s = unsafe { &mut *self.0.get() };
		match s {
			Done(v) => v,
			Started(b) => {
				let (k, m) = b.take().valid();
				let v = m.get(k);
				*s = Done(v);
				v
			}
		}
	}
	pub fn take(self) -> T {
		let s = self.0.into_inner();
		match s {
			Done(_) => ERROR!("Batched value already borrowed, can't take"),
			Started(b) => {
				let (k, m) = b.valid();
				m.take(k)
			}
		}
	}
}

use mSelf::{Done, Started};
