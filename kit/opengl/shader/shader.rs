use super::{args::*, object::*, parsing::*, policy::*, state::*, types::*, uniform::*};
use crate::uses::{asyn::*, *};
use std::ffi::CString;

#[macro_export]
macro_rules! SHADER {
	($n: ident, $($body: expr),+) => {
		#[allow(non_upper_case_globals)]
		pub const $n: $crate::uses::GL::macro_uses::InlineShader = $crate::uses::GL::macro_uses::InlineShader(stringify!($n), const_format::concatcp!($crate::uses::GL::unigl::GLSL_VERSION, $($body,)+));
	};
}

pub struct Shader {
	program: Object<ShdProg>,
	name: Box<str>,
	uniforms: HashMap<u32, i32>,
	tex_cache: HashMap<i32, i32>,
}
impl Shader {
	pub fn pure<'a>(args: impl PureShaderArgs<'a>) -> Self {
		Self::new(args).unwrap()
	}
	pub fn new<'a>(args: impl ShaderArgs<'a>) -> Res<Self> {
		let (program, name) = ShaderManager::compile(args.get())?;

		Ok(Self {
			program,
			name,
			uniforms: Def(),
			tex_cache: Def(),
		})
	}
	pub fn Bind(&mut self) -> ShaderBinding {
		ShaderBinding::new(self)
	}
}
pub struct ShaderBinding<'l> {
	shd: &'l mut Shader,
}
impl<'l> ShaderBinding<'l> {
	pub fn new(o: &'l mut Shader) -> Self {
		ShdProg::Lock(o.program.obj);
		ShdProg::Bind(o.program.obj);
		Self { shd: o }
	} //TODO uniform blocks
	pub fn Uniform(&mut self, (id, name): (u32, &str), args: impl UniformArgs) {
		ASSERT!(crate::uses::GL::macro_uses::uniforms_use::id(name).0 == id, "Use Uniforms!() macro to set uniforms");
		let addr = if let Some(found) = self.shd.uniforms.get(&id) {
			let _collision_map = LocalStatic!(HashMap<u32, String>, { Def() });
			ASSERT!(_collision_map.entry(id).or_insert(name.into()) == name, "Unifrom collision at entry {name}");
			*found
		} else {
			let c_name = match CString::new(name) {
				Ok(str) => str,
				Err(e) => {
					FAIL!(e);
					return;
				}
			};
			let addr = GLCheck!(gl::GetUniformLocation(self.shd.program.obj, c_name.as_ptr()));
			if addr == -1 {
				INFO!("No uniform {name:?} in shader {:?}, or uniform was optimized out", self.shd.name);
			}
			self.shd.uniforms.insert(id, addr);
			addr
		};

		args.get(addr, &mut self.shd.tex_cache);
	}
}
impl Drop for ShaderBinding<'_> {
	fn drop(&mut self) {
		ShdProg::Unlock();
	}
}

enum ShaderType {
	vertex = 0,
	fragment = 1,
	geometry = 2,
}
const SHD_DEFS: [(GLenum, &str); 3] = [(gl::VERTEX_SHADER, "vertex"), (gl::FRAGMENT_SHADER, "pixel"), (gl::GEOMETRY_SHADER, "geometry")];

#[derive(Default)]
pub struct ShaderManager {
	objects: HashMap<Box<str>, u32>,
	sources: HashMap<Box<str>, CString>,
	loading: Vec<Task<SourcePack>>,
}
impl ShaderManager {
	pub fn LoadSources(filename: impl ToString) {
		let m = Self::get();
		let name = filename.to_string();
		m.loading.push(task::spawn(async move {
			let data = OR_DEFAULT!(FS::read_text(&name).await);
			parse_shader_sources(&name, &data)
		}));
	}
	pub fn ForceSource(name: impl ToString, source: impl ToString) {
		let name = name.to_string().into();
		let m = Self::get();
		m.objects.remove(&name).map(|o| GLCheck!(gl::DeleteShader(o)));
		m.sources.insert(name, CString::new(source.to_string()).unwrap());
	}
	pub fn ClearSources() {
		let m = Self::get();
		m.objects.iter().for_each(|(_, &o)| GLCheck!(gl::DeleteShader(o)));
		m.objects.clear();
	}

	fn compile((vert, geom, pix): CompileArgs) -> Res<(Object<ShdProg>, Box<str>)> {
		let m = Self::get();
		let get_object = |(name, typ): (C<'_>, _)| {
			let Self { objects, sources, loading } = m;
			if let Some(found) = objects.get(&*name) {
				return Ok(*found);
			}

			task::block_on(async {
				stream::iter(loading.drain(..))
					.then(|t| t)
					.for_each(|p| {
						p.into_iter()
							.for_each(|(name, body)| sources.insert(name.clone(), body).map(|_| WARN!("Shader source {name:?} was already loaded")).sink())
					})
					.await
			});

			let (name, (typ, type_name)) = (name.into_owned().into_boxed_str(), SHD_DEFS[typ as usize]);
			let source = Res(sources.get(&name)).map_err(|_| format!("No {type_name} shader {name:?} in loaded sources"))?;

			let obj = GLCheck!(gl::CreateShader(typ));
			ASSERT!(obj != 0, "Failed to create {type_name} shader object {name:?}");
			GLCheck!(gl::ShaderSource(obj, 1, &source.as_ptr(), ptr::null()));
			GLCheck!(gl::CompileShader(obj));
			let mut status: i32 = 0;
			GLCheck!(gl::GetShaderiv(obj, gl::COMPILE_STATUS, &mut status));
			if GLbool::to(status) != gl::TRUE {
				let err = format!("Error compiling {type_name} shader {name:?}\n{}", print_shader_log(obj));
				GLCheck!(gl::DeleteShader(obj));
				return Err(err);
			}

			objects.insert(name, obj);
			Ok(obj)
		};

		use ShaderType::*;
		let (name, objects) = if let Some(geom) = geom {
			let n = format!("v:{vert}|g:{geom}|p:{pix}");
			let o = vec![(vert, vertex), (geom, geometry), (pix, fragment)];
			(n, o)
		} else {
			let n = format!("v:{vert}|p:{pix}");
			let o = vec![(vert, vertex), (pix, fragment)];
			(n, o)
		};

		let objects = objects.into_iter().map(get_object).collect::<Res<Vec<_>>>()?;
		let prog = Object::new();
		let obj = prog.obj;

		objects.iter().for_each(|&o| GLCheck!(gl::AttachShader(obj, o)));
		GLCheck!(gl::LinkProgram(obj));
		let mut status: i32 = 0;
		GLCheck!(gl::GetProgramiv(obj, gl::LINK_STATUS, &mut status));
		objects.iter().for_each(|&o| GLCheck!(gl::DetachShader(obj, o)));

		if GLbool::to(status) == gl::FALSE {
			return Err(format!("Error linking program {name:?}, {obj}\n{}", print_shader_log(obj)));
		}

		Ok((prog, name.into()))
	}

	fn inline_source(name: &str, source: &str) {
		let m = Self::get();
		if let Some(_found) = m.sources.get(name) {
			ASSERT!(*_found == CString::new(source).unwrap(), "Shader {name:?} already exists",);
		} else {
			m.sources.insert(name.into(), CString::new(source).unwrap());
		}
	}

	fn get() -> &'static mut Self {
		LocalStatic!(ShaderManager, { Def() })
	}
}

impl<'a> From<InlineShader> for C<'a> {
	fn from(v: InlineShader) -> Self {
		let InlineShader(v, v_t) = v;
		ShaderManager::inline_source(v, v_t);
		v.into()
	}
}
pub struct InlineShader(pub STR, pub STR);
