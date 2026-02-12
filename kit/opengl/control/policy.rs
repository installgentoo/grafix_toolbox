use super::{state::*, tex_state::*, universion::*, *};

macro_rules! m_STATE {
	() => {
		fn bound_obj() -> &'static mut u32 {
			LocalStatic!(u32)
		}
		fn tracked_obj() -> &'static mut u32 {
			LocalStatic!(u32)
		}
	};
}

pub trait Buffer: TrivialBound + State {
	const TYPE: GLenum;
}

#[derive_as_trivial]
pub struct Attribute;
impl State for Attribute {
	m_STATE!();
}
impl Buffer for Attribute {
	const TYPE: GLenum = gl::ARRAY_BUFFER;
}

#[derive_as_trivial]
pub struct Index;
impl State for Index {
	m_STATE!();
}
impl Buffer for Index {
	const TYPE: GLenum = gl::ELEMENT_ARRAY_BUFFER;
}

pub trait ShdBuffType: Buffer {
	fn max_bindings() -> i32;
	fn max_size() -> usize;
}

#[derive_as_trivial]
pub struct Uniform;
impl State for Uniform {
	m_STATE!();
}
impl Buffer for Uniform {
	const TYPE: GLenum = gl::UNIFORM_BUFFER;
}

#[derive_as_trivial]
pub struct ShdStorage;
impl State for ShdStorage {
	m_STATE!();
}
impl Buffer for ShdStorage {
	const TYPE: GLenum = gl::SHADER_STORAGE_BUFFER;
}

macro_rules! impl_shd {
	($n: ident, $t: ident) => {
		#[derive(Debug)]
		pub struct $n;
		impl State for $n {
			m_STATE!();
			fn new(obj: &mut u32) {
				*obj = unsafe { gl::CreateShader(gl::$t) }
			}
			fn del(obj: u32) {
				drop_in_gl(move || unsafe { gl::DeleteShader(obj) });
			}
		}
	};
}
impl_shd!(ShdVertT, VERTEX_SHADER);
impl_shd!(ShdPixT, FRAGMENT_SHADER);
impl_shd!(ShdGeomT, GEOMETRY_SHADER);
impl_shd!(ShdCompT, COMPUTE_SHADER);
impl_shd!(ShdCtrlT, TESS_CONTROL_SHADER);
impl_shd!(ShdEvalT, TESS_EVALUATION_SHADER);

#[derive(Debug)]
pub struct ShaderT;
impl State for ShaderT {
	m_STATE!();
	fn bind(obj: u32) {
		unsafe { gl::UseProgram(obj) }
	}
	fn new(obj: &mut u32) {
		*obj = unsafe { gl::CreateProgram() }
	}
	fn del(obj: u32) {
		drop_in_gl(move || unsafe { gl::DeleteProgram(obj) });
	}
}

#[derive(Debug)]
pub struct VertArrT(Dummy<*const ()>);
impl State for VertArrT {
	m_STATE!();
	fn bind(obj: u32) {
		unsafe { gl::BindVertexArray(obj) }
	}
	fn new(obj: &mut u32) {
		glCreateVao(obj);
	}
	fn del(obj: u32) {
		GL!(gl::DeleteVertexArrays(1, &obj));
	}
}

#[derive(Debug)]
pub struct QueryT(Dummy<*const ()>);
impl State for QueryT {
	m_STATE!();
	fn new(obj: &mut u32) {
		unsafe { gl::GenQueries(1, obj) }
	}
	fn del(obj: u32) {
		GL!(gl::DeleteQueries(1, &obj));
	}
}

#[derive(Debug)]
pub struct FramebuffT(Dummy<*const ()>);
impl State for FramebuffT {
	m_STATE!();
	fn bind(obj: u32) {
		unsafe { gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, obj) }
	}
	fn new(obj: &mut u32) {
		glCreateFramebuff(obj);
	}
	fn del(obj: u32) {
		GL!(gl::DeleteFramebuffers(1, &obj));
	}
}

#[derive(Debug)]
pub struct RenderbuffT;
impl State for RenderbuffT {
	m_STATE!();
	fn new(obj: &mut u32) {
		glCreateRenderbuff(obj);
	}
	fn del(obj: u32) {
		drop_in_gl(move || unsafe { gl::DeleteRenderbuffers(1, &obj) });
	}
}

#[derive(Debug)]
pub struct SamplerT;
impl State for SamplerT {
	m_STATE!();
	fn new(obj: &mut u32) {
		unsafe { gl::GenSamplers(1, obj) }
	}
	fn del(obj: u32) {
		TexState::drop_samp(obj);
		drop_in_gl(move || unsafe { gl::DeleteSamplers(1, &obj) });
	}
}

#[derive(Debug)]
pub struct TextureT<T>(Dummy<T>);
impl<T: TexType> State for TextureT<T> {
	m_STATE!();
	fn new(obj: &mut u32) {
		glCreateTexture(T::TYPE, obj);
	}
	fn del(obj: u32) {
		TexState::drop_tex(obj);
		drop_in_gl(move || glDeleteTexture(&obj));
	}
}
pub trait TexType: TrivialBound {
	const TYPE: GLenum;
}
