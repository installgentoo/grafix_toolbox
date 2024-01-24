use crate::lib::*;

macro_rules! impl_type {
	($t: ty, $for: ty, $f: ident) => {
		impl $t for $for {
			const TYPE: GLenum = gl::$f;
		}
	};
}

pub trait IdxType: TrivialBound {
	const TYPE: GLenum;
}
impl_type!(IdxType, u8, UNSIGNED_BYTE);
impl_type!(IdxType, u16, UNSIGNED_SHORT);
impl_type!(IdxType, u32, UNSIGNED_INT);

pub trait AttrType: TrivialBound {
	const TYPE: GLenum;
}
impl_type!(AttrType, i8, BYTE);
impl_type!(AttrType, u8, UNSIGNED_BYTE);
impl_type!(AttrType, i16, SHORT);
impl_type!(AttrType, u16, UNSIGNED_SHORT);
impl_type!(AttrType, i32, INT);
impl_type!(AttrType, u32, UNSIGNED_INT);
impl_type!(AttrType, f16, HALF_FLOAT);
impl_type!(AttrType, f32, FLOAT);
impl_type!(AttrType, f64, DOUBLE);
