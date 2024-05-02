use super::*;

#[derive(Debug)]
pub enum ShaderObj {
	Vertex(Object<ShaderVert>),
	Fragment(Object<ShaderPix>),
	Geometry(Object<ShaderGeom>),
	Compute(Object<ShaderComp>),
	TessCtrl(Object<ShaderTCtrl>),
	TessEval(Object<ShaderTEval>),
}
impl ShaderObj {
	pub fn new(name: &str, src: &CString) -> Res<Self> {
		let obj = match slice((name, 2)) {
			"vs" => Vertex(Def()),
			"ps" => Fragment(Def()),
			"gs" => Geometry(Def()),
			"cs" => Compute(Def()),
			"tc" => TessCtrl(Def()),
			"te" => TessEval(Def()),
			_ => Err(format!("Shader name {name:?} should start with vs|ps|gs|cs|tc|te according to type"))?,
		};

		let o = obj.obj();
		GLCheck!(gl::ShaderSource(o, 1, &src.as_ptr(), ptr::null()));
		GLCheck!(gl::CompileShader(o));
		let mut status: i32 = 0;
		GLCheck!(gl::GetShaderiv(o, gl::COMPILE_STATUS, &mut status));
		if GLbool::to(status) != gl::TRUE {
			Err(format!("Error compiling {} shader {name:?}\n{}", obj.name(), parsing::print_shader_log(o)))?
		}

		Ok(obj)
	}
	pub fn obj(&self) -> u32 {
		match self {
			Vertex(o) => o.obj,
			Fragment(o) => o.obj,
			Geometry(o) => o.obj,
			Compute(o) => o.obj,
			TessCtrl(o) => o.obj,
			TessEval(o) => o.obj,
		}
	}
	pub fn valid(name: &str) -> Res<()> {
		match slice((name, 2)) {
			"vs" | "ps" | "gs" | "cs" | "tc" | "te" => Ok(()),
			_ => Err(format!("Shader name '{name}' should start with vs|ps|gs|cs|tc|te according to type")),
		}
	}
	fn name(&self) -> &str {
		match self {
			Vertex(_) => "vertex",
			Fragment(_) => "pixel",
			Geometry(_) => "geometry",
			Compute(_) => "compute",
			TessCtrl(_) => "tess. control",
			TessEval(_) => "tess. eval",
		}
	}
}
use ShaderObj::*;
