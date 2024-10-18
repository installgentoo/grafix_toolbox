macro_rules! impl_func_cast {
	($($t: ident),+) => {
		$(pub fn $t<T>(v: T) -> $t
		where
			$t: Cast<T>,
		{
			$t::to(v)
		})+
	};
}
macro_rules! def_vec {
	($n2: ident, $n3: ident, $n4: ident, $t: ty) => {
		pub type $n2 = vec2<$t>;
		pub type $n3 = vec3<$t>;
		pub type $n4 = vec4<$t>;
		impl_func_cast!($n2, $n3, $n4);
	};
}

pub mod pre {
	pub use super::cast::{f16, Cast};
	impl_func_cast!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64, usize, isize);

	def_vec!(hVec2, hVec3, hVec4, f16);
	def_vec!(Vec2, Vec3, Vec4, f32);
	def_vec!(dVec2, dVec3, dVec4, f64);
	def_vec!(ibVec2, ibVec3, ibVec4, i8);
	def_vec!(ubVec2, ubVec3, ubVec4, u8);
	def_vec!(isVec2, isVec3, isVec4, i16);
	def_vec!(usVec2, usVec3, usVec4, u16);
	def_vec!(iVec2, iVec3, iVec4, i32);
	def_vec!(uVec2, uVec3, uVec4, u32);
	def_vec!(ilVec2, ilVec3, ilVec4, isize);
	def_vec!(ulVec2, ulVec3, ulVec4, usize);

	pub type Mat2 = mat2<f32>;
	pub type Mat3 = mat3<f32>;
	pub type Mat4 = mat4<f32>;
	pub type Mat2x3 = mat2x3<f32>;
	pub type Mat3x2 = mat3x2<f32>;
	pub type Mat2x4 = mat2x4<f32>;
	pub type Mat4x2 = mat4x2<f32>;
	pub type Mat3x4 = mat3x4<f32>;
	pub type Mat4x3 = mat4x3<f32>;
	impl_func_cast!(Mat2, Mat3, Mat4, Mat2x3, Mat3x2, Mat2x4, Mat4x2, Mat3x4, Mat4x3);

	pub type vec2<T> = (T, T);
	pub type vec3<T> = (T, T, T);
	pub type vec4<T> = (T, T, T, T);

	pub type mat2<T> = (vec2<T>, vec2<T>);
	pub type mat3<T> = (vec3<T>, vec3<T>, vec3<T>);
	pub type mat4<T> = (vec4<T>, vec4<T>, vec4<T>, vec4<T>);
	pub type mat2x3<T> = (vec3<T>, vec3<T>);
	pub type mat3x2<T> = (vec2<T>, vec2<T>, vec2<T>);
	pub type mat2x4<T> = (vec4<T>, vec4<T>);
	pub type mat4x2<T> = (vec2<T>, vec2<T>, vec2<T>, vec2<T>);
	pub type mat3x4<T> = (vec4<T>, vec4<T>, vec4<T>);
	pub type mat4x3<T> = (vec3<T>, vec3<T>, vec3<T>, vec3<T>);
}

pub mod ext {
	pub use super::{cast::matrix::*, math_ext::*, pre::*, tuple::*};
}

pub mod la;

mod cast;
mod math_ext;
mod tuple;
