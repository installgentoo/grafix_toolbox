#[cfg(debug_assertions)]
macro_rules! cast {
	($v: expr, $t: ty) => {{
		<$t>::try_from($v).unwrap_or_else(|_| ERROR!("Error casting {} to {}", $v, stringify!($t)))
	}};
}
#[cfg(not(debug_assertions))]
macro_rules! cast {
	($v: ident, $t: ty) => {{
		$v as $t
	}};
}

pub trait Cast<T> {
	fn to(val: T) -> Self;
}

pub mod func;

mod bool;
mod f16;
mod f32;
mod f64;
mod i128;
mod i16;
mod i32;
mod i64;
mod i8;
mod isize;
mod nalgebra;
mod tuples;
mod u128;
mod u16;
mod u32;
mod u64;
mod u8;
mod usize;
mod vector;

use super::super::logging;
