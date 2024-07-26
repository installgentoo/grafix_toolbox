use super::{super::f16, *};

impl Cast<bool> for isize {
	fn to(v: bool) -> Self {
		v as Self
	}
}
impl Cast<u8> for isize {
	fn to(v: u8) -> Self {
		v as Self
	}
}
impl Cast<i8> for isize {
	fn to(v: i8) -> Self {
		v as Self
	}
}
impl Cast<u16> for isize {
	fn to(v: u16) -> Self {
		v as Self
	}
}
impl Cast<i16> for isize {
	fn to(v: i16) -> Self {
		v as Self
	}
}
impl Cast<u32> for isize {
	fn to(v: u32) -> Self {
		v as Self
	}
}
impl Cast<i32> for isize {
	fn to(v: i32) -> Self {
		v as Self
	}
}
impl Cast<u64> for isize {
	fn to(v: u64) -> Self {
		cast!(v, isize)
	}
}
impl Cast<i64> for isize {
	fn to(v: i64) -> Self {
		cast!(v, isize)
	}
}
impl Cast<u128> for isize {
	fn to(v: u128) -> Self {
		cast!(v, isize)
	}
}
impl Cast<i128> for isize {
	fn to(v: i128) -> Self {
		cast!(v, isize)
	}
}
impl Cast<usize> for isize {
	fn to(v: usize) -> Self {
		cast!(v, isize)
	}
}
impl Cast<isize> for isize {
	fn to(v: isize) -> Self {
		v
	}
}
impl Cast<f16> for isize {
	fn to(v: f16) -> Self {
		Self::to(v.to_f32())
	}
}
impl Cast<f32> for isize {
	fn to(v: f32) -> Self {
		ASSERT!(v.trunc() >= isize::MIN as f32 && v.trunc() <= isize::MAX as f32, "Error casting {v} to isize");
		unsafe { v.to_int_unchecked() }
	}
}
impl Cast<f64> for isize {
	fn to(v: f64) -> Self {
		ASSERT!(v.trunc() >= isize::MIN as f64 && v.trunc() <= isize::MAX as f64, "Error casting {v} to isize");
		unsafe { v.to_int_unchecked() }
	}
}
