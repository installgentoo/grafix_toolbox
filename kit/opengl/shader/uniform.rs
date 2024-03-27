use super::{GL, *};

#[macro_export]
macro_rules! Uniform {
	($bind: ident, ($n: expr, $v: expr)) => {{
		const _ID: (u32, &str) = $crate::GL::macro_uses::uniforms_use::id($n);
		let mut b = $bind;
		b.Uniform(_ID, $v);
		b
	}};
}

#[macro_export]
macro_rules! Uniforms {
($shd: ident, $(($n: expr, $v: expr)),+) => {{
	let b = $shd.Bind();
	$(let b = Uniform!(b, ($n, $v));)+
	b
}};
}

pub trait UniformImpl {
	fn apply(&self, _: i32);
}
macro_rules! impl_uniform_type {
	($v: ident, $t: ty, $f: ident) => {
		impl UniformImpl for $t {
			fn apply(&self, name: i32) {
				$v(gl::$f, name, &[*self]);
			}
		}
		impl UniformImpl for [$t] {
			fn apply(&self, name: i32) {
				$v(gl::$f, name, self);
			}
		}
	};
}
fn val<T, S>(f: unsafe fn(i32, i32, *const T), name: i32, slice: &[S]) {
	GLCheck!(f(name, i32(slice.len()), slice.as_ptr() as *const T));
}
fn mat<S>(f: unsafe fn(i32, i32, GLbool, *const f32), name: i32, slice: &[S]) {
	GLCheck!(f(name, i32(slice.len()), gl::FALSE, slice.as_ptr() as *const f32));
}
impl_uniform_type!(val, u32, Uniform1uiv);
impl_uniform_type!(val, uVec2, Uniform2uiv);
impl_uniform_type!(val, uVec3, Uniform3uiv);
impl_uniform_type!(val, uVec4, Uniform4uiv);
impl_uniform_type!(val, i32, Uniform1iv);
impl_uniform_type!(val, iVec2, Uniform2iv);
impl_uniform_type!(val, iVec3, Uniform3iv);
impl_uniform_type!(val, iVec4, Uniform4iv);
impl_uniform_type!(val, f32, Uniform1fv);
impl_uniform_type!(val, Vec2, Uniform2fv);
impl_uniform_type!(val, Vec3, Uniform3fv);
impl_uniform_type!(val, Vec4, Uniform4fv);
impl_uniform_type!(mat, Mat2, UniformMatrix2fv);
impl_uniform_type!(mat, Mat3, UniformMatrix3fv);
impl_uniform_type!(mat, Mat4, UniformMatrix4fv);
impl_uniform_type!(mat, Mat2x3, UniformMatrix2x3fv);
impl_uniform_type!(mat, Mat3x2, UniformMatrix3x2fv);
impl_uniform_type!(mat, Mat2x4, UniformMatrix2x4fv);
impl_uniform_type!(mat, Mat4x2, UniformMatrix4x2fv);
impl_uniform_type!(mat, Mat3x4, UniformMatrix3x4fv);
impl_uniform_type!(mat, Mat4x3, UniformMatrix4x3fv);

#[allow(clippy::upper_case_acronyms)]
pub enum ArgsKind {
	Uniform,
	UBO,
	SSBO,
}
pub trait UniformArgs {
	fn pass(self, _: i32, _: &mut BindCache);
	fn kind(&self) -> ArgsKind {
		ArgsKind::Uniform
	}
}
impl<T> UniformArgs for &T
where
	T: UniformImpl,
{
	fn pass(self, addr: i32, _: &mut BindCache) {
		self.apply(addr);
	}
}
impl<T> UniformArgs for T
where
	[T]: UniformImpl,
{
	fn pass(self, addr: i32, _: &mut BindCache) {
		[self].apply(addr);
	}
}
impl<T> UniformArgs for &[T]
where
	[T]: UniformImpl,
{
	fn pass(self, addr: i32, _: &mut BindCache) {
		self.apply(addr);
	}
}
impl<T, const L: usize> UniformArgs for [T; L]
where
	[T]: UniformImpl,
{
	fn pass(self, addr: i32, _: &mut BindCache) {
		self.apply(addr);
	}
}

impl<T: TexType> UniformArgs for &GL::TextureBinding<'_, T> {
	fn pass(self, addr: i32, binds_cache: &mut BindCache) {
		let u = i32(self.u);
		let unit = binds_cache.entry(addr).or_insert(-1);
		if *unit != u {
			DEBUG!("Binding GL texture {addr} to {u} in shader {}, was {unit}", ShaderProg::bound_obj());
			GLCheck!(gl::Uniform1i(addr, u));
			*unit = u;
		}
	}
}

impl UniformArgs for &GL::ShdArrBinding<'_, Uniform> {
	fn pass(self, addr: i32, binds_cache: &mut BindCache) {
		let l = i32(self.l);
		let loc = binds_cache.entry(addr + i32::MAX / 2).or_insert(-1);
		if *loc != l {
			let prog = *ShaderProg::bound_obj();
			DEBUG!("Binding GL UBO {addr} to {l} in shader {prog}, was {loc}",);
			GLCheck!(gl::UniformBlockBinding(prog, u32(addr), u32(l)));
			*loc = l;
		}
	}
	fn kind(&self) -> ArgsKind {
		ArgsKind::UBO
	}
}

impl UniformArgs for &GL::ShdArrBinding<'_, ShdStorage> {
	fn pass(self, _: i32, _: &mut BindCache) {
		unreachable!();
	}
	fn kind(&self) -> ArgsKind {
		ArgsKind::SSBO
	}
}

pub mod uniforms_use {
	pub const fn id(s: &str) -> (u32, &str) {
		(super::chksum::const_fnv1(s.as_bytes()), s)
	}
}

type BindCache = HashMap<i32, i32>;
