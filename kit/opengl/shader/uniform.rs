use super::*;
use crate::math::la;

#[macro_export]
macro_rules! Uniform {
	($bind: ident, ($n: expr, $v: expr)) => {{
		let id = const { $crate::GL::macro_uses::uniforms_use::id($n) };
		$bind.tap(|b| b.Uniform(id, $v))
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

macro_rules! impl_uniform {
	($v: ident, $t: ident, $f: ident) => {
		impl UniformArgs for $t {
			fn apply(&self, addr: i32, cached: UniCache) {
				let apply = || {
					DEBUG!("Setting GL Uniform {addr} to {self:?} in shader {}", ShaderT::bound_obj());
					let s = &[*self];
					$v(gl::$f, addr, s);
				};

				if let Some(CachedUni::$t(v)) = cached {
					if v != self {
						*v = *self;
						apply();
					}
				} else {
					apply();
					*cached = Some(CachedUni::$t(*self));
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
impl_uniform!(val, u32, Uniform1uiv);
impl_uniform!(val, uVec2, Uniform2uiv);
impl_uniform!(val, uVec3, Uniform3uiv);
impl_uniform!(val, uVec4, Uniform4uiv);
impl_uniform!(val, i32, Uniform1iv);
impl_uniform!(val, iVec2, Uniform2iv);
impl_uniform!(val, iVec3, Uniform3iv);
impl_uniform!(val, iVec4, Uniform4iv);
impl_uniform!(val, f32, Uniform1fv);
impl_uniform!(val, Vec2, Uniform2fv);
impl_uniform!(val, Vec3, Uniform3fv);
impl_uniform!(val, Vec4, Uniform4fv);
impl_uniform!(mat, Mat2, UniformMatrix2fv);
impl_uniform!(mat, Mat3, UniformMatrix3fv);
impl_uniform!(mat, Mat4, UniformMatrix4fv);
impl_uniform!(mat, Mat2x3, UniformMatrix2x3fv);
impl_uniform!(mat, Mat3x2, UniformMatrix3x2fv);
impl_uniform!(mat, Mat2x4, UniformMatrix2x4fv);
impl_uniform!(mat, Mat4x2, UniformMatrix4x2fv);
impl_uniform!(mat, Mat3x4, UniformMatrix3x4fv);
impl_uniform!(mat, Mat4x3, UniformMatrix4x3fv);

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

impl<T: TexType> UniformArgs for GL::TextureBind<'_, T> {
	fn apply(&self, addr: i32, cached: UniCache) {
		let u = i32(self.u);
		let apply = || {
			DEBUG!("Binding GL texture {addr} to unit {u} in shader {}", ShaderT::bound_obj());
			GL!(gl::Uniform1i(addr, u));
		};

		if let Some(CachedUni::TexUnit(unit)) = cached {
			if *unit != u {
				DEBUG!("GL texture {addr} was bound to unit {unit}");
				apply();
				*unit = u;
			}
		} else {
			apply();
			*cached = CachedUni::TexUnit(u).pipe(Some);
		}
	}
}

impl UniformArgs for GL::ShdArrBind<'_, Uniform> {
	fn apply(&self, addr: i32, cached: UniCache) {
		let l = i32(self.l);
		let apply = || {
			let prog = *ShaderT::bound_obj();
			let addr = -2 - addr;
			DEBUG!("Binding GL UBO {addr} to location {l} in shader {prog}");
			GL!(gl::UniformBlockBinding(prog, u32(addr), u32(l)));
		};

		if let Some(CachedUni::UboLoc(loc)) = cached {
			if *loc != l {
				DEBUG!("GL UBO {addr} was bound to location {loc}");
				apply();
				*loc = l;
			}
		} else {
			apply();
			*cached = CachedUni::UboLoc(l).pipe(Some);
		}
	}
	fn kind(&self) -> ArgsKind {
		ArgsKind::Ubo
	}
}

impl UniformArgs for GL::ShdArrBind<'_, ShdStorage> {
	fn apply(&self, _: i32, _: UniCache) {
		unreachable!()
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
	u32(u32),
	uVec2(uVec2),
	uVec3(uVec3),
	uVec4(uVec4),
	i32(i32),
	iVec2(iVec2),
	iVec3(iVec3),
	iVec4(iVec4),
	f32(f32),
	Vec2(Vec2),
	Vec3(Vec3),
	Vec4(Vec4),
	Mat2(Mat2),
	Mat3(Mat3),
	Mat4(Mat4),
	Mat2x3(Mat2x3),
	Mat3x2(Mat3x2),
	Mat2x4(Mat2x4),
	Mat4x2(Mat4x2),
	Mat3x4(Mat3x4),
	Mat4x3(Mat4x3),
	TexUnit(i32),
	UboLoc(i32),
}
