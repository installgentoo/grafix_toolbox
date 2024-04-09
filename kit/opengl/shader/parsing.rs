use super::{object::*, *};

pub fn load(n: Astr, s: impl Stream<Item = String> + SendStat) -> Lazy<Vec<ShdSrc>> {
	let mut first = true;
	Lazy::new(s.map({
		move |s| {
			let r = parse_shader_sources(&n, &s)
				.into_iter()
				.filter_map(|s| {
					match s {
						Ok((name, src)) => return Some(ShdSrc { name, src }),
						Err(e) if first => FAIL!(e),
						Err(e) => WARN!(e),
					}
					None
				})
				.collect();
			first = false;
			r
		}
	}))
}

pub fn parse_shader_sources(filename: &str, text: &str) -> Vec<Res<(Str, CString)>> {
	let mut cur_row_number = text.find("//--GLOBAL:").map(|end| text[..end].lines().count()).unwrap_or(0);
	let body: Res<_> = (|| {
		Ok(if let Some(beg) = text.find("//--GLOBAL:") {
			let text = &text[beg + 12..];
			let end = Res(text.find("//--"))?;
			(&text[..end], &text[end..])
		} else {
			Res(text.find("//--"))?;
			("", text)
		})
	})();

	let (header, mut body) = match body {
		Ok(t) => t,
		Err(_) => {
			return vec![Err(format!(
				"
			Shader file {filename:?} should be structured as follows:
			//--GLOBAL:
			Code to include in every shader in the file
			//-- (ps|vs|gs|cs)_shader_name
			Shader code

			Do not include glsl '#version ...' header, it will be generated"
			))]
		}
	};

	let mut end = body.find("//--");
	let parse_shaders = iter::from_fn(|| {
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
				let n = n.trim_end_matches('-');
				ShaderObj::valid(n).map_err(|e| format!("Failed to parse shader in {filename:?}:{cur_row_number}\n{e}"))?;
				if b.is_empty() {
					Err(format!("Failed to parse shader {n:?} in {filename:?}:{cur_row_number}"))?
				};

				let o = 1.or_def(cur_row_number == 0 && header.is_empty()); // skip newline to compensate for version line
				(n.replace(char::is_whitespace, ""), slice((o, b)))
			};
			let newlines = "\n".repeat(cur_row_number);
			cur_row_number += body.lines().count();
			let shader = [GL::unigl::GLSL_VERSION, header, &newlines, body].concat();
			let shader = CString::new(shader).map_err(|e| format!("Malformed shader {name:?} in {filename:?}:{cur_row_number}'\n{e}"))?;
			Ok((name, shader))
		})
		.map(|s| s.map(|(n, b)| (n.into(), b)))
		.collect()
}

pub fn print_shader_log(obj: u32) -> Str {
	let (f_shader, f_prog): (unsafe fn(_, _, _), unsafe fn(_, _, _, _)) = match GLCheck!(gl::IsShader(obj)) {
		gl::TRUE => (gl::GetShaderiv, gl::GetShaderInfoLog),
		_ => (gl::GetProgramiv, gl::GetProgramInfoLog),
	};

	let mut max_len: i32 = 0;
	GLCheck!(f_shader(obj, gl::INFO_LOG_LENGTH, &mut max_len));
	let log = {
		let mut log: Vec<u8> = vec![0; usize(max_len)];
		GLCheck!(f_prog(obj, max_len, ptr::null_mut(), log.as_mut_ptr() as *mut i8));
		let l = log.pop();
		if l.is_none() || l.valid() != 0 {
			return "Error copying error log".into();
		}
		log
	};

	String::from_utf8_lossy(&log).into()
}

pub struct ShdSrc {
	pub name: Str,
	pub src: CString,
}
