use super::{GL::unigl::GLSL_VERSION, object::*, *};

pub fn parse_includes(files: Vec<Astr>) -> Res<String> {
	let includes = Ok(GLSL_VERSION.into())
		.pipe(iter::once)
		.chain(
			files
				.into_iter()
				.map(|name| (name.clone(), FS::Lazy::Text(name).pipe(Feed::new)))
				.collect_vec()
				.into_iter()
				.map(|(name, i)| {
					let i = i.take();
					[GLSL_VERSION, "void main(){}\n", &i]
						.concat()
						.pipe(CString::new)
						.res()
						.and_then(|i| ShaderObj::new("vs_vf", &i))
						.map_err(|e| {
							let e = slice((|c| c == '\n', &e));
							let e = format!("GLSL error in header {name:?}{e}");
							adjust_log(e, -2)
						})
						.map(|_| i)
				}),
		)
		.collect::<Result<Vec<_>, _>>()?
		.join("\n");

	[&includes, "void main(){}"]
		.concat()
		.pipe(CString::new)
		.res()
		.and_then(|i| ShaderObj::new("vs_vf", &i))
		.map_err(|e| {
			let e = slice((|c| c == '\n', &e));
			let e = format!("GLSL headers collision {e:?}");
			adjust_log(e, 0)
		})
		.map(|_| includes)
}

pub fn load(n: Astr, s: impl Stream<Item = String> + SendS) -> ShdFile {
	let mut first = true;
	s.map(move |s| {
		parse_shader_sources(&n, &s)
			.into_iter()
			.filter_map(|s| {
				s.map_err(|err| if first { FAIL!(err) } else { WARN!(err) })
					.map(|(name, src)| ShdSrc { name, src })
					.ok()
			})
			.collect_vec()
			.pipe(Some)
			.tap(|_| first = false)
	})
	.pipe(Feed::new)
}

pub fn parse_shader_sources(filename: &str, text: &str) -> Vec<Res<(Str, String)>> {
	let mut cur_line = text.find("//--GLOBAL:").map(|end| text[..end].lines().count()).unwrap_or(0);
	let body: Res<_> = (|| {
		if let Some(beg) = text.find("//--GLOBAL:") {
			let text = &text[beg + 12..];
			let end = text.find("//--").res()?;
			text.split_at(end)
		} else {
			text.find("//--").res()?;
			("", text)
		}
		.pipe(Ok)
	})();

	let (globals, mut body) = match body {
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
			))];
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
				ShaderObj::valid(n).explain_err(|| format!("Cannot parse shader at {filename:?}:{cur_line}"))?;
				if b.is_empty() {
					format!("Cannot parse shader {n:?} in {filename:?}:{cur_line}").pipe(Err)?
				}

				(n.replace(char::is_whitespace, ""), b)
			};
			let newlines = "\n".repeat(cur_line);
			cur_line += body.lines().count();
			let shader = [globals, &newlines, body].concat();
			(name, shader).pipe(Ok)
		})
		.map(|s| s.map(|(n, b)| (n.into(), b)))
		.collect()
}

pub fn print_shader_log(obj: u32) -> Str {
	let (f_shader, f_prog): (unsafe fn(_, _, _), unsafe fn(_, _, _, _)) = match GL!(gl::IsShader(obj)) {
		gl::TRUE => (gl::GetShaderiv, gl::GetShaderInfoLog),
		_ => (gl::GetProgramiv, gl::GetProgramInfoLog),
	};
	let max_len = 0_i32.tap(|l| GL!(f_shader(obj, gl::INFO_LOG_LENGTH, l)));

	let mut log = vec![0_u8; usize(max_len)];
	GL!(f_prog(obj, max_len, ptr::null_mut(), log.as_mut_ptr() as *mut i8));
	let l = log.pop();
	if l.is_none() || l.valid() != 0 {
		return "Error copying error log".into();
	}
	String::from_utf8_lossy(&log).into()
}

pub fn adjust_log(log: String, offset: i32) -> String {
	log.lines()
		.map(|l| {
			let (_, tail) = split(l, |c| c == ':');
			let (num, tail) = split(tail, |c| c == '(');
			num.trim_matches(':')
				.parse::<i32>()
				.map_or_else(|_| l.into(), |num| [&(num + offset).to_string(), tail].concat())
		})
		.rev()
		.collect_vec()
		.join("\n")
}

#[derive(Debug)]
pub struct ShdSrc {
	pub name: Str,
	pub src: String,
}
