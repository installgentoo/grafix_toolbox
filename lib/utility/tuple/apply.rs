use super::args::*;

pub trait TupleApply<RA, A, R>: Sized {
	type AR;
	fn apply<F: Fn(A, A) -> R>(self, r: RA, op: F) -> Self::AR;
}
pub trait TupleTrans<A>: Sized {
	fn trans<F: Fn(A) -> A>(self, op: F) -> Self;
	fn merge<F: Fn(A, A) -> A>(self, op: F) -> A;
}

impl<RA, A, R> TupleApply<RA, A, R> for (A, A)
where
	RA: TupleArg2<A>,
{
	type AR = (R, R);
	fn apply<F: Fn(A, A) -> R>(self, r: RA, op: F) -> Self::AR {
		let (l, r) = (self, r.get2());
		(op(l.0, r.0), op(l.1, r.1))
	}
}
impl<A> TupleTrans<A> for (A, A) {
	fn trans<F: Fn(A) -> A>(self, op: F) -> Self {
		(op(self.0), op(self.1))
	}
	fn merge<F: Fn(A, A) -> A>(self, op: F) -> A {
		op(self.0, self.1)
	}
}

impl<RA, A, R> TupleApply<RA, A, R> for (A, A, A)
where
	RA: TupleArg3<A>,
{
	type AR = (R, R, R);
	fn apply<F: Fn(A, A) -> R>(self, r: RA, op: F) -> Self::AR {
		let (l, r) = (self, r.get3());
		(op(l.0, r.0), op(l.1, r.1), op(l.2, r.2))
	}
}
impl<A> TupleTrans<A> for (A, A, A) {
	fn trans<F: Fn(A) -> A>(self, op: F) -> Self {
		(op(self.0), op(self.1), op(self.2))
	}
	fn merge<F: Fn(A, A) -> A>(self, op: F) -> A {
		op(op(self.0, self.1), self.2)
	}
}

impl<RA, A, R> TupleApply<RA, A, R> for (A, A, A, A)
where
	RA: TupleArg4<A>,
{
	type AR = (R, R, R, R);
	fn apply<F: Fn(A, A) -> R>(self, r: RA, op: F) -> Self::AR {
		let (l, r) = (self, r.get4());
		(op(l.0, r.0), op(l.1, r.1), op(l.2, r.2), op(l.3, r.3))
	}
}
impl<A> TupleTrans<A> for (A, A, A, A) {
	fn trans<F: Fn(A) -> A>(self, op: F) -> Self {
		(op(self.0), op(self.1), op(self.2), op(self.3))
	}
	fn merge<F: Fn(A, A) -> A>(self, op: F) -> A {
		op(op(op(self.0, self.1), self.2), self.3)
	}
}
