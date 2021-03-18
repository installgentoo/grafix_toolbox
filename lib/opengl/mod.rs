#[macro_use]
mod control;
mod buffer;
#[macro_use]
mod shader;
#[macro_use]
mod texture;

mod consts_def;
#[macro_use]
mod funcs_def;
mod screen;

mod debug;
mod utility;

use control::*;
use opengl::types;

pub mod opengl {
	pub use super::opengl;

	pub use super::bindless::*;

	pub use gl;
	pub use types::*;
	pub mod types {
		pub use gl::types::{GLbitfield, GLboolean as GLbool, GLenum, GLvoid};
		pub use half::f16;
	}

	pub use super::{consts_def::*, funcs_def::*};

	pub type Query = spec::Object<spec::Query>;

	pub use super::debug::{DebugLevel, EnableDebugContext};
	pub use super::utility::{Font, Screen};
	pub use atlas::{Animation, AtlasTex2d, TexAtlas, VTex2d};
	pub use bind::*;
	pub use buffer::*;
	pub use fbo::*;
	pub use sampler::*;
	pub use screen::*;
	pub use shader::*;
	pub use tex::*;
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
	pub mod sampler {
		pub use super::super::texture::{sampler_use, Sampler};
	}
	pub mod screen {
		pub use super::super::screen::{BindScreenFbo, ClearColor, ClearDepth, ClearScreen, Viewport};
	}
	pub mod shader {
		pub use super::super::shader::{shader_use, Shader, ShaderManager};
	}
	pub mod tex {
		pub type Tex2d<S, F> = Tex<GL_TEXTURE_2D, S, F>;
		pub type CubeTex<S, F> = Tex<GL_TEXTURE_CUBE_MAP, S, F>;
		use super::super::*;
		pub use texture::{chans::*, spec::*, Tex, TexBuffer, TexParam};
		pub use utility::{fImage, uImage, Image};
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
		pub use super::super::utility::Glyph;
	}
	pub mod pingpong {
		pub use super::super::utility::{PPDrawableArg, Slab};
	}
	pub mod pbrt {
		pub use super::super::utility::{EnvTex, Environment};
	}
	pub mod mesh {
		pub use super::super::utility::{AnyMesh, Camera, Mesh, Model, Skybox};
	}
}
