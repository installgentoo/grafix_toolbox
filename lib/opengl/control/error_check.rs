#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! GLCheck {
	($v: expr) => {{
		unsafe { $v }
	}};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! GLCheck {
	($v: expr) => {{
		pub type CowStr = std::borrow::Cow<'static, str>;
		fn code_to_error(code: gl::types::GLenum) -> CowStr {
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

		let (val, err) = unsafe { ($v, gl::GetError()) };
		if err != gl::NO_ERROR {
			WARN!("OpenGL error {} in {}", code_to_error(err), stringify!($v));
		}
		val
	}};
}
