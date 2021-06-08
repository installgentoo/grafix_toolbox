use super::{args::*, traits::*};
use crate::uses::{cmp::PartialOrd as O, ops::*, *};

pub trait TupleApply<RA, A, R>: Sized {
	type AR;
	fn apply<F: Fn(A, A) -> R>(self, r: RA, op: F) -> Self::AR;
}
pub trait TupleTrans<A>: Sized {
	fn trans<F: Fn(A) -> A>(self, op: F) -> Self;
	fn merge<F: Fn(A, A) -> A>(self, op: F) -> A;
}
pub trait TupleMath<RA, A: TupleMathReq<A>>: TupleApply<RA, A, A, AR = Self> {
	fn clmp<LA>(self, l: LA, r: RA) -> Self
	where
		RA: Cast<LA>,
	{
		self.fmax(RA::to(l)).fmin(r)
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
	fn pow(self, r: RA) -> Self {
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

pub trait TupleSigned<A: TupleMathReq<A> + Rounding + Neg<Output = A>>: TupleTrans<A> {
	fn neg(self) -> Self {
		self.trans(|s| -s)
	}
	fn sgn(self) -> Self {
		self.trans(|s| A::to((s > A::to(0)) as i32) * A::to(2) - A::to(1))
	}
	fn round(self) -> Self {
		self.trans(|s| s.round())
	}
	fn abs(self) -> Self {
		self.trans(|s| s.abs())
	}
}
impl<S: TupleTrans<A>, A: TupleMathReq<A> + Rounding + Neg<Output = A>> TupleSigned<A> for S {}

pub trait TupleSelf<A: TupleMathReq<A>>: TupleTrans<A> + TupleMath<A, A> + TupleVecIdentity + Copy {
	fn pow2(self) -> Self {
		self.trans(|v| v * v)
	}
	fn len(self) -> A {
		self.pow2().merge(|l, r| l + r).root()
	}
	fn norm(self) -> Self {
		let l = self.len();
		self.div(l).or_def(!l.is_zero())
	}
}
impl<S: TupleTrans<A> + TupleTrans<A> + TupleMath<A, A> + TupleVecIdentity + Copy, A: TupleMathReq<A>> TupleSelf<A> for S {}

pub trait TupleMathEext<RA: TupleMath<f32, A>, A: TupleMathReq<A>>: TupleMath<RA, A> + TupleMath<f32, A> {
	fn mix<M>(self, a: M, r: RA) -> Self
	where
		f32: Cast<M>,
	{
		let a = f32::to(a);
		self.mul(1. - a).sum(r.mul(a))
	}
}
impl<S: TupleMath<RA, A> + TupleMath<f32, A>, RA: TupleMath<f32, A>, A: TupleMathReq<A>> TupleMathEext<RA, A> for S {}

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

//TODO trait alias
pub trait TupleMathReq<A>: O + Cast<i32> + Cast<A> + Add<Output = A> + Sub<Output = A> + Mul<Output = A> + Div<Output = A> + Pow<A> + EucMod<A> + Sqrt + Copy {}
impl<A: O + Cast<i32> + Cast<A> + Add<Output = A> + Sub<Output = A> + Mul<Output = A> + Div<Output = A> + Pow<A> + EucMod<A> + Sqrt + Copy> TupleMathReq<A> for A {}

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
impl<RA, A: O + EpsilonEqual> TupleComparison<RA, A> for (A, A)
where
	Self: TupleApply<RA, A, bool, AR = vec2<bool>>,
{
	type B = vec2<bool>;
	impl_comparison!();
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
impl<RA, A: O + EpsilonEqual> TupleComparison<RA, A> for (A, A, A)
where
	Self: TupleApply<RA, A, bool, AR = vec3<bool>>,
{
	type B = vec3<bool>;
	impl_comparison!();
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
impl<RA, A: O + EpsilonEqual> TupleComparison<RA, A> for (A, A, A, A)
where
	Self: TupleApply<RA, A, bool, AR = vec4<bool>>,
{
	type B = vec4<bool>;
	impl_comparison!();
}

pub trait Tuple2Geometry<A> {
	fn rotate(self, def: A) -> Self;
}
impl<T: TupleArg2<f32>, A> Tuple2Geometry<A> for T
where
	f32: Cast<A>,
	Self: Cast<glm::Vec2>,
{
	fn rotate(self, deg: A) -> Self {
		let rad = std::f32::consts::FRAC_PI_2 * f32::to(deg);
		let rot = na::Rotation2::new(rad);
		Self::to(rot * glm::Vec2::to(self.get2()))
	}
}
