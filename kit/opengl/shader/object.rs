use super::*;

#[derive(Debug)]
pub enum ShaderObj {
	Vertex(Obj<ShdVertT>),
	Fragment(Obj<ShdPixT>),
	Geometry(Obj<ShdGeomT>),
	Compute(Obj<ShdCompT>),
	TessCtrl(Obj<ShdCtrlT>),
	TessEval(Obj<ShdEvalT>),
}
impl ShaderObj {
	pub fn new(name: &str, src: &CString) -> Res<Self> {
		let obj = match name.slice(..2) {
			"vs" => Vertex(Def()),
			"ps" => Fragment(Def()),
			"gs" => Geometry(Def()),
			"cs" => Compute(Def()),
			"tc" => TessCtrl(Def()),
			"te" => TessEval(Def()),
			_ => format!("Shader name {name:?} should start with vs|ps|gs|cs|tc|te according to type").pipe(Err)?,
		};

		let o = obj.obj();
		GL!(gl::ShaderSource(o, 1, &src.as_ptr(), ptr::null()));
		GL!(gl::CompileShader(o));
		let mut status: i32 = 0;
		GL!(gl::GetShaderiv(o, gl::COMPILE_STATUS, &mut status));
		if GLbool::to(status) != gl::TRUE {
			format!("Error compiling {} shader {name:?}\n\n{}", obj.name(), parsing::print_shader_log(o)).pipe(Err)?
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
		match name.slice(..2) {
			"vs" | "ps" | "gs" | "cs" | "tc" | "te" => Ok(()),
			_ => format!("Shader name '{name}' should start with vs|ps|gs|cs|tc|te according to type").pipe(Err),
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
