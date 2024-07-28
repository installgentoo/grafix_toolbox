use super::super::pre::*;

impl Cast<bool> for f32 {
	fn to(v: bool) -> Self {
		v as u32 as Self
	}
}
impl Cast<u8> for f32 {
	fn to(v: u8) -> Self {
		v as Self
	}
}
impl Cast<i8> for f32 {
	fn to(v: i8) -> Self {
		v as Self
	}
}
impl Cast<u16> for f32 {
	fn to(v: u16) -> Self {
		v as Self
	}
}
impl Cast<i16> for f32 {
	fn to(v: i16) -> Self {
		v as Self
	}
}
impl Cast<u32> for f32 {
	fn to(v: u32) -> Self {
		v as Self
	}
}
impl Cast<i32> for f32 {
	fn to(v: i32) -> Self {
		v as Self
	}
}
impl Cast<u64> for f32 {
	fn to(v: u64) -> Self {
		v as Self
	}
}
impl Cast<i64> for f32 {
	fn to(v: i64) -> Self {
		v as Self
	}
}
impl Cast<u128> for f32 {
	fn to(v: u128) -> Self {
		v as Self
	}
}
impl Cast<i128> for f32 {
	fn to(v: i128) -> Self {
		v as Self
	}
}
impl Cast<usize> for f32 {
	fn to(v: usize) -> Self {
		v as Self
	}
}
impl Cast<isize> for f32 {
	fn to(v: isize) -> Self {
		v as Self
	}
}
impl Cast<f16> for f32 {
	fn to(v: f16) -> Self {
		v.to_f32()
	}
}
impl Cast<f32> for f32 {
	fn to(v: f32) -> Self {
		v
	}
}
impl Cast<f64> for f32 {
	fn to(v: f64) -> Self {
		v as Self
	}
}
