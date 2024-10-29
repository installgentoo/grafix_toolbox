use crate::stdlib::*;

derive_common_OBJ! {
pub struct CachedStr {
	str: String,
	old_str: Box<str>,
	accessed: bool,
}}
impl CachedStr {
	pub fn str(&mut self) -> &mut String {
		let Self { str, accessed, .. } = self;
		*accessed = true;
		str
	}
	pub fn new(s: impl Into<String>) -> Self {
		let (str, old_str) = (s.into(), "".into());
		Self { str, old_str, accessed: true }
	}
	pub fn changed(&mut self) -> bool {
		if !self.check() {
			return false;
		}

		let Self { ref str, old_str, accessed } = self;
		*accessed = false;
		*old_str = str.clone().into();
		true
	}
	pub fn check(&mut self) -> bool {
		let Self { ref str, old_str, accessed } = self;
		if !*accessed || str[..] == old_str[..] {
			*accessed = false;
			return false;
		}
		true
	}
}
impl AsRef<str> for CachedStr {
	fn as_ref(&self) -> &str {
		&self.str
	}
}
impl Borrow<str> for CachedStr {
	fn borrow(&self) -> &str {
		&self.str
	}
}
impl ops::Deref for CachedStr {
	type Target = str;

	fn deref(&self) -> &str {
		&self.str
	}
}
impl fmt::Display for CachedStr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.str)
	}
}
impl<S: Into<String>> From<S> for CachedStr {
	fn from(s: S) -> Self {
		Self::new(s)
	}
}
impl Eq for CachedStr {}
impl PartialEq for CachedStr {
	fn eq(&self, r: &Self) -> bool {
		self.str == r.str
	}
}
impl PartialEq<String> for CachedStr {
	fn eq(&self, r: &String) -> bool {
		self.str == *r
	}
}
