use crate::lib::*;

impl<T: Copy> Cast<vec4<T>> for vec3<T> {
	fn to((v1, v2, v3, _): vec4<T>) -> Self {
		(v1, v2, v3)
	}
}
impl<T: Copy> Cast<vec4<T>> for vec2<T> {
	fn to((v1, v2, _, _): vec4<T>) -> Self {
		(v1, v2)
	}
}
impl<T: Copy> Cast<vec3<T>> for vec2<T> {
	fn to((v1, v2, _): vec3<T>) -> Self {
		(v1, v2)
	}
}

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

macro_rules! array_recast {
	($to: ty, $t: ty, $dim: literal) => {
		impl Cast<$to> for [$t; $dim] {
			fn to(v: $to) -> Self {
				v.into()
			}
		}
		impl Cast<[$t; $dim]> for $to {
			fn to(v: [$t; $dim]) -> Self {
				v.into()
			}
		}
	};
}

macro_rules! impl_transmute {
	($t: ty) => {
		array_recast!(vec2<$t>, $t, 2);
		array_recast!(vec3<$t>, $t, 3);
		array_recast!(vec4<$t>, $t, 4);
	};
}
impl_transmute!(u8);
impl_transmute!(i8);
impl_transmute!(u16);
impl_transmute!(i16);
impl_transmute!(u32);
impl_transmute!(i32);
impl_transmute!(u64);
impl_transmute!(i64);
impl_transmute!(u128);
impl_transmute!(i128);
impl_transmute!(f16);
impl_transmute!(f32);
impl_transmute!(f64);
