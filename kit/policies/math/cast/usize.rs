use super::{super::f16, *};

impl Cast<bool> for usize {
	fn to(v: bool) -> Self {
		v as Self
	}
}
impl Cast<u8> for usize {
	fn to(v: u8) -> Self {
		v as Self
	}
}
impl Cast<i8> for usize {
	fn to(v: i8) -> Self {
		cast!(v, usize)
	}
}
impl Cast<u16> for usize {
	fn to(v: u16) -> Self {
		v as Self
	}
}
impl Cast<i16> for usize {
	fn to(v: i16) -> Self {
		cast!(v, usize)
	}
}
impl Cast<u32> for usize {
	fn to(v: u32) -> Self {
		v as Self
	}
}
impl Cast<i32> for usize {
	fn to(v: i32) -> Self {
		cast!(v, usize)
	}
}
impl Cast<u64> for usize {
	fn to(v: u64) -> Self {
		cast!(v, usize)
	}
}
impl Cast<i64> for usize {
	fn to(v: i64) -> Self {
		cast!(v, usize)
	}
}
impl Cast<u128> for usize {
	fn to(v: u128) -> Self {
		cast!(v, usize)
	}
}
impl Cast<i128> for usize {
	fn to(v: i128) -> Self {
		cast!(v, usize)
	}
}
impl Cast<usize> for usize {
	fn to(v: usize) -> Self {
		v
	}
}
impl Cast<isize> for usize {
	fn to(v: isize) -> Self {
		cast!(v, usize)
	}
}
impl Cast<f16> for usize {
	fn to(v: f16) -> Self {
		Self::to(v.to_f32())
	}
}
impl Cast<f32> for usize {
	fn to(v: f32) -> Self {
		ASSERT!(
			v.trunc() >= usize::min_value() as f32 && v.trunc() <= usize::max_value() as f32,
			"Error casting {v} to usize"
		);
		unsafe { v.to_int_unchecked() }
	}
}
impl Cast<f64> for usize {
	fn to(v: f64) -> Self {
		ASSERT!(
			v.trunc() >= usize::min_value() as f64 && v.trunc() <= usize::max_value() as f64,
			"Error casting {v} to usize"
		);
		unsafe { v.to_int_unchecked() }
	}
}
