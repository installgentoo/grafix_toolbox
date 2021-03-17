use crate::uses::*;

impl Cast<glm::Vec2> for Vec2 {
	fn to(v: glm::Vec2) -> Self {
		Self::from(unsafe { mem::transmute::<[_; 2], _>(v.into()) })
	}
}
impl Cast<glm::Vec3> for Vec3 {
	fn to(v: glm::Vec3) -> Self {
		Self::from(unsafe { mem::transmute::<[_; 3], _>(v.into()) })
	}
}
impl Cast<glm::Vec4> for Vec4 {
	fn to(v: glm::Vec4) -> Self {
		Self::from(unsafe { mem::transmute::<[_; 4], _>(v.into()) })
	}
}

impl<T: Copy> Cast<&[T]> for vec2<T> {
	fn to(v: &[T]) -> Self {
		ASSERT!(v.len() > 1, "Slice is too short for Vec2");
		unsafe { (*v.get_unchecked(0), *v.get_unchecked(1)) }
	}
}
impl<T: Copy> Cast<&[T]> for vec3<T> {
	fn to(v: &[T]) -> Self {
		ASSERT!(v.len() > 2, "Slice is too short for Vec3");
		unsafe { (*v.get_unchecked(0), *v.get_unchecked(1), *v.get_unchecked(2)) }
	}
}
impl<T: Copy> Cast<&[T]> for vec4<T> {
	fn to(v: &[T]) -> Self {
		ASSERT!(v.len() > 3, "Slice is too short for Vec4");
		unsafe { (*v.get_unchecked(0), *v.get_unchecked(1), *v.get_unchecked(2), *v.get_unchecked(3)) }
	}
}

impl Cast<Vec2> for glm::Vec2 {
	fn to(v: Vec2) -> Self {
		Self::from(<[_; 2]>::to(v))
	}
}
impl Cast<Vec3> for glm::Vec3 {
	fn to(v: Vec3) -> Self {
		Self::from(<[_; 3]>::to(v))
	}
}
impl Cast<Vec4> for glm::Vec4 {
	fn to(v: Vec4) -> Self {
		Self::from(<[_; 4]>::to(v))
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
