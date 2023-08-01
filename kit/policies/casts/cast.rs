#[cfg(debug_assertions)]
macro_rules! cast {
	($v: expr, $t: ty) => {{
		<$t>::try_from($v).unwrap_or_else(|_| ASSERT!(false, "Error casting {} to {}", $v, stringify!($t)))
	}};
}

#[cfg(not(debug_assertions))]
macro_rules! cast {
	($v: ident, $t: ty) => {{
		$v as $t
	}};
}

pub use super::super::logging;
pub use crate::uses::f16;

pub trait Cast<T> {
	fn to(val: T) -> Self;
}
