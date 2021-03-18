use super::args::*;
use crate::uses::{slicing::*, *};

pub fn parse_shader_sources(filename: &str, text: &str) -> SourcePack {
	let mut cur_row_number = 0;
	let parsed: Res<_> = (|| {
		cur_row_number = {
			let end = PASS!(text.find("//--GLOBAL:"));
			text[..end].lines().count()
		};
		let (header, mut body) = {
			if let Some(beg) = text.find("//--GLOBAL:") {
				let text = &text[beg + 11..];
				let end = PASS!(text.find("//--"));
				(&text[..end], &text[end..])
			} else {
				("", text)
			}
		};

		let mut end = body.find("//--");
		let parse_shaders = iter::from_fn(move || {
			end.map(|beg| {
				body = &body[4 + beg..];
				end = body.find("//--");
				let end = end.unwrap_or(body.len());
				&body[..end]
			})
		});

		parse_shaders
			.map(|body| {
				let (name, body) = {
					let body = slice((char::is_whitespace, body));
					let body = slice((|c: char| c.is_ascii_alphanumeric(), body));
					let (n, b) = split(body, char::is_control);
					if n.is_empty() || b.is_empty() {
						Err("Failed to parse shader name")
					} else {
						Ok((n.replace(char::is_whitespace, ""), b))
					}
				}?;
				let newlines = "\n".repeat(cur_row_number);
				cur_row_number += body.lines().count();
				let shader = PASS!(std::ffi::CString::new(CONCAT![header, &newlines, body]));
				Ok((name, shader))
			})
			.collect::<Res<_>>()
	})();

	OR_DEF!(parsed, "Malformed .glsl file {}, row {}, '{:?}'", filename, cur_row_number)
}

pub fn print_shader_log(obj: u32) -> String {
	let (f_shader, f_prog): (unsafe fn(_, _, _), unsafe fn(_, _, _, _)) = match GLCheck!(gl::IsShader(obj)) {
		gl::TRUE => (gl::GetShaderiv, gl::GetShaderInfoLog),
		_ => (gl::GetProgramiv, gl::GetProgramInfoLog),
	};

	let mut max_len: i32 = 0;
	GLCheck!(f_shader(obj, gl::INFO_LOG_LENGTH, &mut max_len));
	let log = {
		let mut log: Vec<u8> = vec![0; usize::to(max_len)];
		GLCheck!(f_prog(obj, max_len, ptr::null_mut(), log.as_mut_ptr() as *mut i8));
		let l = log.pop();
		if l.is_none() || l.unwrap() != 0 {
			return "Error copying error log".into();
		}
		log
	};

	String::from_utf8_lossy(&log).into()
}
