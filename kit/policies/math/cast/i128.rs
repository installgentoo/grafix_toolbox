use super::{super::f16, *};

impl Cast<bool> for i128 {
	fn to(v: bool) -> Self {
		v as Self
	}
}
impl Cast<u8> for i128 {
	fn to(v: u8) -> Self {
		v as Self
	}
}
impl Cast<i8> for i128 {
	fn to(v: i8) -> Self {
		v as Self
	}
}
impl Cast<u16> for i128 {
	fn to(v: u16) -> Self {
		v as Self
	}
}
impl Cast<i16> for i128 {
	fn to(v: i16) -> Self {
		v as Self
	}
}
impl Cast<u32> for i128 {
	fn to(v: u32) -> Self {
		v as Self
	}
}
impl Cast<i32> for i128 {
	fn to(v: i32) -> Self {
		v as Self
	}
}
impl Cast<u64> for i128 {
	fn to(v: u64) -> Self {
		v as Self
	}
}
impl Cast<i64> for i128 {
	fn to(v: i64) -> Self {
		v as Self
	}
}
impl Cast<u128> for i128 {
	fn to(v: u128) -> Self {
		cast!(v, i128)
	}
}
impl Cast<i128> for i128 {
	fn to(v: i128) -> Self {
		v
	}
}
impl Cast<usize> for i128 {
	fn to(v: usize) -> Self {
		cast!(v, i128)
	}
}
impl Cast<isize> for i128 {
	fn to(v: isize) -> Self {
		cast!(v, i128)
	}
}
impl Cast<f16> for i128 {
	fn to(v: f16) -> Self {
		Self::to(v.to_f32())
	}
}
impl Cast<f32> for i128 {
	fn to(v: f32) -> Self {
		ASSERT!(v.trunc() >= i128::min_value() as f32 && v.trunc() <= i128::max_value() as f32, "Error casting {v} to i128");
		unsafe { v.to_int_unchecked() }
	}
}
impl Cast<f64> for i128 {
	fn to(v: f64) -> Self {
		ASSERT!(v.trunc() >= i128::min_value() as f64 && v.trunc() <= i128::max_value() as f64, "Error casting {v} to i128");
		unsafe { v.to_int_unchecked() }
	}
}
