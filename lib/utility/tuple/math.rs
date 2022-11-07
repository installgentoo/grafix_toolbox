use super::{apply::*, ops::*, traits::*};
use crate::uses::{ops::*, *};

pub trait TupleMath<RA, A: Math>: TupleApply<RA, A, R<A> = Self> {
	fn clmp<LA>(self, l: LA, r: RA) -> Self
	where
		RA: Cast<LA>,
	{
		self.fmax(RA::to(l)).fmin(r)
	}
	fn mix<M>(self, a: M, r: RA) -> Self
	where
		f32: Cast<M>,
	{
		let a = f32(a);
		self.apply(r, move |l, r| l.mix(a, r))
	}
	fn sum(self, r: RA) -> Self {
		self.apply(r, |l, r| l + r)
	}
	fn sub(self, r: RA) -> Self {
		self.apply(r, |l, r| l - r)
	}
	fn mul(self, r: RA) -> Self {
		self.apply(r, |l, r| l * r)
	}
	fn div(self, r: RA) -> Self {
		self.apply(r, |l, r| l / r)
	}
	fn powi(self, r: RA) -> Self {
		self.apply(r, |l, r| l.power(r))
	}
	fn fmin(self, r: RA) -> Self {
		self.apply(r, |l, r| if l < r { l } else { r })
	}
	fn fmax(self, r: RA) -> Self {
		self.apply(r, |l, r| if l > r { l } else { r })
	}
	fn rem_euc(self, r: RA) -> Self {
		self.apply(r, |l, r| l.euc_mod(r))
	}
}
impl<S: TupleApply<RA, A, R<A> = Self>, RA, A: Math> TupleMath<RA, A> for S {}

pub trait TupleSelf<A: Math>: TupleMap<A, R<A> = Self> + TupleFold<A> + TupleMath<A, A> + TupleIdentity {
	fn round(self) -> Self {
		self.map(|v| v.round())
	}
	fn abs(self) -> Self {
		self.map(|v| v.abs())
	}
	fn sgn(self) -> Self {
		self.map(|v| A::to((v >= Def()) as i32 * 2 - 1))
	}
	fn pow2(self) -> Self {
		self.map(|v| v * v)
	}
	fn mag(self) -> A {
		self.pow2().fold(|l, r| l + r).root()
	}
	fn norm(self) -> Self {
		let l = self.mag();
		self.div(l).or_def(!l.is_zero())
	}
}
impl<S: TupleMap<A, R<A> = Self> + TupleFold<A> + TupleMath<A, A> + TupleIdentity, A: Math> TupleSelf<A> for S {}

pub trait TupleSigned<A: Neg<Output = A>>: TupleMap<A, R<A> = Self> {
	fn neg(self) -> Self {
		self.map(|v| -v)
	}
}
impl<S: TupleMap<A, R<A> = Self>, A: Neg<Output = A>> TupleSigned<A> for S {}

pub trait TupleComparison<B, RA, A: EpsEq>: TupleApply<RA, A, R<bool> = B> {
	fn ls(self, r: RA) -> B {
		self.apply(r, |l, r| l < r)
	}
	fn gt(self, r: RA) -> B {
		self.apply(r, |l, r| l > r)
	}
	fn le(self, r: RA) -> B {
		self.apply(r, |l, r| l <= r)
	}
	fn ge(self, r: RA) -> B {
		self.apply(r, |l, r| l >= r)
	}
	fn eps_eq(self, r: RA) -> B {
		self.apply(r, |l, r| l.eps_eq(r))
	}
	fn trsh_eq(self, r: RA, e: A) -> B {
		self.apply(r, |l, r| l.trsh_eq(r, &e))
	}
}
impl<S: TupleApply<RA, A, R<bool> = (bool, bool)>, RA, A: EpsEq> TupleComparison<(bool, bool), RA, A> for S {}
impl<S: TupleApply<RA, A, R<bool> = (bool, bool, bool)>, RA, A: EpsEq> TupleComparison<(bool, bool, bool), RA, A> for S {}
impl<S: TupleApply<RA, A, R<bool> = (bool, bool, bool, bool)>, RA, A: EpsEq> TupleComparison<(bool, bool, bool, bool), RA, A> for S {}
impl<const N: usize, S: TupleApply<RA, A, R<bool> = [bool; N]>, RA, A: EpsEq> TupleComparison<[bool; N], RA, A> for S {}

trait_set! { pub trait Math = Cast<i32> + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Pow<Self> + EucMod<Self> + Precise + Round + Default }
