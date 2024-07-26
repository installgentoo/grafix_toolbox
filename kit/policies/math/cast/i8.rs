use super::{super::f16, *};

impl Cast<bool> for i8 {
	fn to(v: bool) -> Self {
		v as Self
	}
}
impl Cast<u8> for i8 {
	fn to(v: u8) -> Self {
		cast!(v, i8)
	}
}
impl Cast<i8> for i8 {
	fn to(v: i8) -> Self {
		v
	}
}
impl Cast<u16> for i8 {
	fn to(v: u16) -> Self {
		cast!(v, i8)
	}
}
impl Cast<i16> for i8 {
	fn to(v: i16) -> Self {
		cast!(v, i8)
	}
}
impl Cast<u32> for i8 {
	fn to(v: u32) -> Self {
		cast!(v, i8)
	}
}
impl Cast<i32> for i8 {
	fn to(v: i32) -> Self {
		cast!(v, i8)
	}
}
impl Cast<u64> for i8 {
	fn to(v: u64) -> Self {
		cast!(v, i8)
	}
}
impl Cast<i64> for i8 {
	fn to(v: i64) -> Self {
		cast!(v, i8)
	}
}
impl Cast<u128> for i8 {
	fn to(v: u128) -> Self {
		cast!(v, i8)
	}
}
impl Cast<i128> for i8 {
	fn to(v: i128) -> Self {
		cast!(v, i8)
	}
}
impl Cast<usize> for i8 {
	fn to(v: usize) -> Self {
		cast!(v, i8)
	}
}
impl Cast<isize> for i8 {
	fn to(v: isize) -> Self {
		cast!(v, i8)
	}
}
impl Cast<f16> for i8 {
	fn to(v: f16) -> Self {
		Self::to(v.to_f32())
	}
}
impl Cast<f32> for i8 {
	fn to(v: f32) -> Self {
		ASSERT!(v.trunc() >= i8::MIN as f32 && v.trunc() <= i8::MAX as f32, "Error casting {v} to i8");
		unsafe { v.to_int_unchecked() }
	}
}
impl Cast<f64> for i8 {
	fn to(v: f64) -> Self {
		ASSERT!(v.trunc() >= i8::MIN as f64 && v.trunc() <= i8::MAX as f64, "Error casting {v} to i8");
		unsafe { v.to_int_unchecked() }
	}
}
