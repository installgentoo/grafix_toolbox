use {crate::lib::*, Copy as P};

impl<A, B, T1, T2> Cast<(A, B)> for (T1, T2)
where
	T1: Cast<A>,
	T2: Cast<B>,
{
	#[inline(always)]
	fn to((a, b): (A, B)) -> Self {
		(T1::to(a), T2::to(b))
	}
}
impl<T: P, T1, T2> Cast<&T> for (T1, T2)
where
	Self: Cast<T>,
{
	#[inline(always)]
	fn to(t: &T) -> Self {
		Self::to(*t)
	}
}
impl<T: P, T1, T2> Cast<&mut T> for (T1, T2)
where
	Self: Cast<T>,
{
	#[inline(always)]
	fn to(t: &mut T) -> Self {
		Self::to(*t)
	}
}
macro_rules! impl_tuple2 {
	($($f: ty),+) => {
		$(impl<T1, T2> Cast<$f> for (T1, T2)
		where
			T1: Cast<$f>,
			T2: Cast<$f>,
		{
			#[inline(always)]
			fn to(v: $f) -> Self {
				Self::to((v, v))
			}
		})+
	};
}
impl_tuple2!(bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64, usize, isize);

impl<A, B, C, T1, T2, T3> Cast<(A, B, C)> for (T1, T2, T3)
where
	T1: Cast<A>,
	T2: Cast<B>,
	T3: Cast<C>,
{
	#[inline(always)]
	fn to((a, b, c): (A, B, C)) -> Self {
		(T1::to(a), T2::to(b), T3::to(c))
	}
}
impl<T: P, T1, T2, T3> Cast<&T> for (T1, T2, T3)
where
	Self: Cast<T>,
{
	#[inline(always)]
	fn to(t: &T) -> Self {
		Self::to(*t)
	}
}
impl<T: P, T1, T2, T3> Cast<&mut T> for (T1, T2, T3)
where
	Self: Cast<T>,
{
	#[inline(always)]
	fn to(t: &mut T) -> Self {
		Self::to(*t)
	}
}
macro_rules! impl_tuple3 {
	($($f: ty),+) => {
		$(impl<T1, T2, T3> Cast<$f> for (T1, T2, T3)
		where
			T1: Cast<$f>,
			T2: Cast<$f>,
			T3: Cast<$f>,
		{
			#[inline(always)]
			fn to(v: $f) -> Self {
				Self::to((v, v, v))
			}
		})+
	};
}
impl_tuple3!(bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64, usize, isize);

impl<A, B, C, D, T1, T2, T3, T4> Cast<(A, B, C, D)> for (T1, T2, T3, T4)
where
	T1: Cast<A>,
	T2: Cast<B>,
	T3: Cast<C>,
	T4: Cast<D>,
{
	#[inline(always)]
	fn to((a, b, c, d): (A, B, C, D)) -> Self {
		(T1::to(a), T2::to(b), T3::to(c), T4::to(d))
	}
}
impl<T: P, T1, T2, T3, T4> Cast<&T> for (T1, T2, T3, T4)
where
	Self: Cast<T>,
{
	#[inline(always)]
	fn to(t: &T) -> Self {
		Self::to(*t)
	}
}
impl<T: P, T1, T2, T3, T4> Cast<&mut T> for (T1, T2, T3, T4)
where
	Self: Cast<T>,
{
	#[inline(always)]
	fn to(t: &mut T) -> Self {
		Self::to(*t)
	}
}
macro_rules! impl_tuple4 {
	($($f: ty),+) => {
		$(impl<T1, T2, T3, T4> Cast<$f> for (T1, T2, T3, T4)
		where
			T1: Cast<$f>,
			T2: Cast<$f>,
			T3: Cast<$f>,
			T4: Cast<$f>,
		{
			#[inline(always)]
			fn to(v: $f) -> Self {
				Self::to((v, v, v, v))
			}
		})+
	};
}
impl_tuple4!(bool, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64, usize, isize);

impl<T> Cast<vec4<T>> for vec3<T> {
	#[inline(always)]
	fn to((v1, v2, v3, _): vec4<T>) -> Self {
		(v1, v2, v3)
	}
}
impl<T> Cast<vec4<T>> for vec2<T> {
	#[inline(always)]
	fn to((v1, v2, ..): vec4<T>) -> Self {
		(v1, v2)
	}
}
impl<T> Cast<vec3<T>> for vec2<T> {
	#[inline(always)]
	fn to((v1, v2, _): vec3<T>) -> Self {
		(v1, v2)
	}
}

impl<T: Copy> Cast<&[T]> for vec2<T> {
	#[inline(always)]
	fn to(v: &[T]) -> Self {
		ASSERT!(v.len() == 2, "Vec2 cast from [_][..{}]", v.len());
		(*v.at(0), *v.at(1))
	}
}
impl<T: Copy> Cast<&[T]> for vec3<T> {
	#[inline(always)]
	fn to(v: &[T]) -> Self {
		ASSERT!(v.len() == 3, "Vec3 cast from [_][..{}]", v.len());
		(*v.at(0), *v.at(1), *v.at(2))
	}
}
impl<T: Copy> Cast<&[T]> for vec4<T> {
	#[inline(always)]
	fn to(v: &[T]) -> Self {
		ASSERT!(v.len() == 4, "Vec4 cast from [_][..{}]", v.len());
		(*v.at(0), *v.at(1), *v.at(2), *v.at(3))
	}
}

macro_rules! impl_arr {
	($to: ident, $dim: literal) => {
		impl<T, A> Cast<A> for [T; $dim]
		where
			$to<T>: Cast<A>,
		{
			#[inline(always)]
			fn to(v: A) -> Self {
				$to::to(v).into()
			}
		}
		impl<T, A> Cast<[A; $dim]> for $to<T>
		where
			T: Cast<A>,
		{
			#[inline(always)]
			fn to(v: [A; $dim]) -> Self {
				Self::to($to::<A>::from(v))
			}
		}
	};
}
impl_arr!(vec2, 2);
impl_arr!(vec3, 3);
impl_arr!(vec4, 4);
