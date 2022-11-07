use crate::uses::*;

impl<T: Copy> Cast<&[T]> for vec2<T> {
	fn to(v: &[T]) -> Self {
		ASSERT!(v.len() > 1, "Slice is too short for Vec2");
		(*v.at(0), *v.at(1))
	}
}
impl<T: Copy> Cast<&[T]> for vec3<T> {
	fn to(v: &[T]) -> Self {
		ASSERT!(v.len() > 2, "Slice is too short for Vec3");
		(*v.at(0), *v.at(1), *v.at(2))
	}
}
impl<T: Copy> Cast<&[T]> for vec4<T> {
	fn to(v: &[T]) -> Self {
		ASSERT!(v.len() > 3, "Slice is too short for Vec4");
		(*v.at(0), *v.at(1), *v.at(2), *v.at(3))
	}
}

macro_rules! impl_transmute {
	($t: ty) => {
		impl Cast<vec2<$t>> for [$t; 2] {
			fn to(v: vec2<$t>) -> Self {
				unsafe { mem::transmute(v) }
			}
		}
		impl Cast<vec3<$t>> for [$t; 3] {
			fn to(v: vec3<$t>) -> Self {
				unsafe { mem::transmute(v) }
			}
		}
		impl Cast<vec4<$t>> for [$t; 4] {
			fn to(v: vec4<$t>) -> Self {
				unsafe { mem::transmute(v) }
			}
		}
	};
}
impl_transmute!(u8);
impl_transmute!(i8);
impl_transmute!(u16);
impl_transmute!(i16);
impl_transmute!(u32);
impl_transmute!(i32);
impl_transmute!(f16);
impl_transmute!(f32);
