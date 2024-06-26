use super::super::shader::uniform::*;
use crate::GL::{tex::*, Fbo};

#[macro_export]
macro_rules! ComputeShader {
	($shd: ident, ($n0: literal, $samp: ident, $slab: ident)$(, $($args: tt),+)?) => {{
		let mut fbo = &mut $slab.tgt;
		let src = &$slab.src.tex;
		ComputeShader!($shd, fbo, ($n0, $samp, src)$(, $($args),+)?);
		$slab.swap();
	}};
	($shd: ident, $fbo: expr) => {{
		$fbo.clear((0, 1));
		$fbo.bind();
		let _ = $shd.Bind();
		$crate::GL::mesh::Screen::Draw();
	}};
	($shd: ident, $fbo: expr, ($n0: literal, $v0: expr)) => {{
		$fbo.clear((0, 1));
		$fbo.bind();
		let s = $shd.Bind();
		let _ = Uniform!(s, ($n0, $v0));
		let _ = ComputeShader!($shd, $fbo);
	}};
	($shd: ident, $fbo: expr, ($n0: literal, $s0: ident, $t0: expr)) => {{
		$fbo.clear((0, 1));
		$fbo.bind();
		let s = $shd.Bind();
		let b = $t0.Bind(&$s0);
		let _ = Uniform!(s, ($n0, b));
		let _ = ComputeShader!($shd, $fbo);
	}};
	($shd: ident, $fbo: expr, ($n0: literal, $v0: expr), $($args: tt),+) => {{
		$fbo.clear((0, 1));
		$fbo.bind();
		let s = $shd.Bind();
		let _ = Uniform!(s, ($n0, $v0));
		let _ = ComputeShader!($shd, $fbo, $($args),+);
	}};
	($shd: ident, $fbo: expr, ($n0: literal, $s0: ident, $t0: expr), $($args: tt),+) => {{
		$fbo.clear((0, 1));
		$fbo.bind();
		let s = $shd.Bind();
		let b = $t0.Bind(&$s0);
		let _ = Uniform!(s, ($n0, b));
		let _ = ComputeShader!($shd, $fbo, $($args),+);
	}};
}

impl<T: UniformArgs> UniformArgs for (T, &Sampler) {
	fn apply(&self, addr: i32, c: UniCache) {
		self.0.apply(addr, c);
	}
}
impl<S, F> UniformArgs for (&Tex<S, F, GL_TEXTURE_2D>, &Sampler) {
	fn apply(&self, addr: i32, c: UniCache) {
		self.0.Bind(self.1).apply(addr, c);
	}
}
impl<S, F> UniformArgs for (&Fbo<S, F>, &Sampler) {
	fn apply(&self, addr: i32, c: UniCache) {
		(&self.0.tex, self.1).apply(addr, c);
	}
}
