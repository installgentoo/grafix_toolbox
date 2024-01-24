use super::{args::*, parsing::*, uniform::*, *};
use crate::GL::{offhand::*, window::*};
use crate::{lazy::*, sync::*, FS};
use std::ffi::CString;

#[macro_export]
macro_rules! SHADER {
	($n: ident, $($body: expr),+) => {
		#[allow(non_upper_case_globals)]
		pub const $n: $crate::GL::macro_uses::InlineShader = $crate::GL::macro_uses::InlineShader(stringify!($n), const_format::concatcp!($crate::GL::unigl::GLSL_VERSION, $($body,)+));
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
		ASSERT!(crate::GL::macro_uses::uniforms_use::id(name).0 == id, "Use Uniforms!() macro to set uniforms");
		let addr = if let Some(found) = self.shd.uniforms.get(&id) {
			let _collision_map = LocalStatic!(HashMap<u32, String>);
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
	sources: HashMap<Box<str>, (CString, Option<RcLazy<SourcePack>>)>,
	//watched: HashMap<Box<str>, >   ////////////////////////////////////////////  do like wath that we can check per name
	files: Vec<Lazy<SourcePack>>,
}
impl ShaderManager {
	pub fn LoadSources(filename: impl ToString) {
		let n = filename.to_string();
		Self::get().files.push(Lazy::new(FS::Lazy::Text(n.clone()).map(move |s| parse_shader_sources(&n, &s))));
	}
	pub fn WatchSources(filename: impl ToString) {
		let n = filename.to_string();
		Self::get().files.push(Lazy::new(FS::Watch::Text(n.clone()).map(move |s| parse_shader_sources(&n, &s))));
	}
	pub fn ForceSource(name: impl ToString, source: impl ToString) {
		let Self { objects, sources, .. } = Self::get();
		let name = name.to_string().into();
		objects.remove(&name).map(|o| GLCheck!(gl::DeleteShader(o)));
		sources.insert(name, (CString::new(source.to_string()).unwrap(), None));
	}
	pub fn ClearSources() {
		let Self { objects, .. } = Self::get();
		objects.iter().for_each(|(_, &o)| GLCheck!(gl::DeleteShader(o)));
		objects.clear();
	}

	pub fn RebuildWatched() {
		let Self { objects, sources, .. } = Self::get();
	}

	fn compile((vert, geom, pix): CompileArgs) -> Res<(Object<ShdProg>, Box<str>)> {
		let Self { objects, sources, files, .. } = Self::get();
		let get_object = |(name, typ): (C<'_>, _)| {
			if let Some(found) = objects.get(&*name) {
				return Ok(*found);
			}

			files.drain(..).for_each(|f| {
				let mut f = RcLazy::from(f);
				let rc = f.clone();
				f.get().iter_mut().for_each(|(name, body)| {
					sources
						.insert(name.clone(), (mem::take(body), Some(rc.clone())))
						.map(|_| WARN!("Shader source {name:?} was already loaded"))
						.sink()
					})
			});

			let (name, (typ, type_name)) = (name.into_owned().into_boxed_str(), SHD_DEFS[typ as usize]);
			let source = Res(sources.get(&name)).map_err(|_| format!("No {type_name} shader {name:?} in loaded sources"))?;

			let obj = GLCheck!(gl::CreateShader(typ));
			ASSERT!(obj != 0, "Failed to create {type_name} shader object {name:?}");
			GLCheck!(gl::ShaderSource(obj, 1, &source.0.as_ptr(), ptr::null()));
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
			ASSERT!((*_found).0 == CString::new(source).unwrap(), "Shader {name:?} already exists",);
		} else {
			m.sources.insert(name.into(), (CString::new(source).unwrap(), None));
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
