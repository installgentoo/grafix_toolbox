pub mod ext {
	pub use super::{func::file as FS, func::rand, func::slicing, pre::logging, profiling, types::lazy, types::prefetch};
}

pub mod pre {
	pub use super::func::{chksum, ext::*, index::*, logging, n_iter::*, result::*, vec::*};
	pub use super::types::{cached::*, cached_str::*, ext::*, memoized::MemRes, memoized::Memoized};
	pub use super::{math::pre::*, traits::*};

	pub type STR = &'static str;
	pub type Str = Box<str>;
	pub type Astr = std::sync::Arc<str>;

	pub fn Box<T>(v: T) -> Box<T> {
		Box::new(v)
	}
	pub fn Cell<T>(v: T) -> std::cell::Cell<T> {
		std::cell::Cell::new(v)
	}
	pub fn Def<T: Default>() -> T {
		Default::default()
	}

	pub fn type_name<T: ?Sized>() -> String {
		let mut str = std::any::type_name::<T>()
			.split('<')
			.map(|s| [s.split("::").last().unwrap_or(""), "<"].concat())
			.collect::<String>();
		str.pop();
		str
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

#[macro_use]
mod traits;
#[macro_use]
mod func;
#[macro_use]
mod types;

pub mod math;
pub mod profiling;
pub mod task;
