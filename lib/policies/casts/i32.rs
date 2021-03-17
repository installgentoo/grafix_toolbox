use super::cast::*;

impl Cast<bool> for i32 {
	fn to(v: bool) -> Self {
		v as Self
	}
}
impl Cast<u8> for i32 {
	fn to(v: u8) -> Self {
		v as Self
	}
}
impl Cast<i8> for i32 {
	fn to(v: i8) -> Self {
		v as Self
	}
}
impl Cast<u16> for i32 {
	fn to(v: u16) -> Self {
		v as Self
	}
}
impl Cast<i16> for i32 {
	fn to(v: i16) -> Self {
		v as Self
	}
}
impl Cast<u32> for i32 {
	fn to(v: u32) -> Self {
		cast!(v, i32)
	}
}
impl Cast<i32> for i32 {
	fn to(v: i32) -> Self {
		v
	}
}
impl Cast<u64> for i32 {
	fn to(v: u64) -> Self {
		cast!(v, i32)
	}
}
impl Cast<i64> for i32 {
	fn to(v: i64) -> Self {
		cast!(v, i32)
	}
}
impl Cast<u128> for i32 {
	fn to(v: u128) -> Self {
		cast!(v, i32)
	}
}
impl Cast<i128> for i32 {
	fn to(v: i128) -> Self {
		cast!(v, i32)
	}
}
impl Cast<usize> for i32 {
	fn to(v: usize) -> Self {
		cast!(v, i32)
	}
}
impl Cast<isize> for i32 {
	fn to(v: isize) -> Self {
		cast!(v, i32)
	}
}
impl Cast<f16> for i32 {
	fn to(v: f16) -> Self {
		Self::to(v.to_f32())
	}
}
impl Cast<f32> for i32 {
	fn to(v: f32) -> Self {
		let check = |v: f32| v.trunc() >= i32::min_value() as f32 && v.trunc() <= i32::max_value() as f32;
		ASSERT!(check(v), "Error casting {} to i32", v);
		unsafe { v.to_int_unchecked() }
	}
}
impl Cast<f64> for i32 {
	fn to(v: f64) -> Self {
		let check = |v: f64| v.trunc() >= i32::min_value() as f64 && v.trunc() <= i32::max_value() as f64;
		ASSERT!(check(v), "Error casting {} to i32", v);
		unsafe { v.to_int_unchecked() }
	}
}
