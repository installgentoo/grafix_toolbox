#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! GL {
	($fun: expr) => {{
		#[allow(clippy::macro_metavars_in_unsafe, unused_unsafe)]
		unsafe {
			$fun
		}
	}};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! GL {
	($fun: expr) => {{
		ASSERT!(*$crate::GL::macro_uses::gl_was_initialized(), "Opengl wasn't initialized on this thread");

		fn code_to_error(code: gl::types::GLenum) -> String {
			match code {
				gl::INVALID_ENUM => "GL_INVALID_ENUM".into(),
				gl::INVALID_VALUE => "GL_INVALID_VALUE".into(),
				gl::INVALID_OPERATION => "GL_INVALID_OPERATION".into(),
				gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW".into(),
				gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW".into(),
				gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY".into(),
				gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION".into(),
				gl::CONTEXT_LOST => "GL_CONTEXT_LOST".into(),
				_ => format!("GL_?_{code}"),
			}
		}

		#[allow(clippy::macro_metavars_in_unsafe)]
		let (val, err) = unsafe { ($fun, gl::GetError()) };
		if err != gl::NO_ERROR {
			FAIL!("OpenGL error {} in {}", code_to_error(err), stringify!($fun));
		}
		val
	}};
}

pub mod universion;

#[macro_use]
pub mod consts;
#[macro_use]
pub mod funcs;

pub mod obj;
pub mod policy;
pub mod state;
pub mod tex_state;
pub mod uniform_state;

pub fn gl_was_initialized() -> &'static mut bool {
	LocalStatic!(bool, { false })
}

fn drop_in_gl(f: impl FnOnce() + SendS) {
	if *gl_was_initialized() {
		return GL!(f());
	}

	task::GLRuntime()
		.spawn(async || {
			task::sleep_ms(16).await;
			task::yield_now().await;
			GL!(f())
		})
		.detach();
}

use crate::lib::*;
