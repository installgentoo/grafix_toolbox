use crate::uses::*;

pub struct Cached<T> {
	val: T,
	changed: bool,
}
impl<T> Cached<T> {
	pub fn get(&self) -> &T {
		&self.val
	}
	pub fn new(v: impl Into<T>) -> Self {
		Self { val: v.into(), changed: true }
	}
	pub fn replace(self, v: impl Into<T>) -> Self {
		Self { val: v.into(), changed: true }
	}
	pub fn changed(&mut self) -> bool {
		mem::replace(&mut self.changed, false)
	}
}
impl<T> AsRef<T> for Cached<T> {
	fn as_ref(&self) -> &T {
		&self.val
	}
}
impl<T> Borrow<T> for Cached<T> {
	fn borrow(&self) -> &T {
		&self.val
	}
}
impl<T> ops::Deref for Cached<T> {
	type Target = T;

	fn deref(&self) -> &T {
		&self.val
	}
}
impl<T: Default> Default for Cached<T> {
	fn default() -> Self {
		Self { val: T::default(), changed: true }
	}
}
impl<T: Clone> Clone for Cached<T> {
	fn clone(&self) -> Self {
		Self {
			val: self.val.clone(),
			changed: true,
		}
	}
}
impl<T: Debug> Debug for Cached<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{:?}", self.val)
	}
}
impl<T: fmt::Display> fmt::Display for Cached<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.val)
	}
}
impl<T: Eq> Eq for Cached<T> {}
impl<T: PartialEq> PartialEq for Cached<T> {
	fn eq(&self, r: &Self) -> bool {
		self.val == r.val
	}
}
