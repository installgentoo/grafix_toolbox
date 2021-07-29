#![allow(dead_code)]
use crate::uses::*;

#[macro_export]
macro_rules! func_cast {
	($t: ident) => {
		pub fn $t<T>(v: T) -> $t
		where
			$t: Cast<T>,
		{
			$t::to(v)
		}
	};
}
func_cast!(f16);
func_cast!(f32);
func_cast!(f64);
func_cast!(i128);
func_cast!(i16);
func_cast!(i32);
func_cast!(i64);
func_cast!(i8);
func_cast!(isize);
func_cast!(u128);
func_cast!(u16);
func_cast!(u32);
func_cast!(u64);
func_cast!(u8);
func_cast!(usize);
func_cast!(hVec2);
func_cast!(hVec3);
func_cast!(hVec4);
func_cast!(Vec2);
func_cast!(Vec3);
func_cast!(Vec4);
func_cast!(dVec2);
func_cast!(dVec3);
func_cast!(dVec4);
func_cast!(ubVec2);
func_cast!(ubVec3);
func_cast!(ubVec4);
func_cast!(ibVec2);
func_cast!(ibVec3);
func_cast!(ibVec4);
func_cast!(usVec2);
func_cast!(usVec3);
func_cast!(usVec4);
func_cast!(isVec2);
func_cast!(isVec3);
func_cast!(isVec4);
func_cast!(uVec2);
func_cast!(uVec3);
func_cast!(uVec4);
func_cast!(iVec2);
func_cast!(iVec3);
func_cast!(iVec4);
func_cast!(ulVec2);
func_cast!(ulVec3);
func_cast!(ulVec4);
func_cast!(ilVec2);
func_cast!(ilVec3);
func_cast!(ilVec4);
func_cast!(Mat2);
func_cast!(Mat3);
func_cast!(Mat4);
func_cast!(Mat2x3);
func_cast!(Mat3x2);
func_cast!(Mat2x4);
func_cast!(Mat4x2);
func_cast!(Mat3x4);
func_cast!(Mat4x3);

pub fn Res<T, V>(v: T) -> Res<V>
where
	Res<V>: Cast<T>,
{
	Res::to(v)
}
