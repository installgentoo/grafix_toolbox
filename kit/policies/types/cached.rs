use crate::lib::*;

#[derive_as_ser]
pub struct Cached<T> {
	val: T,
	#[cfg_attr(feature = "adv_fs", serde(skip))]
	old: Cell<Option<T>>,
	same: Cell<bool>,
}
impl<T: Clone + PartialEq> Cached<T> {
	#[allow(clippy::should_implement_trait)]
	pub fn clone(&self) -> T {
		let _ = self.changed();
		(&self.old).bind().as_valid().clone()
	}
	#[must_use]
	pub fn changed(&self) -> bool {
		if *(&self.same).bind() || self.check(true) {
			return false;
		}
		true
	}
	pub fn accessed(&self) -> bool {
		if *(&self.same).bind() || self.check(false) {
			self.same.set(true);
			return false;
		}
		true
	}
	fn check(&self, flush: bool) -> bool {
		let Self { val, old, same } = self;
		let eq = old.bind().as_ref().map(|o| o == val).unwrap_or(false);
		if !eq && flush {
			old.set(val.clone().into());
			same.set(true);
		}
		eq
	}
}
impl<T> AsRef<T> for Cached<T> {
	fn as_ref(&self) -> &T {
		self
	}
}
impl<T> ops::Deref for Cached<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.val
	}
}
impl<T> ops::DerefMut for Cached<T> {
	fn deref_mut(&mut self) -> &mut T {
		let Self { val, same, .. } = self;
		same.set(false);
		val
	}
}
impl<T: Default> Default for Cached<T> {
	fn default() -> Self {
		let (val, old) = Def();
		Self { val, old: Some(old).into(), same: true.into() }
	}
}
impl<T: Debug> Debug for Cached<T> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		if *(&self.same).bind() {
			self.val.fmt(f)
		} else {
			write!(f, "{:?}|{:?}", self.val, (&self.old).bind())
		}
	}
}
impl<T: Display> Display for Cached<T> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		self.val.fmt(f)
	}
}
impl<T> From<T> for Cached<T> {
	fn from(val: T) -> Self {
		let (same, old) = Def();
		Self { val, same, old }
	}
}
impl<T: Eq> Eq for Cached<T> {}
impl<T: PartialEq> PartialEq for Cached<T> {
	fn eq(&self, r: &Self) -> bool {
		self.val == r.val
	}
}
impl<T: PartialEq> PartialEq<T> for Cached<T> {
	fn eq(&self, r: &T) -> bool {
		&self.val == r
	}
}
unsafe impl<T: Send> Send for Cached<T> {}
