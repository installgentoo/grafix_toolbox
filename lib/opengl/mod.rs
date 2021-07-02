#[macro_use]
mod control;
#[macro_use]
mod funcs_def;
#[macro_use]
mod shader;

mod buffer;
mod consts_def;
mod debug;
mod fence;
mod screen;
mod texture;
mod utility;

use control::*;
use opengl::types;

pub mod opengl {
	// TODO negative traits stabilization - remove send/sync from Query, Framebuffer and Vao //impl !Send for //impl !Sync for
	pub use gl;
	pub type Query = spec::Object<spec::Query>;
	pub use super::debug::{DebugLevel, EnableDebugContext};
	pub use super::fence::Fence;
	pub use bind::*;
	pub use buffer::*;
	pub use fbo::*;
	pub use screen::*;
	pub use shader::*;
	pub use states::*;
	pub use tex::*;
	pub use types::*;
	pub mod macro_uses {
		pub use super::super::{error_check::gl_was_initialized, shader::uniforms::uniforms_use, shader::InlineShader, texture::sampler_use};
	}
	pub mod bind {
		use super::super::*;
		pub use buffer::{Mapping, MappingMut, VaoBinding};
		pub use shader::ShaderBinding;
		pub use texture::{TexBuffBinding, TextureBinding};
	}
	pub mod buffer {
		pub use super::super::buffer::{AttrArr, IdxArr, Vao};
	}
	pub mod fbo {
		use super::super::*;
		pub use texture::{Framebuffer, Renderbuffer};
		pub use utility::Fbo;
	}
	pub mod screen {
		pub use super::super::screen::{BindScreenFbo, ClearColor, ClearDepth, ClearScreen, Viewport};
	}
	pub mod shader {
		pub use super::super::shader::{Shader, ShaderManager};
	}
	pub mod states {
		pub use super::super::{consts_def::*, funcs_def::*};
	}
	pub mod tex {
		pub type Tex2d<S, F> = Tex<GL_TEXTURE_2D, S, F>;
		pub type CubeTex<S, F> = Tex<GL_TEXTURE_CUBE_MAP, S, F>;
		use super::super::*;
		pub use texture::{chans::*, spec::*, Sampler, Tex, TexBuffer, TexParam};
		pub use utility::{fImage, uImage, Image};
	}
	pub mod types {
		pub use gl::types::{GLbitfield, GLboolean as GLbool, GLenum, GLvoid};
		pub use half::f16;
	}
	pub mod spec {
		use super::super::*;
		pub use buffer::{AttrType, IdxType};
		pub use {object::*, policy::*, state::State};
	}
	pub mod atlas {
		pub use super::super::utility::{pack_into_atlas, Animation, AtlasTex2d, TexAtlas, Tile, VTex2d};
	}
	pub mod font {
		pub use super::super::utility::{Font, Glyph};
	}
	pub mod offhand {
		pub use super::super::utility::Offhand;
	}
	pub mod pingpong {
		pub use super::super::utility::{PPDrawableArg, Slab};
	}
	pub mod pbrt {
		pub use super::super::utility::{EnvTex, Environment};
	}
	pub mod mesh {
		pub use super::super::utility::{AnyMesh, Camera, Mesh, Model, Screen, Skybox};
	}
	pub mod unigl {
		pub use super::super::universion::*;
	}
}
