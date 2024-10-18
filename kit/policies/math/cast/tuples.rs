use crate::lib::*;

impl<A, B, T1, T2> Cast<(A, B)> for (T1, T2)
where
	T1: Cast<A>,
	T2: Cast<B>,
{
	fn to((a, b): (A, B)) -> Self {
		(T1::to(a), T2::to(b))
	}
}

impl<A, B, C, T1, T2, T3> Cast<(A, B, C)> for (T1, T2, T3)
where
	T1: Cast<A>,
	T2: Cast<B>,
	T3: Cast<C>,
{
	fn to((a, b, c): (A, B, C)) -> Self {
		(T1::to(a), T2::to(b), T3::to(c))
	}
}

impl<A, B, C, D, T1, T2, T3, T4> Cast<(A, B, C, D)> for (T1, T2, T3, T4)
where
	T1: Cast<A>,
	T2: Cast<B>,
	T3: Cast<C>,
	T4: Cast<D>,
{
	fn to((a, b, c, d): (A, B, C, D)) -> Self {
		(T1::to(a), T2::to(b), T3::to(c), T4::to(d))
	}
}

impl<A, T, const N: usize> Cast<[A; N]> for [T; N]
where
	T: Cast<A>,
{
	fn to(a: [A; N]) -> Self {
		a.map(|x| T::to(x))
	}
}

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
	($($t: ty),+) => {
		$(
			array_recast!(vec2<$t>, $t, 2);
			array_recast!(vec3<$t>, $t, 3);
			array_recast!(vec4<$t>, $t, 4);
		)+
	};
}
impl_transmute!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64);
