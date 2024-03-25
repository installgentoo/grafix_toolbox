use crate::lib::*;

pub enum DebugLevel {
	High = 0,
	Medium = 1,
	Low = 2,
	All = 3,
}

pub fn EnableDebugContext(level: DebugLevel) {
	static mut LEVEL: u32 = 0;
	unsafe {
		LEVEL = level as u32;
	}
	GLEnable!(DEBUG_OUTPUT, DEBUG_OUTPUT_SYNCHRONOUS);
	GLCheck!(gl::DebugMessageCallback(Some(debug_gl_printer), ptr::addr_of!(LEVEL) as *const GLvoid));
}

extern "system" fn debug_gl_printer(src: GLenum, typ: GLenum, id: u32, lvl: GLenum, _: i32, msg: *const i8, filter: *mut GLvoid) {
	let f = unsafe { *(filter as *mut u32) };
	let lvl = match lvl {
		gl::DEBUG_SEVERITY_HIGH => "HIG".into(),
		gl::DEBUG_SEVERITY_MEDIUM => {
			if f > 0 {
				return;
			}
			"MED".into()
		}
		gl::DEBUG_SEVERITY_LOW => {
			if f > 1 {
				return;
			}
			"LOW".into()
		}
		gl::DEBUG_SEVERITY_NOTIFICATION => {
			if f > 2 {
				return;
			}
			"TIP".into()
		}
		_ => format!("SEVERITY_?_{lvl}"),
	};

	let src = match src {
		gl::DEBUG_SOURCE_API => "SOURCE_API".into(),
		gl::DEBUG_SOURCE_WINDOW_SYSTEM => "SOURCE_WINDOW_SYSTEM".into(),
		gl::DEBUG_SOURCE_SHADER_COMPILER => "SOURCE_SHADER_COMPILER".into(),
		gl::DEBUG_SOURCE_THIRD_PARTY => "SOURCE_THIRD_PARTY".into(),
		gl::DEBUG_SOURCE_APPLICATION => "SOURCE_APPLICATION".into(),
		gl::DEBUG_SOURCE_OTHER => "SOURCE_OTHER".into(),
		_ => format!("SOURCE_?_{src}"),
	};

	let typ = match typ {
		gl::DEBUG_TYPE_ERROR => "TYPE_ERROR".into(),
		gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "TYPE_DEPRECATED_BEHAVIOR".into(),
		gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => "TYPE_UNDEFINED_BEHAVIOR".into(),
		gl::DEBUG_TYPE_PORTABILITY => "TYPE_PORTABILITY".into(),
		gl::DEBUG_TYPE_PERFORMANCE => "TYPE_PERFORMANCE".into(),
		gl::DEBUG_TYPE_MARKER => "TYPE_MARKER".into(),
		gl::DEBUG_TYPE_PUSH_GROUP => "TYPE_PUSH_GROUP".into(),
		gl::DEBUG_TYPE_POP_GROUP => "TYPE_POP_GROUP".into(),
		gl::DEBUG_TYPE_OTHER => "TYPE_OTHER".into(),
		_ => format!("TYPE_?_{typ}"),
	};

	WARN!("GLDBG_{id}, {lvl}: {typ} {src} {:?}", unsafe { std::ffi::CStr::from_ptr(msg) });
}
