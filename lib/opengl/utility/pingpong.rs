use super::super::shader::uniforms::*;
use crate::uses::HashMap;
use crate::GL::{tex::*, window::*, Fbo, Sampler};

#[macro_export]
macro_rules! ComputeShader {
	($s: ident, $samp: ident, $fbo: ident, $(($n: expr, $v: expr)),+) => {{
		(&mut $fbo).bind_appl();
		let samp = &$samp;
		let s = $s.Bind();
		$(let s = Uniform!(s, ($n, ($v, samp)));)+
		Screen::Draw();
	}};
	($s: ident, $samp: ident, ($n0: expr, $slab: ident), $(($n: expr, $v: expr)),+) => {{
		{
			let mut fbo = &mut $slab.tgt;
			let src = &$slab.src.tex.Bind(&$samp);
			ComputeShader!($s, $samp, fbo, ($n0, src), $(($n, $v)),+);
		}
		$slab.swap();
	}};
	($s: ident, $samp: ident, ($n0: expr, $slab: ident)) => {{
		{
			let mut fbo = &mut $slab.tgt;
			let src = &$slab.src.tex.Bind(&$samp);
			ComputeShader!($s, $samp, fbo, ($n0, src));
		}
		$slab.swap();
	}};
}

impl<T: UniformArgs> UniformArgs for (T, &Sampler) {
	fn get(self, name: i32, tex_cache: &mut HashMap<i32, i32>) {
		self.0.get(name, tex_cache);
	}
}
impl<S: TexSize, F: TexFmt> UniformArgs for (&Tex<GL_TEXTURE_2D, S, F>, &Sampler) {
	fn get(self, name: i32, tex_cache: &mut HashMap<i32, i32>) {
		self.0.Bind(self.1).get(name, tex_cache);
	}
}
impl<S: TexSize, F: TexFmt> UniformArgs for (&Fbo<S, F>, &Sampler) {
	fn get(self, name: i32, tex_cache: &mut HashMap<i32, i32>) {
		(&self.0.tex, self.1).get(name, tex_cache);
	}
}

pub trait PPDrawableArg {
	fn bind_appl(self);
}
impl<S: TexSize, F: TexFmt> PPDrawableArg for &mut Fbo<S, F> {
	fn bind_appl(self) {
		self.clear();
		self.bind();
	}
}
impl<T: WindowPolicy> PPDrawableArg for &mut T {
	fn bind_appl(self) {
		self.draw_to_screen();
	}
}
