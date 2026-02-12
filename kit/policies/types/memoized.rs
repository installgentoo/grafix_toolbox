use crate::lib::*;

pub struct MemRes<T> {
	pub changed: bool,
	pub val: T,
}

pub trait MemoizedArgs<A> {
	fn take(self) -> A;
	fn equal(&self, r: &A) -> bool;
}
pub struct Memoized<T, A> {
	last_args: A,
	func: fn(&A) -> T,
	val: Cell<Option<T>>,
}
impl<T, A: PC + Default> Memoized<T, A> {
	pub fn zero(func: fn(&A) -> T) -> Self {
		Self { func, ..Def() }
	}
}
impl<T, A: PC> Memoized<T, A> {
	pub fn new(func: fn(&A) -> T, a: impl Into<A>) -> Self {
		Self { last_args: a.into(), func, val: Def() }
	}
	pub fn apply(&mut self, a: impl MemoizedArgs<A>) -> MemRes<&T> {
		let changed = (&self.val).bind().is_none() || *self != a;
		let Self { func, val, last_args } = self;
		if changed {
			let a = a.take();
			val.set(Some(func(&a)));
			*last_args = a;
		}

		MemRes { changed, val: unsafe { &*val.as_ptr() }.as_valid() }
	}
	pub fn get_args(&self) -> &A {
		&self.last_args
	}
	fn get_impl(&self) -> *mut Option<T> {
		let Self { last_args, func, val } = self;
		if val.bind().is_none() {
			let v = func(last_args);
			val.set(Some(v));
		}

		val.as_ptr()
	}
	pub fn take(self) -> T {
		self.as_ref();
		self.val.take().valid()
	}
	pub fn reset(&mut self) {
		self.val.take();
	}
	pub fn finalize_deserialization(self, func: fn(&A) -> T) -> Self {
		Self { func, ..self }
	}
}
impl<T, A: PC> AsRef<T> for Memoized<T, A> {
	fn as_ref(&self) -> &T {
		self
	}
}
impl<T, A: PC> AsMut<T> for Memoized<T, A> {
	fn as_mut(&mut self) -> &mut T {
		unsafe { &mut *self.get_impl() }.mut_valid()
	}
}
impl<T, A: PC> ops::Deref for Memoized<T, A> {
	type Target = T;

	fn deref(&self) -> &T {
		unsafe { &*self.get_impl() }.as_valid()
	}
}
impl<T, A: PC + Default> Default for Memoized<T, A> {
	fn default() -> Self {
		let func = |_: &A| ERROR!("Memoized<{},{}>::default() has undefined contents", type_name::<T>(), type_name::<A>());
		let (val, last_args) = Def();
		Self { last_args, func, val }
	}
}
impl<T, A: PC> Clone for Memoized<T, A> {
	fn clone(&self) -> Self {
		let Self { last_args: a, func, .. } = self;
		Self { last_args: a.clone(), func: *func, val: Def() }
	}
}
impl<T: Debug, A: PC> Debug for Memoized<T, A> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		f.debug_tuple("Memoized").field(&**self).finish()
	}
}
impl<T: Display, A: PC> Display for Memoized<T, A> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		(**self).fmt(f)
	}
}
impl<T, A: PC, R: MemoizedArgs<A>> PartialEq<R> for Memoized<T, A> {
	fn eq(&self, r: &R) -> bool {
		r.equal(self.get_args())
	}
}
impl<T, A: PC> MemoizedArgs<A> for Memoized<T, A> {
	fn take(self) -> A {
		self.last_args
	}
	fn equal(&self, r: &A) -> bool {
		self.get_args() == r
	}
}

#[cfg(feature = "adv_fs")]
mod serde {
	use {super::*, crate::ser::*};

	impl<T, A: PC + Serialize> Serialize for Memoized<T, A> {
		fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
			self.last_args.serialize(s)
		}
	}
	impl<'de, T, A: PC + Deserialize<'de>> Deserialize<'de> for Memoized<T, A> {
		fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
			let func = |_: &A| {
				ERROR!(
					"Deserialization of Memoized<{},{}> requires you to insert appropriate function via .finalize_deserialization()",
					type_name::<T>(),
					type_name::<A>()
				)
			};

			let last_args = A::deserialize(d)?;
			Self { last_args, func, val: Def() }.pipe(Ok)
		}
	}
}

impl<A: PC> MemoizedArgs<A> for A {
	fn take(self) -> A {
		self
	}
	fn equal(&self, r: &A) -> bool {
		self.eq(r)
	}
}

impl<A1: PC, A2: PC> MemoizedArgs<(A1, A2)> for &(A1, A2) {
	fn take(self) -> (A1, A2) {
		self.clone()
	}
	fn equal(&self, r: &(A1, A2)) -> bool {
		self.eq(&r)
	}
}
impl<A1: PC, A2: PC> MemoizedArgs<(A1, A2)> for (&A1, &A2) {
	fn take(self) -> (A1, A2) {
		let (a, b) = self;
		(a.clone(), b.clone())
	}
	fn equal(&self, (r1, r2): &(A1, A2)) -> bool {
		let (a, b) = self;
		a.eq(&r1) && b.eq(&r2)
	}
}

impl<A1: PC, A2: PC, A3: PC> MemoizedArgs<(A1, A2, A3)> for &(A1, A2, A3) {
	fn take(self) -> (A1, A2, A3) {
		self.clone()
	}
	fn equal(&self, r: &(A1, A2, A3)) -> bool {
		self.eq(&r)
	}
}
impl<A1: PC, A2: PC, A3: PC> MemoizedArgs<(A1, A2, A3)> for (&A1, &A2, &A3) {
	fn take(self) -> (A1, A2, A3) {
		let (a, b, c) = self;
		(a.clone(), b.clone(), c.clone())
	}
	fn equal(&self, (r1, r2, r3): &(A1, A2, A3)) -> bool {
		let (a, b, c) = self;
		a.eq(&r1) && b.eq(&r2) && c.eq(&r3)
	}
}

impl<A1: PC, A2: PC, A3: PC, A4: PC> MemoizedArgs<(A1, A2, A3, A4)> for &(A1, A2, A3, A4) {
	fn take(self) -> (A1, A2, A3, A4) {
		self.clone()
	}
	fn equal(&self, r: &(A1, A2, A3, A4)) -> bool {
		self.eq(&r)
	}
}
impl<A1: PC, A2: PC, A3: PC, A4: PC> MemoizedArgs<(A1, A2, A3, A4)> for (&A1, &A2, &A3, &A4) {
	fn take(self) -> (A1, A2, A3, A4) {
		let (a, b, c, d) = self;
		(a.clone(), b.clone(), c.clone(), d.clone())
	}
	fn equal(&self, (r1, r2, r3, r4): &(A1, A2, A3, A4)) -> bool {
		let (a, b, c, d) = self;
		a.eq(&r1) && b.eq(&r2) && c.eq(&r3) && d.eq(&r4)
	}
}

trait_alias!(pub PC, PartialEq + Clone);
