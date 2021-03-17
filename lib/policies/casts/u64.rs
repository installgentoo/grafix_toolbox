use super::cast::*;

impl Cast<bool> for u64 {
	fn to(v: bool) -> Self {
		v as Self
	}
}
impl Cast<u8> for u64 {
	fn to(v: u8) -> Self {
		v as Self
	}
}
impl Cast<i8> for u64 {
	fn to(v: i8) -> Self {
		cast!(v, u64)
	}
}
impl Cast<u16> for u64 {
	fn to(v: u16) -> Self {
		v as Self
	}
}
impl Cast<i16> for u64 {
	fn to(v: i16) -> Self {
		cast!(v, u64)
	}
}
impl Cast<u32> for u64 {
	fn to(v: u32) -> Self {
		v as Self
	}
}
impl Cast<i32> for u64 {
	fn to(v: i32) -> Self {
		cast!(v, u64)
	}
}
impl Cast<u64> for u64 {
	fn to(v: u64) -> Self {
		v
	}
}
impl Cast<i64> for u64 {
	fn to(v: i64) -> Self {
		cast!(v, u64)
	}
}
impl Cast<u128> for u64 {
	fn to(v: u128) -> Self {
		cast!(v, u64)
	}
}
impl Cast<i128> for u64 {
	fn to(v: i128) -> Self {
		cast!(v, u64)
	}
}
impl Cast<usize> for u64 {
	fn to(v: usize) -> Self {
		cast!(v, u64)
	}
}
impl Cast<isize> for u64 {
	fn to(v: isize) -> Self {
		cast!(v, u64)
	}
}
impl Cast<f16> for u64 {
	fn to(v: f16) -> Self {
		Self::to(v.to_f32())
	}
}
impl Cast<f32> for u64 {
	fn to(v: f32) -> Self {
		let check = |v: f32| v.trunc() >= u64::min_value() as f32 && v.trunc() <= u64::max_value() as f32;
		ASSERT!(check(v), "Error casting {} to u64", v);
		unsafe { v.to_int_unchecked() }
	}
}
impl Cast<f64> for u64 {
	fn to(v: f64) -> Self {
		let check = |v: f64| v.trunc() >= u64::min_value() as f64 && v.trunc() <= u64::max_value() as f64;
		ASSERT!(check(v), "Error casting {} to u64", v);
		unsafe { v.to_int_unchecked() }
	}
}
