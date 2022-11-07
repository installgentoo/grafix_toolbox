use crate::uses::*;

pub trait TexSize: TrivialBound {
	const TYPE: GLenum;
	const SIZE: i32;
}
derive_common_VAL! { pub struct RED; }
impl TexSize for RED {
	const TYPE: GLenum = gl::RED;
	const SIZE: i32 = 1;
}
derive_common_VAL! { pub struct RG; }
impl TexSize for RG {
	const TYPE: GLenum = gl::RG;
	const SIZE: i32 = 2;
}
derive_common_VAL! { pub struct RGB; }
impl TexSize for RGB {
	const TYPE: GLenum = gl::RGB;
	const SIZE: i32 = 3;
}
derive_common_VAL! { pub struct RGBA; }
impl TexSize for RGBA {
	const TYPE: GLenum = gl::RGBA;
	const SIZE: i32 = 4;
}

pub trait TexFmt: TrivialBound {
	const TYPE: GLenum;
	const ZERO: Self;
}
impl TexFmt for i8 {
	const TYPE: GLenum = gl::BYTE;
	const ZERO: Self = 0;
}
impl TexFmt for u8 {
	const TYPE: GLenum = gl::UNSIGNED_BYTE;
	const ZERO: Self = 0;
}
impl TexFmt for i16 {
	const TYPE: GLenum = gl::SHORT;
	const ZERO: Self = 0;
}
impl TexFmt for u16 {
	const TYPE: GLenum = gl::UNSIGNED_SHORT;
	const ZERO: Self = 0;
}
impl TexFmt for i32 {
	const TYPE: GLenum = gl::INT;
	const ZERO: Self = 0;
}
impl TexFmt for u32 {
	const TYPE: GLenum = gl::UNSIGNED_INT;
	const ZERO: Self = 0;
}
impl TexFmt for f16 {
	const TYPE: GLenum = gl::HALF_FLOAT;
	const ZERO: Self = f16::from_bits(0);
}
impl TexFmt for f32 {
	const TYPE: GLenum = gl::FLOAT;
	const ZERO: Self = 0.;
}

pub fn get_internal_fmt<S: TexSize, F: TexFmt>() -> GLenum {
	match S::TYPE {
		gl::RED => match F::TYPE {
			gl::BYTE => gl::R8I,
			gl::UNSIGNED_BYTE => gl::R8UI,
			gl::SHORT => gl::R16I,
			gl::UNSIGNED_SHORT => gl::R16UI,
			gl::INT => gl::R32I,
			gl::UNSIGNED_INT => gl::R32UI,
			gl::HALF_FLOAT => gl::R16F,
			gl::FLOAT => gl::R32F,
			_ => unreachable!(),
		},
		gl::RG => match F::TYPE {
			gl::BYTE => gl::RG8I,
			gl::UNSIGNED_BYTE => gl::RG8UI,
			gl::SHORT => gl::RG16I,
			gl::UNSIGNED_SHORT => gl::RG16UI,
			gl::INT => gl::RG32I,
			gl::UNSIGNED_INT => gl::RG32UI,
			gl::HALF_FLOAT => gl::RG16F,
			gl::FLOAT => gl::RG32F,
			_ => unreachable!(),
		},
		gl::RGB => match F::TYPE {
			gl::BYTE => gl::RGB8I,
			gl::UNSIGNED_BYTE => gl::RGB8UI,
			gl::SHORT => gl::RGB16I,
			gl::UNSIGNED_SHORT => gl::RGB16UI,
			gl::INT => gl::RGB32I,
			gl::UNSIGNED_INT => gl::RGB32UI,
			gl::HALF_FLOAT => gl::RGB16F,
			gl::FLOAT => gl::RGB32F,
			_ => unreachable!(),
		},
		gl::RGBA => match F::TYPE {
			gl::BYTE => gl::RGBA8I,
			gl::UNSIGNED_BYTE => gl::RGBA8UI,
			gl::SHORT => gl::RGBA16I,
			gl::UNSIGNED_SHORT => gl::RGBA16UI,
			gl::INT => gl::RGBA32I,
			gl::UNSIGNED_INT => gl::RGBA32UI,
			gl::HALF_FLOAT => gl::RGBA16F,
			gl::FLOAT => gl::RGBA32F,
			_ => unreachable!(),
		},
		_ => unreachable!(),
	}
}

pub fn normalize_internal_fmt(fmt: GLenum) -> GLenum {
	match fmt {
		gl::R8I | gl::R8UI => gl::R8,
		gl::R16I | gl::R16UI => gl::R16,

		gl::RG8I | gl::RG8UI => gl::RG8,
		gl::RG16I | gl::RG16UI => gl::RG16,

		gl::RGB8I | gl::RGB8UI => gl::RGB8,
		gl::RGB16I | gl::RGB16UI => gl::RGB16,

		gl::RGBA8I | gl::RGBA8UI => gl::RGBA8,
		gl::RGBA16I | gl::RGBA16UI => gl::RGBA16,

		f => f,
	}
}
