use super::{apply::*, ops::*, traits::*};
use crate::uses::{ops::*, *};

pub trait TupleMath<RA, A: TupleMathReq<A>>: TupleApply<RA, A, A, AR = Self> {
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
impl<S: TupleApply<RA, A, A, AR = Self>, RA, A: TupleMathReq<A>> TupleMath<RA, A> for S {}

pub trait TupleSelf<A: TupleMathReq<A>>: TupleTrans<A> + TupleMath<A, A> + TupleVecIdentity {
	fn round(self) -> Self {
		self.map(|v| v.round())
	}
	fn abs(self) -> Self {
		self.map(|v| v.abs())
	}
	fn sgn(self) -> Self {
		self.map(|v| A::to((v > A::to(0)) as i32) * A::to(2) - A::to(1))
	}
	fn pow2(self) -> Self {
		self.map(|v| v * v)
	}
	fn len(self) -> A {
		self.pow2().fold(|l, r| l + r).root()
	}
	fn norm(self) -> Self {
		let l = self.len();
		self.div(l).or_def(!l.is_zero())
	}
}
impl<S: TupleTrans<A> + TupleTrans<A> + TupleMath<A, A> + TupleVecIdentity, A: TupleMathReq<A>> TupleSelf<A> for S {}

pub trait TupleSigned<A: Neg<Output = A>>: TupleTrans<A> {
	fn neg(self) -> Self {
		self.map(|v| -v)
	}
}
impl<S: TupleTrans<A>, A: Neg<Output = A>> TupleSigned<A> for S {}

pub trait TupleComparison<RA, A>: TupleApply<RA, A, bool> {
	type B;
	fn ls(self, r: RA) -> Self::B;
	fn gt(self, r: RA) -> Self::B;
	fn le(self, r: RA) -> Self::B;
	fn ge(self, r: RA) -> Self::B;
	fn eps_eq(self, r: RA) -> Self::B;
	fn eps_eq_c(self, r: RA, e: A) -> Self::B;
}
macro_rules! impl_comparison {
	() => {
		fn ls(self, r: RA) -> Self::B {
			self.apply(r, |l, r| l < r)
		}
		fn gt(self, r: RA) -> Self::B {
			self.apply(r, |l, r| l > r)
		}
		fn le(self, r: RA) -> Self::B {
			self.apply(r, |l, r| l <= r)
		}
		fn ge(self, r: RA) -> Self::B {
			self.apply(r, |l, r| l >= r)
		}
		fn eps_eq(self, r: RA) -> Self::B {
			self.apply(r, |l, r| l.eps_eq(r))
		}
		fn eps_eq_c(self, r: RA, e: A) -> Self::B {
			self.apply(r, |l, r| l.eps_eq_c(r, &e))
		}
	};
}

impl<RA, A: EpsilonEq> TupleComparison<RA, A> for (A, A)
where
	Self: TupleApply<RA, A, bool, AR = vec2<bool>>,
{
	type B = vec2<bool>;
	impl_comparison!();
}

impl<RA, A: EpsilonEq> TupleComparison<RA, A> for (A, A, A)
where
	Self: TupleApply<RA, A, bool, AR = vec3<bool>>,
{
	type B = vec3<bool>;
	impl_comparison!();
}

impl<RA, A: EpsilonEq> TupleComparison<RA, A> for (A, A, A, A)
where
	Self: TupleApply<RA, A, bool, AR = vec4<bool>>,
{
	type B = vec4<bool>;
	impl_comparison!();
}

trait_set! { pub trait TupleMathReq<A> = Cast<i32> + Cast<A> + Add<Output = A> + Sub<Output = A> + Mul<Output = A> + Div<Output = A> + Pow<A> + EucMod<A> + Precise + Round }
