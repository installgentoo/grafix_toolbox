pub use crate::map_variant;
pub mod cached;
pub mod cached_str;
pub mod lazy;
pub mod n_iter;
pub mod prefetch;
pub use {def::*, vec::*};

pub type STR = &'static str;

pub fn Box<T>(v: T) -> Box<T> {
	Box::new(v)
}

pub fn Def<T: Default>() -> T {
	Default::default()
}

pub trait OptionSink {
	fn sink(self);
}
impl<T> OptionSink for Option<T> {
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
