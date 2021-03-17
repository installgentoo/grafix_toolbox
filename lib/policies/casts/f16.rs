use super::cast::*;

impl Cast<bool> for f16 {
	fn to(v: bool) -> Self {
		Self::from_f32(v as u32 as f32)
	}
}
impl Cast<u8> for f16 {
	fn to(v: u8) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<i8> for f16 {
	fn to(v: i8) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<u16> for f16 {
	fn to(v: u16) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<i16> for f16 {
	fn to(v: i16) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<u32> for f16 {
	fn to(v: u32) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<i32> for f16 {
	fn to(v: i32) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<u64> for f16 {
	fn to(v: u64) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<i64> for f16 {
	fn to(v: i64) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<u128> for f16 {
	fn to(v: u128) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<i128> for f16 {
	fn to(v: i128) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<usize> for f16 {
	fn to(v: usize) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<isize> for f16 {
	fn to(v: isize) -> Self {
		Self::from_f32(v as f32)
	}
}
impl Cast<f16> for f16 {
	fn to(v: f16) -> Self {
		v
	}
}
impl Cast<f32> for f16 {
	fn to(v: f32) -> Self {
		Self::from_f32(v)
	}
}
impl Cast<f64> for f16 {
	fn to(v: f64) -> Self {
		Self::from_f64(v)
	}
}
