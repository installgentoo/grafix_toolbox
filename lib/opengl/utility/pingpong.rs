use super::super::shader::uniforms::*;
use crate::uses::HashMap;
use crate::GL::{tex::*, window::*, ClearScreen, Fbo, Sampler};

#[macro_export]
macro_rules! ComputeShader {
	($shd: ident, ($n0: literal, $samp: ident, $slab: ident)$(, $($args: tt),+)?) => {{
		{
			let mut fbo = &mut $slab.tgt;
			let src = &$slab.src.tex;
			ComputeShader!($shd, fbo, ($n0, $samp, src)$(, $($args),+)?);
		}
		$slab.swap();
	}};
	($shd: ident, $fbo: expr) => {{
		(&mut $fbo).bind_appl();
		let _ = $shd.Bind();
		Screen::Draw();
	}};
	($shd: ident, $fbo: expr, ($n0: literal, $v0: expr)) => {{
		(&mut $fbo).bind_appl();
		let s = $shd.Bind();
		let _ = Uniform!(s, ($n0, $v0));
		let _ = ComputeShader!($shd, $fbo);
	}};
	($shd: ident, $fbo: expr, ($n0: literal, $s0: ident, $t0: expr)) => {{
		(&mut $fbo).bind_appl();
		let s = $shd.Bind();
		let b = $t0.Bind(&$s0);
		let _ = Uniform!(s, ($n0, &b));
		let _ = ComputeShader!($shd, $fbo);
	}};
	($shd: ident, $fbo: expr, ($n0: literal, $v0: expr), $($args: tt),+) => {{
		(&mut $fbo).bind_appl();
		let s = $shd.Bind();
		let _ = Uniform!(s, ($n0, $v0));
		let _ = ComputeShader!($shd, $fbo, $($args),+);
	}};
	($shd: ident, $fbo: expr, ($n0: literal, $s0: ident, $t0: expr), $($args: tt),+) => {{
		(&mut $fbo).bind_appl();
		let s = $shd.Bind();
		let b = $t0.Bind(&$s0);
		let _ = Uniform!(s, ($n0, &b));
		let _ = ComputeShader!($shd, $fbo, $($args),+);
	}};
}

impl<T: UniformArgs> UniformArgs for (T, &Sampler) {
	fn get(self, name: i32, tex_cache: &mut HashMap<i32, i32>) {
		self.0.get(name, tex_cache);
	}
}
impl<S, F> UniformArgs for (&Tex<S, F, GL_TEXTURE_2D>, &Sampler) {
	fn get(self, name: i32, tex_cache: &mut HashMap<i32, i32>) {
		self.0.Bind(self.1).get(name, tex_cache);
	}
}
impl<S, F> UniformArgs for (&Fbo<S, F>, &Sampler) {
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
		ClearScreen((0., 1.));
	}
}
