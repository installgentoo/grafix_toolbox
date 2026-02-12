use crate::lib::*;

pub trait TupleApply<RA, A>: Sized {
	type R<B>;
	fn apply<B, F: Fn(A, A) -> B>(self, r: RA, op: F) -> Self::R<B>;
}
pub trait TupleMap<A>: Sized {
	type R<B>;
	fn map<B, F: Fn(A) -> B>(self, op: F) -> Self::R<B>;
}
pub trait TupleFold<A>: Sized {
	fn fold<F: Fn(A, A) -> A>(self, op: F) -> A;
}

impl<RA, A> TupleApply<RA, A> for (A, A)
where
	Self: Cast<RA>,
{
	type R<B> = (B, B);
	#[inline(always)]
	fn apply<B, F: Fn(A, A) -> B>(self, r: RA, op: F) -> Self::R<B> {
		let (l, r) = (self, Self::to(r));
		(op(l.0, r.0), op(l.1, r.1))
	}
}
impl<A> TupleMap<A> for (A, A) {
	type R<B> = (B, B);
	#[inline(always)]
	fn map<B, F: Fn(A) -> B>(self, op: F) -> Self::R<B> {
		(op(self.0), op(self.1))
	}
}
impl<A> TupleFold<A> for (A, A) {
	#[inline(always)]
	fn fold<F: Fn(A, A) -> A>(self, op: F) -> A {
		op(self.0, self.1)
	}
}

impl<RA, A> TupleApply<RA, A> for (A, A, A)
where
	Self: Cast<RA>,
{
	type R<B> = (B, B, B);
	#[inline(always)]
	fn apply<B, F: Fn(A, A) -> B>(self, r: RA, op: F) -> Self::R<B> {
		let (l, r) = (self, Self::to(r));
		(op(l.0, r.0), op(l.1, r.1), op(l.2, r.2))
	}
}
impl<A> TupleMap<A> for (A, A, A) {
	type R<B> = (B, B, B);
	#[inline(always)]
	fn map<B, F: Fn(A) -> B>(self, op: F) -> Self::R<B> {
		(op(self.0), op(self.1), op(self.2))
	}
}
impl<A> TupleFold<A> for (A, A, A) {
	#[inline(always)]
	fn fold<F: Fn(A, A) -> A>(self, op: F) -> A {
		op(op(self.0, self.1), self.2)
	}
}

impl<RA, A> TupleApply<RA, A> for (A, A, A, A)
where
	Self: Cast<RA>,
{
	type R<B> = (B, B, B, B);
	#[inline(always)]
	fn apply<B, F: Fn(A, A) -> B>(self, r: RA, op: F) -> Self::R<B> {
		let (l, r) = (self, Self::to(r));
		(op(l.0, r.0), op(l.1, r.1), op(l.2, r.2), op(l.3, r.3))
	}
}
impl<A> TupleMap<A> for (A, A, A, A) {
	type R<B> = (B, B, B, B);
	#[inline(always)]
	fn map<B, F: Fn(A) -> B>(self, op: F) -> Self::R<B> {
		(op(self.0), op(self.1), op(self.2), op(self.3))
	}
}
impl<A> TupleFold<A> for (A, A, A, A) {
	#[inline(always)]
	fn fold<F: Fn(A, A) -> A>(self, op: F) -> A {
		op(op(op(self.0, self.1), self.2), self.3)
	}
}

impl<RA, A, const N: usize> TupleApply<RA, A> for [A; N]
where
	Self: Cast<RA>,
{
	type R<B> = [B; N];
	#[inline(always)]
	fn apply<B, F: Fn(A, A) -> B>(self, r: RA, op: F) -> Self::R<B> {
		let (l, r) = (self, Self::to(r));
		let Ok(b) = l.into_iter().zip(r).map(|(l, r)| op(l, r)).collect::<Vec<_>>().try_into() else {
			unreachable!()
		};
		b
	}
}
impl<A, const N: usize> TupleMap<A> for [A; N] {
	type R<B> = [B; N];
	#[inline(always)]
	fn map<B, F: Fn(A) -> B>(self, op: F) -> Self::R<B> {
		self.map(op)
	}
}
impl<A, const N: usize> TupleFold<A> for [A; N] {
	#[inline(always)]
	fn fold<F: Fn(A, A) -> A>(self, op: F) -> A {
		let mut i = self.into_iter();
		let h = i.next().valid();
		i.fold(h, op)
	}
}
