#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! GLCheck {
	($fun: expr) => {{
		unsafe { $fun }
	}};
}

pub fn gl_was_initialized(set: bool) -> bool {
	*UnsafeLocal!(bool, { set })
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! GLCheck {
	($fun: expr) => {{
		ASSERT!(crate::uses::GL::macro_uses::gl_was_initialized(false), "Opengl wasn't initialized on this thread");

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
				_ => CONCAT!("GL_?_", &code.to_string()).into(),
			}
		}

		let (val, err) = unsafe { ($fun, gl::GetError()) };
		if err != gl::NO_ERROR {
			WARN!("OpenGL error {} in {}", code_to_error(err), stringify!($fun));
		}
		val
	}};
}
