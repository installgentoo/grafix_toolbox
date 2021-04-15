use super::{policy::*, state::*, texture::TextureBinding};
use crate::uses::*;

#[macro_export]
macro_rules! Uniform {
	($b: ident, ($n: expr, $v: expr)) => {{
		const _ID: (u32, &str) = shader_use::id($n);
		let mut b = $b;
		b.Uniform(_ID, $v);
		b
	}};
}

#[macro_export]
macro_rules! Uniforms {
($s: ident, $(($n: expr, $v: expr)),+) => {{
	let b = $s.Bind();
	$(let b = Uniform!(b, ($n, $v));)+
	b
}};
}

pub trait UniformImpl {
	fn apply(&self, _: i32);
}
impl UniformImpl for [u32] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform1uiv, name, self);
	}
}
impl UniformImpl for [uVec2] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform2uiv, name, self);
	}
}
impl UniformImpl for [uVec3] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform3uiv, name, self);
	}
}
impl UniformImpl for [uVec4] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform4uiv, name, self);
	}
}
impl UniformImpl for [i32] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform1iv, name, self);
	}
}
impl UniformImpl for [iVec2] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform2iv, name, self);
	}
}
impl UniformImpl for [iVec3] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform3iv, name, self);
	}
}
impl UniformImpl for [iVec4] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform4iv, name, self);
	}
}
impl UniformImpl for [f32] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform1fv, name, self);
	}
}
impl UniformImpl for [Vec2] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform2fv, name, self);
	}
}
impl UniformImpl for [Vec3] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform3fv, name, self);
	}
}
impl UniformImpl for [Vec4] {
	fn apply(&self, name: i32) {
		uni(gl::Uniform4fv, name, self);
	}
}
impl UniformImpl for [Mat2] {
	fn apply(&self, name: i32) {
		uni_mat(gl::UniformMatrix2fv, name, self);
	}
}
impl UniformImpl for [Mat3] {
	fn apply(&self, name: i32) {
		uni_mat(gl::UniformMatrix3fv, name, self);
	}
}
impl UniformImpl for [Mat4] {
	fn apply(&self, name: i32) {
		uni_mat(gl::UniformMatrix4fv, name, self);
	}
}
impl UniformImpl for [Mat2x3] {
	fn apply(&self, name: i32) {
		uni_mat(gl::UniformMatrix2x3fv, name, self);
	}
}
impl UniformImpl for [Mat3x2] {
	fn apply(&self, name: i32) {
		uni_mat(gl::UniformMatrix3x2fv, name, self);
	}
}
impl UniformImpl for [Mat2x4] {
	fn apply(&self, name: i32) {
		uni_mat(gl::UniformMatrix2x4fv, name, self);
	}
}
impl UniformImpl for [Mat4x2] {
	fn apply(&self, name: i32) {
		uni_mat(gl::UniformMatrix4x2fv, name, self);
	}
}
impl UniformImpl for [Mat3x4] {
	fn apply(&self, name: i32) {
		uni_mat(gl::UniformMatrix3x4fv, name, self);
	}
}
impl UniformImpl for [Mat4x3] {
	fn apply(&self, name: i32) {
		uni_mat(gl::UniformMatrix4x3fv, name, self);
	}
}

fn uni<T, S>(f: unsafe fn(i32, i32, *const T), name: i32, slice: &[S]) {
	GLCheck!(f(name, i32::to(slice.len()), slice.as_ptr() as *const T));
}

fn uni_mat<S>(f: unsafe fn(i32, i32, GLbool, *const f32), name: i32, slice: &[S]) {
	GLCheck!(f(name, i32::to(slice.len()), gl::FALSE, slice.as_ptr() as *const f32));
}

pub trait UniformArgs {
	fn get(self, _: i32, _: &mut HashMap<i32, i32>);
}
impl<'l, T: TexType> UniformArgs for &TextureBinding<'l, T> {
	fn get(self, name: i32, tex_cache: &mut HashMap<i32, i32>) {
		let u = i32::to(self.u);
		let ent = tex_cache.entry(name).or_insert(-1);
		if *ent != u {
			DEBUG!("Updating GL tex {} to {} in shader {}", *ent, u, ShdProg::bound_obj());
			GLCheck!(gl::Uniform1i(name, u));
			*ent = u;
		}
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
