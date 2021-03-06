use super::{args::*, object::*, parsing::*, policy::*, state::*, types::*, uniforms::*};
use crate::uses::{asyn::*, *};
use std::ffi::CString;

#[macro_export]
macro_rules! SHADER {
	($n: ident, $($body: expr),+) => {
		#[allow(non_upper_case_globals)]
		pub const $n: crate::uses::GL::macro_uses::InlineShader = crate::uses::GL::macro_uses::InlineShader(stringify!($n), const_format::concatcp!(crate::uses::GL::unigl::GLSL_VERSION, $($body,)+));
	};
}

pub struct Shader {
	program: Object<ShdProg>,
	name: String,
	uniforms: HashMap<u32, i32>,
	tex_cache: HashMap<i32, i32>,
}
impl Shader {
	pub fn new(args: impl ShdTypeArgs) -> Res<Shader> {
		let (program, name) = ShaderManager::compile(args.get())?;

		Ok(Self {
			program,
			name,
			uniforms: HashMap::new(),
			tex_cache: HashMap::new(),
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
	}
	//TODO uniform blocks
	pub fn Uniform(&mut self, (id, name): (u32, Str), args: impl UniformArgs) {
		ASSERT!(crate::uses::GL::macro_uses::uniforms_use::id(name).0 == id, "Use Uniforms!() macro to set uniforms");
		let addr = if let Some(found) = self.shd.uniforms.get(&id) {
			let _collision_map = UnsafeOnce!(HashMap<u32, String>, { HashMap::new() });
			ASSERT!(_collision_map.entry(id).or_insert(name.into()) == name, "Unifrom collision at entry {}", name);
			*found
		} else {
			let c_name = match CString::new(name) {
				Ok(str) => str,
				Err(e) => {
					FAILED!(&e.to_string());
					return;
				}
			};
			let addr = GLCheck!(gl::GetUniformLocation(self.shd.program.obj, c_name.as_ptr()));
			if addr == -1 {
				INFO!("No uniform '{}' in shader '{}', or uniform was optimized out", name, self.shd.name);
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
	VERTEX = 0,
	FRAGMENT = 1,
	GEOMETRY = 2,
}
const SHD_DEFS: [(GLenum, &str); 3] = [(gl::VERTEX_SHADER, "vertex"), (gl::FRAGMENT_SHADER, "pixel"), (gl::GEOMETRY_SHADER, "geometry")];

#[derive(Default)]
pub struct ShaderManager {
	objects: HashMap<CowStr, u32>,
	sources: HashMap<CowStr, CString>,
	loading: Vec<Task<SourcePack>>,
}
impl ShaderManager {
	pub fn LoadSources(filename: impl Into<CowStr>) {
		let m = Self::get();
		let name = filename.into();
		m.loading.push(task::spawn(async move {
			let data = OR_DEF!(Res(FS::read_text(name.as_ref()).await));
			parse_shader_sources(&name, &data)
		}));
	}
	pub fn ForceSource(name: impl Into<CowStr>, source: String) {
		let name = name.into();
		let m = Self::get();
		m.sources
			.insert(name.clone(), EXPECT!(CString::new(source)))
			.map(|_| m.objects.remove(&name).map(|o| GLCheck!(gl::DeleteShader(o))));
	}
	pub fn ClearSources() {
		let m = Self::get();
		m.objects.iter().for_each(|(_, o)| GLCheck!(gl::DeleteShader(*o)));
		m.objects.clear();
	}

	fn compile((vert, geom, pix): CompileArgs) -> Res<(Object<ShdProg>, String)> {
		let m = Self::get();
		let mut get_object = |name: CowStr, typ| {
			let Self { objects, sources, loading } = m;
			if let Some(found) = objects.get(&name) {
				return Ok(*found);
			}

			task::block_on(async move {
				let mut s = vec![];
				//TODO ASYNC CLOSURES
				for t in loading.drain(..) {
					s.push(t.await)
				}
				s
			})
			.into_iter()
			.flatten()
			.for_each(|(name, body)| {
				let exists = sources.insert(name.clone().into(), body);
				if exists.is_some() {
					FAILED!("Shader source '{}' was already loaded", name);
				};
			});

			let (typ, type_name) = SHD_DEFS[typ as usize];
			let source = PASS!(sources.get(&name), |_| CONCAT!("No ", type_name, " shader '", &name, "' in loaded sources"));

			let obj = GLCheck!(gl::CreateShader(typ));
			ASSERT!(obj != 0, "Failed to create {} shader object '{}'", type_name, &name);
			GLCheck!(gl::ShaderSource(obj, 1, &source.as_ptr(), ptr::null()));
			GLCheck!(gl::CompileShader(obj));
			let mut status: i32 = 0;
			GLCheck!(gl::GetShaderiv(obj, gl::COMPILE_STATUS, &mut status));
			if GLbool::to(status) != gl::TRUE {
				let err = CONCAT!("Error compiling ", type_name, " shader '", &name, "'\n", &print_shader_log(obj));
				GLCheck!(gl::DeleteShader(obj));
				return Err(err);
			}

			objects.insert(name, obj);
			Ok(obj)
		};

		use ShaderType::*;
		let (name, objects) = if let Some(geom) = geom {
			let n = CONCAT!("v:", &vert, "|g:", &geom, "|p:", &pix);
			let o = vec![(vert, VERTEX), (geom, GEOMETRY), (pix, FRAGMENT)];
			(n, o)
		} else {
			let n = CONCAT!("v:", &vert, "|p:", &pix);
			let o = vec![(vert, VERTEX), (pix, FRAGMENT)];
			(n, o)
		};

		let objects: Vec<_> = objects.into_iter().map(|(n, t)| get_object(n, t)).collect::<Res<_>>()?;
		let prog = Object::new();
		let obj = prog.obj;

		objects.iter().for_each(|&o| GLCheck!(gl::AttachShader(obj, o)));
		GLCheck!(gl::LinkProgram(obj));
		let mut status: i32 = 0;
		GLCheck!(gl::GetProgramiv(obj, gl::LINK_STATUS, &mut status));
		objects.iter().for_each(|&o| GLCheck!(gl::DetachShader(obj, o)));

		if GLbool::to(status) == gl::FALSE {
			return Err(CONCAT!("Error linking program '", &name, "',", &obj.to_string(), "\n", &print_shader_log(obj)));
		}

		Ok((prog, name))
	}

	fn inline_source(name: Str, source: Str) {
		let m = Self::get();
		if let Some(_found) = m.sources.get(name) {
			ASSERT!(*_found == EXPECT!(CString::new(source)), "Shader source '{}' already loaded", name);
		} else {
			m.sources.insert(name.into(), EXPECT!(CString::new(source)));
		}
	}

	fn get() -> &'static mut Self {
		UnsafeOnce!(ShaderManager, { Def() })
	}
}

impl Into<CowStr> for InlineShader {
	fn into(self) -> CowStr {
		let InlineShader(v, v_t) = self;
		ShaderManager::inline_source(v, v_t);
		v.into()
	}
}
pub struct InlineShader(pub Str, pub Str);
