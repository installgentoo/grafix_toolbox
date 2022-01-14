use crate::uses::*;

pub trait Fetcher<K, T> {
	fn get(&self, _: K) -> &T;
	fn take(&self, _: K) -> T;
}

pub struct Prefetched<'a, K, T, M: Fetcher<K, T>> {
	s: UnsafeCell<mSelf<'a, K, T, M>>,
}
enum mSelf<'a, K, T, M: Fetcher<K, T>> {
	Started(Box<Option<(K, &'a M)>>),
	Done(&'a T),
}

impl<'a, K, T, M: Fetcher<K, T>> Prefetched<'a, K, T, M> {
	pub fn new(k: K, m: &'a M) -> Self {
		Self {
			s: UnsafeCell::new(Started(Box::new(Some((k, m))))),
		}
	}
	pub fn get(&self) -> &T {
		let s = unsafe { &mut *self.s.get() };
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
		let s = self.s.into_inner();
		match s {
			Done(_) => ASSERT!(false, "Batched value already borrowed, can't take"),
			Started(b) => {
				let (k, m) = b.valid();
				m.take(k)
			}
		}
	}
}

use mSelf::{Done, Started};
