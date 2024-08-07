use super::*;
use crate::math::la;

#[macro_export]
macro_rules! Uniform {
	($bind: ident, ($n: expr, $v: expr)) => {{
		let id = const { $crate::GL::macro_uses::uniforms_use::id($n) };
		let mut b = $bind;
		b.Uniform(id, $v);
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

pub enum ArgsKind {
	Uniform,
	Ubo,
	Ssbo,
}
pub trait UniformArgs {
	fn apply(&self, _: i32, _: UniCache);
	fn kind(&self) -> ArgsKind {
		ArgsKind::Uniform
	}
}

macro_rules! impl_uniform_type {
	($v: ident, $t: ident, $f: ident) => {
		impl UniformArgs for $t {
			fn apply(&self, addr: i32, cached: UniCache) {
				let apply = || {
					DEBUG!("Setting GL Uniform {addr} to {self:?} in shader {}", ShaderProg::bound_obj());
					let s = &[*self];
					$v(gl::$f, addr, s);
				};

				if let Some(CachedUni::$t(v)) = cached {
					if **v != *self {
						**v = *self;
						apply();
					}
				} else {
					apply();
					*cached = Some(CachedUni::$t(Box(*self)));
				}
			}
		}
	};
}
fn val<T, S>(f: unsafe fn(i32, i32, *const T), name: i32, slice: &[S]) {
	GL!(f(name, i32(slice.len()), slice.as_ptr() as *const T));
}
fn mat<S>(f: unsafe fn(i32, i32, GLbool, *const f32), name: i32, slice: &[S]) {
	GL!(f(name, i32(slice.len()), gl::FALSE, slice.as_ptr() as *const f32)); // coulmn-major
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

macro_rules! impl_la_adapter {
	($t: ident, $lat: ident) => {
		impl UniformArgs for la::$lat {
			fn apply(&self, addr: i32, cached: UniCache) {
				$t(*self).apply(addr, cached)
			}
		}
		impl UniformArgs for &la::$lat {
			fn apply(&self, addr: i32, cached: UniCache) {
				(*self).apply(addr, cached)
			}
		}
	};
}
impl_la_adapter!(Vec2, V2);
impl_la_adapter!(Vec3, V3);
impl_la_adapter!(Vec4, V4);
impl_la_adapter!(Mat3, M3);
impl_la_adapter!(Mat4, M4);

impl<T: TexType> UniformArgs for GL::TextureBinding<'_, T> {
	fn apply(&self, addr: i32, cached: UniCache) {
		let u = i32(self.u);
		let apply = || {
			DEBUG!("Binding GL texture {addr} to unit {u} in shader {}", ShaderProg::bound_obj());
			GL!(gl::Uniform1i(addr, u));
		};

		if let Some(CachedUni::TexUnit(unit)) = cached {
			if **unit != u {
				DEBUG!("GL texture {addr} was bound to unit {unit}");
				apply();
				**unit = u;
			}
		} else {
			apply();
			*cached = Some(CachedUni::TexUnit(Box(u)));
		}
	}
}

impl UniformArgs for GL::ShdArrBinding<'_, Uniform> {
	fn apply(&self, addr: i32, cached: UniCache) {
		let l = i32(self.l);
		let apply = || {
			let prog = *ShaderProg::bound_obj();
			let addr = -2 - addr;
			DEBUG!("Binding GL UBO {addr} to location {l} in shader {prog}");
			GL!(gl::UniformBlockBinding(prog, u32(addr), u32(l)));
		};

		if let Some(CachedUni::UboLoc(loc)) = cached {
			if **loc != l {
				DEBUG!("GL UBO {addr} was bound to location {loc}");
				apply();
				**loc = l;
			}
		} else {
			apply();
			*cached = Some(CachedUni::UboLoc(Box(l)));
		}
	}
	fn kind(&self) -> ArgsKind {
		ArgsKind::Ubo
	}
}

impl UniformArgs for GL::ShdArrBinding<'_, ShdStorage> {
	fn apply(&self, _: i32, _: UniCache) {
		unreachable!();
	}
	fn kind(&self) -> ArgsKind {
		ArgsKind::Ssbo
	}
}

pub mod uniforms_use {
	pub const fn id(s: &str) -> (u32, &str) {
		(super::chksum::const_fnv1(s.as_bytes()), s)
	}
}

pub type UniCache<'a> = &'a mut Option<CachedUni>;

#[derive(Debug)]
pub enum CachedUni {
	u32(Box<u32>),
	uVec2(Box<uVec2>),
	uVec3(Box<uVec3>),
	uVec4(Box<uVec4>),
	i32(Box<i32>),
	iVec2(Box<iVec2>),
	iVec3(Box<iVec3>),
	iVec4(Box<iVec4>),
	f32(Box<f32>),
	Vec2(Box<Vec2>),
	Vec3(Box<Vec3>),
	Vec4(Box<Vec4>),
	Mat2(Box<Mat2>),
	Mat3(Box<Mat3>),
	Mat4(Box<Mat4>),
	Mat2x3(Box<Mat2x3>),
	Mat3x2(Box<Mat3x2>),
	Mat2x4(Box<Mat2x4>),
	Mat4x2(Box<Mat4x2>),
	Mat3x4(Box<Mat3x4>),
	Mat4x3(Box<Mat4x3>),
	TexUnit(Box<i32>),
	UboLoc(Box<i32>),
}
