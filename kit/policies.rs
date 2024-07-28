pub mod pre {
	pub use super::func::{chksum, ext::*, index::*, logging, n_iter::*, result::*, vec::*};
	pub use super::types::{cached::*, cached_str::*, ext::*, memoized::*, pointer::*};
	pub use super::{math::pre::*, traits::*, type_name};
}

pub mod ext {
	pub use super::{func::file as FS, func::rand, func::slicing, pre::logging, profiling, types::lazy, types::prefetch};
}

#[macro_use]
pub mod traits;
#[macro_use]
pub mod func;
#[macro_use]
pub mod types;

pub mod math;
pub mod profiling;

pub fn type_name<T: ?Sized>() -> String {
	let mut str = std::any::type_name::<T>()
		.split('<')
		.map(|s| [s.split("::").last().unwrap_or(""), "<"].concat())
		.collect::<String>();
	str.pop();
	str
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
