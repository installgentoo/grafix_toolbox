use super::cast::*;

impl Cast<bool> for bool {
	fn to(v: bool) -> Self {
		v
	}
}
impl Cast<u8> for bool {
	fn to(v: u8) -> Self {
		v != 0
	}
}
impl Cast<i8> for bool {
	fn to(v: i8) -> Self {
		v != 0
	}
}
impl Cast<u16> for bool {
	fn to(v: u16) -> Self {
		v != 0
	}
}
impl Cast<i16> for bool {
	fn to(v: i16) -> Self {
		v != 0
	}
}
impl Cast<u32> for bool {
	fn to(v: u32) -> Self {
		v != 0
	}
}
impl Cast<i32> for bool {
	fn to(v: i32) -> Self {
		v != 0
	}
}
impl Cast<u64> for bool {
	fn to(v: u64) -> Self {
		v != 0
	}
}
impl Cast<i64> for bool {
	fn to(v: i64) -> Self {
		v != 0
	}
}
impl Cast<u128> for bool {
	fn to(v: u128) -> Self {
		v != 0
	}
}
impl Cast<i128> for bool {
	fn to(v: i128) -> Self {
		v != 0
	}
}
impl Cast<usize> for bool {
	fn to(v: usize) -> Self {
		v != 0
	}
}
impl Cast<isize> for bool {
	fn to(v: isize) -> Self {
		v != 0
	}
}
impl Cast<f16> for bool {
	fn to(v: f16) -> Self {
		v != f16::ZERO
	}
}
impl Cast<f32> for bool {
	fn to(v: f32) -> Self {
		v != 0.
	}
}
impl Cast<f64> for bool {
	fn to(v: f64) -> Self {
		v != 0.
	}
}
