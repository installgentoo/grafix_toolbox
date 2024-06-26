pub use crate::map_variant;
pub mod cached;
pub mod cached_str;
pub mod lazy;
pub mod memoized;
pub mod n_iter;
pub mod prefetch;
pub use {def::*, vec::*};

pub type STR = &'static str;
pub type Str = Box<str>;
pub type Astr = std::sync::Arc<str>;

pub fn Box<T>(v: T) -> Box<T> {
	Box::new(v)
}

pub fn Def<T: Default>() -> T {
	Default::default()
}

pub trait InspectCell<T> {
	fn inspect<R>(&self, f: impl Fn(&T) -> R) -> R;
}
impl<T> InspectCell<T> for std::cell::Cell<T> {
	fn inspect<R>(&self, f: impl Fn(&T) -> R) -> R {
		let s = unsafe { &*self.as_ptr() };
		f(s)
	}
}

pub trait OptionSink {
	fn sink(self);
}
impl<T> OptionSink for Option<T> {
	fn sink(self) {
		let _ = self;
	}
}
impl<T, R> OptionSink for Result<T, R> {
	fn sink(self) {
		let _ = self;
	}
}

#[macro_export]
macro_rules! map_variant {
	($t: pat = $e: expr => $do: expr) => {{
		if let $t = $e {
			Some($do)
		} else {
			None
		}
	}};
}

mod def;
mod vec;
