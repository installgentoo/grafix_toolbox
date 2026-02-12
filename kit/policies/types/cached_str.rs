use crate::lib::*;

#[derive_as_ser]
pub struct CachedStr {
	str: String,
	#[cfg_attr(feature = "adv_fs", serde(skip))]
	old_str: Cell<Cache>,
	same: Cell<bool>,
}

impl CachedStr {
	#[allow(clippy::should_implement_trait)]
	pub fn clone(&self) -> Astr {
		let _ = self.changed();
		(&self.old_str).bind().s.clone()
	}
	pub fn str(&mut self) -> &mut String {
		let Self { str, same, .. } = self;
		same.set(false);
		str
	}
	#[must_use]
	pub fn changed(&self) -> bool {
		if *(&self.same).bind() || self.str_check(true) {
			return false;
		}
		DEBUG!("CachedStr {:?} changed", self.str);
		true
	}
	pub fn accessed(&self) -> bool {
		if *(&self.same).bind() || self.str_check(false) {
			self.same.set(true);
			return false;
		}
		true
	}
	fn str_check(&self, flush: bool) -> bool {
		let Self { str, old_str, same } = self;
		let ptr = str.as_ptr();
		let eq = {
			let &Cache { p, ref s } = old_str.bind();
			ptr::eq(ptr, p) && str[..] == s[..]
		};
		if !eq && flush {
			old_str.set(Cache { s: str.clone().into(), p: ptr });
			same.set(true);
		}
		eq
	}
}
impl AsRef<str> for CachedStr {
	fn as_ref(&self) -> &str {
		self
	}
}
impl ops::Deref for CachedStr {
	type Target = str;

	fn deref(&self) -> &str {
		&self.str
	}
}
impl Default for CachedStr {
	fn default() -> Self {
		let (str, old_str) = Def();
		Self { str, old_str, same: true.into() }
	}
}
impl Debug for CachedStr {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		if *(&self.same).bind() {
			write!(f, "{:?}", self.str)
		} else {
			write!(f, "{:?}|{:?}", self.str, (&self.old_str).bind())
		}
	}
}
impl Display for CachedStr {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		write!(f, "{}", self.str)
	}
}
impl<S: Into<String>> From<S> for CachedStr {
	fn from(s: S) -> Self {
		let (same, old_str) = Def();
		Self { str: s.into(), same, old_str }
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
		&self.str == r
	}
}
unsafe impl Send for CachedStr {}

#[derive(Debug)]
struct Cache {
	s: Astr,
	p: *const u8,
}
impl Default for Cache {
	fn default() -> Self {
		Self { s: Def(), p: ptr::null() }
	}
}
