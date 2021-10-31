#[cfg(debug_assertions)]
macro_rules! cast {
	($v: expr, $t: ident) => {{
		$t::try_from($v).unwrap_or_else(|_| ASSERT!(false, "Error casting {} to {}", $v, stringify!($t)))
	}};
}

#[cfg(not(debug_assertions))]
macro_rules! cast {
	($v: expr, $t: ident) => {{
		$v as $t
	}};
}

pub use super::super::logging;
pub use crate::uses::f16;

pub trait Cast<T> {
	fn to(val: T) -> Self;
}
