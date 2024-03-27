#![allow(private_bounds)]
pub use {sampler::*, texbuff::*, texture::*};
pub mod chans {
	pub use super::format::{RED, RG, RGB, RGBA};
	pub type GREEN = RG;
	pub type BLUE = RGB;
	pub type ALPHA = RGBA;
}
pub mod spec {
	use super::*;
	pub type Tex2d<S, F> = Tex<S, F, GL_TEXTURE_2D>;
	pub type CubeTex<S, F> = Tex<S, F, GL_TEXTURE_CUBE_MAP>;
	pub use format::{TexFmt, TexSize};

	derive_common_VAL! { pub struct GL_TEXTURE_1D; }
	impl TexType for GL_TEXTURE_1D {
		const TYPE: GLenum = gl::TEXTURE_1D;
	}
	derive_common_VAL! { pub struct GL_TEXTURE_2D; }
	impl TexType for GL_TEXTURE_2D {
		const TYPE: GLenum = gl::TEXTURE_2D;
	}
	derive_common_VAL! { pub struct GL_TEXTURE_3D; }
	impl TexType for GL_TEXTURE_3D {
		const TYPE: GLenum = gl::TEXTURE_3D;
	}
	derive_common_VAL! { pub struct GL_TEXTURE_1D_ARRAY; }
	impl TexType for GL_TEXTURE_1D_ARRAY {
		const TYPE: GLenum = gl::TEXTURE_1D_ARRAY;
	}
	derive_common_VAL! { pub struct GL_TEXTURE_2D_ARRAY; }
	impl TexType for GL_TEXTURE_2D_ARRAY {
		const TYPE: GLenum = gl::TEXTURE_2D_ARRAY;
	}
	derive_common_VAL! { pub struct GL_TEXTURE_CUBE_MAP; }
	impl TexType for GL_TEXTURE_CUBE_MAP {
		const TYPE: GLenum = gl::TEXTURE_CUBE_MAP;
	}
	derive_common_VAL! { pub struct GL_TEXTURE_CUBE_MAP_ARRAY; }
	impl TexType for GL_TEXTURE_CUBE_MAP_ARRAY {
		const TYPE: GLenum = gl::TEXTURE_CUBE_MAP_ARRAY;
	}
	derive_common_VAL! { pub struct GL_TEXTURE_BUFFER; }
	impl TexType for GL_TEXTURE_BUFFER {
		const TYPE: GLenum = gl::TEXTURE_BUFFER;
	}
	derive_common_VAL! { pub struct GL_TEXTURE_2D_MULTISAMPLE; }
	impl TexType for GL_TEXTURE_2D_MULTISAMPLE {
		const TYPE: GLenum = gl::TEXTURE_2D_MULTISAMPLE;
	}
	derive_common_VAL! { pub struct GL_TEXTURE_2D_MULTISAMPLE_ARRAY; }
	impl TexType for GL_TEXTURE_2D_MULTISAMPLE_ARRAY {
		const TYPE: GLenum = gl::TEXTURE_2D_MULTISAMPLE_ARRAY;
	}
}

mod args;
mod format;
mod sampler;
mod texbuff;
mod texture;

use {super::internal::*, crate::lib::*, crate::GL};
