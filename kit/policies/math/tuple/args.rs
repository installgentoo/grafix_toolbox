use super::super::pre::Cast;

pub trait Tuple2<R> {
	fn get(self) -> (R, R);
}
impl<R, A, B> Tuple2<R> for (A, B)
where
	(R, R): Cast<Self>,
{
	fn get(self) -> (R, R) {
		<_>::to(self)
	}
}
impl<R, T: ToZero> Tuple2<R> for T
where
	(R, R): Cast<(T, T)>,
{
	fn get(self) -> (R, R) {
		<_>::to((self, self))
	}
}

pub trait Tuple3<R> {
	fn get(self) -> (R, R, R);
}
impl<R, A, B, C> Tuple3<R> for (A, B, C)
where
	(R, R, R): Cast<Self>,
{
	fn get(self) -> (R, R, R) {
		<_>::to(self)
	}
}
impl<R, T: ToZero> Tuple3<R> for T
where
	(R, R, R): Cast<(T, T, T)>,
{
	fn get(self) -> (R, R, R) {
		<_>::to((self, self, self))
	}
}

pub trait Tuple4<R> {
	fn get(self) -> (R, R, R, R);
}
impl<R, A, B, C, D> Tuple4<R> for (A, B, C, D)
where
	(R, R, R, R): Cast<Self>,
{
	fn get(self) -> (R, R, R, R) {
		<_>::to(self)
	}
}
impl<R, T: ToZero> Tuple4<R> for T
where
	(R, R, R, R): Cast<(T, T, T, T)>,
{
	fn get(self) -> (R, R, R, R) {
		<_>::to((self, self, self, self))
	}
}

pub trait TupleA<R, const N: usize> {
	fn get(self) -> [R; N];
}
impl<R, T, const N: usize> TupleA<R, N> for [T; N]
where
	[R; N]: Cast<Self>,
{
	fn get(self) -> [R; N] {
		<_>::to(self)
	}
}
impl<R, T: ToZero, const N: usize> TupleA<R, N> for T
where
	R: Cast<T>,
{
	fn get(self) -> [R; N] {
		[self; N].map(|x| R::to(x))
	}
}

trait_alias!(pub(super) ToZero, Cast<u32> + Copy + Default);
