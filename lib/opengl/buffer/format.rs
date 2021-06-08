use crate::GL::types::*;

pub trait IdxType {
	const TYPE: GLenum;
}
impl IdxType for u8 {
	const TYPE: GLenum = gl::UNSIGNED_BYTE;
}
impl IdxType for u16 {
	const TYPE: GLenum = gl::UNSIGNED_SHORT;
}
impl IdxType for u32 {
	const TYPE: GLenum = gl::UNSIGNED_INT;
}

pub trait AttrType {
	const TYPE: GLenum;
}
impl AttrType for i8 {
	const TYPE: GLenum = gl::BYTE;
}
impl AttrType for u8 {
	const TYPE: GLenum = gl::UNSIGNED_BYTE;
}
impl AttrType for i16 {
	const TYPE: GLenum = gl::SHORT;
}
impl AttrType for u16 {
	const TYPE: GLenum = gl::UNSIGNED_SHORT;
}
impl AttrType for i32 {
	const TYPE: GLenum = gl::INT;
}
impl AttrType for u32 {
	const TYPE: GLenum = gl::UNSIGNED_INT;
}
impl AttrType for f16 {
	const TYPE: GLenum = gl::HALF_FLOAT;
}
impl AttrType for f32 {
	const TYPE: GLenum = gl::FLOAT;
}
impl AttrType for f64 {
	const TYPE: GLenum = gl::DOUBLE;
}
