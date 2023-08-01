use crate::uses::*;

pub trait TexSize: TrivialBound {
	const TYPE: GLenum;
	const SIZE: i32;
}
macro_rules! impl_size {
	($t: ident, $s: literal) => {
		derive_common_VAL! { pub struct $t; }
		impl TexSize for $t {
			const TYPE: GLenum = gl::$t;
			const SIZE: i32 = $s;
		}
	};
}
impl_size!(RED, 1);
impl_size!(RG, 2);
impl_size!(RGB, 3);
impl_size!(RGBA, 4);

pub trait TexFmt: TrivialBound {
	const TYPE: GLenum;
}
macro_rules! impl_fmt {
	($t: ty, $g: ident) => {
		impl TexFmt for $t {
			const TYPE: GLenum = gl::$g;
		}
	};
}
impl_fmt!(i8, BYTE);
impl_fmt!(u8, UNSIGNED_BYTE);
impl_fmt!(i16, SHORT);
impl_fmt!(u16, UNSIGNED_SHORT);
impl_fmt!(i32, INT);
impl_fmt!(u32, UNSIGNED_INT);
impl_fmt!(f16, HALF_FLOAT);
impl_fmt!(f32, FLOAT);

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
