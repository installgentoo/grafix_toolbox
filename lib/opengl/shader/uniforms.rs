use super::{policy::*, texture::TextureBinding};
use crate::uses::*;

#[macro_export]
macro_rules! Uniform {
	($bind: ident, ($n: expr, $v: expr)) => {{
		const _ID: (u32, &str) = crate::uses::GL::macro_uses::uniforms_use::id($n);
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

pub trait UniformArgs {
	fn get(self, _: i32, _: &mut HashMap<i32, i32>);
}
impl<T: TexType> UniformArgs for &TextureBinding<'_, T> {
	fn get(self, name: i32, tex_cache: &mut HashMap<i32, i32>) {
		let u = i32(self.u);
		let ent = tex_cache.entry(name).or_insert(-1);
		if *ent != u {
			DEBUG!("Updating GL tex {ent} to {u} in shader {}", {
				use super::state::*;
				ShdProg::bound_obj()
			});
			GLCheck!(gl::Uniform1i(name, u));
			*ent = u;
		}
	}
}
impl<T> UniformArgs for &T
where
	T: UniformImpl,
{
	fn get(self, name: i32, _: &mut HashMap<i32, i32>) {
		self.apply(name);
	}
}
impl<T> UniformArgs for T
where
	[T]: UniformImpl,
{
	fn get(self, name: i32, _: &mut HashMap<i32, i32>) {
		[self].apply(name);
	}
}
impl<T> UniformArgs for &[T]
where
	[T]: UniformImpl,
{
	fn get(self, name: i32, _: &mut HashMap<i32, i32>) {
		self.apply(name);
	}
}
impl<T, const L: usize> UniformArgs for [T; L]
where
	[T]: UniformImpl,
{
	fn get(self, name: i32, _: &mut HashMap<i32, i32>) {
		self.apply(name);
	}
}
impl<T> UniformArgs for &Vec<T>
where
	[T]: UniformImpl,
{
	fn get(self, name: i32, _: &mut HashMap<i32, i32>) {
		self[..].apply(name);
	}
}
impl<T> UniformArgs for Vec<T>
where
	[T]: UniformImpl,
{
	fn get(self, name: i32, _: &mut HashMap<i32, i32>) {
		self[..].apply(name);
	}
}

pub mod uniforms_use {
	pub const fn id(s: &str) -> (u32, &str) {
		(super::chksum::const_fnv1(s.as_bytes()), s)
	}
}
