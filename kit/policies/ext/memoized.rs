use crate::lib::*;
use std::{borrow::Borrow, fmt, ops};
use {Clone as C, PartialEq as P};

pub struct Memoized<T, A: P + C> {
	last_args: Cell<A>,
	func: fn(&A) -> T,
	val: Cell<Option<T>>,
}
impl<T, A: P + C + Default> Memoized<T, A> {
	pub fn zero(func: fn(&A) -> T) -> Self {
		Self { last_args: Cell::new(Def()), func, val: Cell::new(None) }
	}
	pub fn set(&mut self, v: T) {
		let Self { last_args, val, .. } = self;
		last_args.replace(Def());
		val.replace(Some(v));
	}
}
impl<T, A: P + C> Memoized<T, A> {
	pub fn new(func: fn(&A) -> T, a: impl Into<A>) -> Self {
		Self { last_args: Cell::new(a.into()), func, val: Cell::new(None) }
	}
	pub fn apply(&self, a: impl Borrow<A>) -> (bool, &T) {
		let Self { last_args, func, val } = self;
		let a = a.borrow();
		let changed = if unsafe { (*val.as_ptr()).is_none() || a != &*last_args.as_ptr() } {
			val.set(Some(func(a)));
			last_args.set(a.clone());
			true
		} else {
			false
		};

		(changed, unsafe { (*val.as_ptr()).as_ref().valid() })
	}
	pub fn get_mut(&mut self) -> &mut T {
		let Self { last_args, func, val } = self;
		if unsafe { (*val.as_ptr()).is_none() } {
			let v = func(unsafe { &*last_args.as_ptr() });
			val.set(Some(v));
		}

		unsafe { (*val.as_ptr()).as_mut().valid() }
	}
	pub fn get(&self) -> &T {
		let Self { last_args, func, val } = self;
		if unsafe { (*val.as_ptr()).is_none() } {
			let v = func(unsafe { &*last_args.as_ptr() });
			val.set(Some(v));
		}

		unsafe { (*val.as_ptr()).as_ref().valid() }
	}
	pub fn take(&self) -> T {
		self.get();
		unsafe { (*self.val.as_ptr()).take().valid() }
	}
	pub fn reset(&self) {
		unsafe { (*self.val.as_ptr()).take() };
	}
	pub fn finalize_deserialization(self, func: fn(&A) -> T) -> Self {
		Self { func, ..self }
	}
}
impl<T, A: P + C> AsRef<T> for Memoized<T, A> {
	fn as_ref(&self) -> &T {
		self.get()
	}
}
impl<T, A: P + C> Borrow<T> for Memoized<T, A> {
	fn borrow(&self) -> &T {
		self.get()
	}
}
impl<T, A: P + C> ops::Deref for Memoized<T, A> {
	type Target = T;

	fn deref(&self) -> &T {
		self.get()
	}
}
impl<T, A: P + C + Default> Default for Memoized<T, A> {
	fn default() -> Self {
		let func = |_: &A| ERROR!("Memoized<{},{}>::default() has undefined contents", type_name::<T>(), type_name::<A>());
		let (val, last_args) = Def();
		Self { last_args, func, val }
	}
}
impl<T, A: P + C> C for Memoized<T, A> {
	fn clone(&self) -> Self {
		let Self { last_args: a, func, .. } = self;
		unsafe {
			Self {
				last_args: Cell::new((*a.as_ptr()).clone()),
				func: *func,
				val: Def(),
			}
		}
	}
}
impl<T: fmt::Debug, A: P + C> fmt::Debug for Memoized<T, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.get())
	}
}
impl<T: fmt::Display, A: P + C> fmt::Display for Memoized<T, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.get())
	}
}
impl<T: Eq, A: P + C> Eq for Memoized<T, A> {}
impl<T: P, A: P + C> P for Memoized<T, A> {
	fn eq(&self, r: &Self) -> bool {
		self.get() == r.get()
	}
}

#[cfg(feature = "adv_fs")]
mod serde {
	use {super::*, crate::ser::*};

	impl<T, A: P + C + Serialize> Serialize for Memoized<T, A> {
		fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
			unsafe { &*self.last_args.as_ptr() }.serialize(s)
		}
	}
	impl<'de, T, A: P + C + Deserialize<'de>> Deserialize<'de> for Memoized<T, A> {
		fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
			let func = |_: &A| {
				ERROR!(
					"Deserialization of Memoized<{},{}> requires you to insert appropriate function via .finalize_deserialization()",
					type_name::<T>(),
					type_name::<A>()
				)
			};

			let last_args = Cell::new(A::deserialize(d)?);
			Ok(Self { last_args, func, val: Def() })
		}
	}
}
