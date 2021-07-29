use crate::uses::*;

pub trait TupleArg2<A> {
	fn get2(self) -> (A, A);
}
impl<A, A2, B2> TupleArg2<A> for (A2, B2)
where
	(A, A): Cast<(A2, B2)>,
{
	fn get2(self) -> (A, A) {
		<(A, A)>::to(self)
	}
}
impl<A, T: ToU32> TupleArg2<A> for T
where
	(A, A): Cast<(T, T)>,
{
	fn get2(self) -> (A, A) {
		<(A, A)>::to((self, self))
	}
}

pub trait TupleArg3<A> {
	fn get3(self) -> (A, A, A);
}
impl<A, A2, B2, C2> TupleArg3<A> for (A2, B2, C2)
where
	(A, A, A): Cast<(A2, B2, C2)>,
{
	fn get3(self) -> (A, A, A) {
		<(A, A, A)>::to(self)
	}
}
impl<A, T: ToU32> TupleArg3<A> for T
where
	(A, A, A): Cast<(T, T, T)>,
{
	fn get3(self) -> (A, A, A) {
		<(A, A, A)>::to((self, self, self))
	}
}

pub trait TupleArg4<A> {
	fn get4(self) -> (A, A, A, A);
}
impl<A, A2, B2, C2, D2> TupleArg4<A> for (A2, B2, C2, D2)
where
	(A, A, A, A): Cast<(A2, B2, C2, D2)>,
{
	fn get4(self) -> (A, A, A, A) {
		<(A, A, A, A)>::to(self)
	}
}
impl<A, T: ToU32> TupleArg4<A> for T
where
	(A, A, A, A): Cast<(T, T, T, T)>,
{
	fn get4(self) -> (A, A, A, A) {
		<(A, A, A, A)>::to((self, self, self, self))
	}
}

trait_set! { pub trait ToU32 = Cast<u32> + Copy }
