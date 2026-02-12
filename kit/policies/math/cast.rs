pub use half::f16;
pub mod matrix;

pub trait Cast<T> {
	fn to(val: T) -> Self;
}

macro_rules! to_int {
	($t: ty, $($f: ty),+) => {
		$(impl Cast<$f> for $t {
			#[inline(always)]
			fn to(v: $f) -> Self {
				#[cfg(debug_assertions)]
				{
					<$t>::try_from(v).unwrap_or_else(|_| ERROR!("Error casting {v} to {}", stringify!($t)))
				}
				#[cfg(not(debug_assertions))]
				{
					v as $t
				}
			}
		})+
	}
}
macro_rules! to_float {
	($t: ty, $($f: ty),+) => {
		$(impl Cast<$f> for $t {
			#[inline(always)]
			fn to(v: $f) -> Self {
				ASSERT!(
					v.trunc() >= Self::MIN as $f && v.trunc() <= Self::MAX as $f,
					"Error casting {v} to {}", super::super::pre::type_name::<Self>()
				);
				unsafe { v.to_int_unchecked() }
			}
		})+
	}
}
macro_rules! impl_self {
	($t: ty, $($f: ty),+) => {
		$(impl Cast<$f> for $t {
			#[inline(always)]
			fn to(v: $f) -> Self {
				v as Self
			}
		})+
	}
}
macro_rules! impl_to_half {
	($($t: ty),+) => {
		$(impl Cast<$t> for f16 {
			#[inline(always)]
			fn to(v: $t) -> f16 {
				f16::from_f32(v as f32)
			}
		})+
	}
}
macro_rules! impl_from_half {
	($($t: ty),+) => {
		$(impl Cast<f16> for $t {
			#[inline(always)]
			fn to(v: f16) -> Self {
				Self::to(v.to_f32())
			}
		})+
	}
}

macro_rules! impl_cast {
	($($t: ty),+) => {
		$(
			to_int!($t, bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
			to_float!($t, f32, f64);
			impl_from_half!($t);
			impl_to_half!($t);
			impl_self!(f32, $t);
			impl_self!(f64, $t);
		)+
	}
}
impl_cast!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_self!(f32, f32, f64);
impl_self!(f64, f32, f64);
impl_self!(f16, f16);
impl_from_half!(f32);
impl_to_half!(f32);

impl Cast<f16> for f64 {
	#[inline(always)]
	fn to(v: f16) -> Self {
		v.to_f64()
	}
}
impl Cast<f64> for f16 {
	#[inline(always)]
	fn to(v: f64) -> Self {
		Self::from_f64(v)
	}
}

impl Cast<bool> for f16 {
	#[inline(always)]
	fn to(v: bool) -> Self {
		Self::from_f32(v as u32 as f32)
	}
}
impl Cast<bool> for f32 {
	#[inline(always)]
	fn to(v: bool) -> Self {
		v as u32 as Self
	}
}
impl Cast<bool> for f64 {
	#[inline(always)]
	fn to(v: bool) -> Self {
		v as u32 as Self
	}
}

macro_rules! from_ref {
	($t: ty, $($f: ty),+) => {
		$(impl Cast<&$f> for $t {
			#[inline(always)]
			fn to(v: &$f) -> Self {
				Self::to(*v)
			}
		}
		impl Cast<&mut $f> for $t {
			#[inline(always)]
			fn to(v: &mut $f) -> Self {
				Self::to(*v)
			}
		})+
	}
}
macro_rules! impl_cast_ref {
	($($t: ty),+) => {
		$(from_ref!($t, bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64, usize, isize);)+
	}
}
impl_cast_ref!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64, usize, isize);

mod nalgebra;
mod tuples;
