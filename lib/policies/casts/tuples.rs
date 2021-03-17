use super::cast::Cast;

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
