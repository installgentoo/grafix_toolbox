use super::{state::*, tex_state::*, universion::*};
use crate::uses::*;

macro_rules! m_STATE {
	() => {
		fn bound_obj() -> &'static mut u32 {
			static mut STATE: u32 = 0;
			unsafe { &mut STATE }
		}
		fn tracked_obj() -> &'static mut u32 {
			static mut STATE: u32 = 0;
			unsafe { &mut STATE }
		}
	};
}

pub trait Buffer: TrivialBound + State {
	const TYPE: GLenum;
}

derive_common_VAL! { pub struct Attribute; }
impl State for Attribute {
	m_STATE!();
}
impl Buffer for Attribute {
	const TYPE: GLenum = gl::ARRAY_BUFFER;
}

derive_common_VAL! { pub struct Index; }
impl State for Index {
	m_STATE!();
}
impl Buffer for Index {
	const TYPE: GLenum = gl::ELEMENT_ARRAY_BUFFER;
}

#[derive(Default)]
pub struct ShdProg;
impl State for ShdProg {
	m_STATE!();
	unsafe fn bind(obj: u32) {
		gl::UseProgram(obj);
	}
	unsafe fn gen(obj: &mut u32) {
		*obj = gl::CreateProgram();
	}
	unsafe fn del(obj: &mut u32) {
		gl::DeleteProgram(*obj);
	}
}

#[derive(Default)]
pub struct VertArrObj;
impl State for VertArrObj {
	m_STATE!();
	unsafe fn bind(obj: u32) {
		gl::BindVertexArray(obj);
	}
	unsafe fn gen(obj: &mut u32) {
		glCreateVao(obj);
	}
	unsafe fn del(obj: &mut u32) {
		gl::DeleteVertexArrays(1, obj);
	}
}

#[derive(Default)]
pub struct Query;
impl State for Query {
	m_STATE!();
	unsafe fn gen(obj: &mut u32) {
		gl::GenQueries(1, obj);
	}
	unsafe fn del(obj: &mut u32) {
		gl::DeleteQueries(1, obj);
	}
}

#[derive(Default)]
pub struct Framebuff;
impl State for Framebuff {
	m_STATE!();
	unsafe fn bind(obj: u32) {
		gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, obj);
	}
	unsafe fn gen(obj: &mut u32) {
		glCreateFramebuff(obj);
	}
	unsafe fn del(obj: &mut u32) {
		gl::DeleteFramebuffers(1, obj);
	}
}

#[derive(Default)]
pub struct Renderbuff;
impl State for Renderbuff {
	m_STATE!();
	unsafe fn gen(obj: &mut u32) {
		glCreateRenderbuff(obj);
	}
	unsafe fn del(obj: &mut u32) {
		gl::DeleteRenderbuffers(1, obj);
	}
}

#[derive(Default)]
pub struct SamplObj;
impl State for SamplObj {
	m_STATE!();
	unsafe fn gen(obj: &mut u32) {
		gl::GenSamplers(1, obj);
	}
	unsafe fn del(obj: &mut u32) {
		TexState::drop_samp(*obj);
		gl::DeleteSamplers(1, obj);
	}
}

#[derive(Debug, Default)]
pub struct Texture<T> {
	t: Dummy<T>,
}
impl<T: TexType> State for Texture<T> {
	m_STATE!();
	unsafe fn gen(obj: &mut u32) {
		glCreateTexture(T::TYPE, obj);
	}
	unsafe fn del(obj: &mut u32) {
		TexState::drop_tex(*obj);
		glDeleteTexture(obj);
	}
}

pub trait TexType: TrivialBound {
	const TYPE: GLenum;
}
