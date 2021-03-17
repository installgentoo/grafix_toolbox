use crate::uses::{cmp::PartialOrd as O, ops::*, *};

pub trait TupleApply<T, A, R>: Sized {
	type AR;
	fn apply<F: Fn(A, A) -> R>(self, r: T, op: F) -> Self::AR;
	fn transform<F: Fn(A) -> A>(self, op: F) -> Self;
}

pub trait TupleMath<T, A>: TupleApply<T, A, A> {
	fn sum(self, r: T) -> Self;
	fn sub(self, r: T) -> Self;
	fn mul(self, r: T) -> Self;
	fn div(self, r: T) -> Self;
	fn fmin(self, r: T) -> Self;
	fn fmax(self, r: T) -> Self;
	fn clmp(self, l: T, r: T) -> Self {
		self.fmax(l).fmin(r)
	}
}

pub trait TupleComparison<T, A>: TupleApply<T, A, bool> {
	type B;
	fn ls(self, r: T) -> Self::B;
	fn gt(self, r: T) -> Self::B;
	fn le(self, r: T) -> Self::B;
	fn ge(self, r: T) -> Self::B;
	fn eps_eq(self, r: T) -> Self::B;
	fn eps_eq_c(self, r: T, e: A) -> Self::B;
}

pub trait TupleSigned<A>: TupleApply<Self, A, A> {
	fn neg(self) -> Self;
	fn round(self) -> Self;
	fn abs(self) -> Self;
	fn sgn(self) -> Self;
}

pub trait TupleAllAny {
	fn all(self) -> bool;
	fn any(self) -> bool;
}

pub trait TupleVecIdentity: Default {
	fn one() -> Self;
	fn zero() -> Self {
		Self::default()
	}
}

pub trait TupleMathExtensions<T: TupleMath<Self, A> + TupleMath<f32, A>, A>: TupleMath<T, A> + TupleMath<f32, A> {
	fn mix<M>(self, a: M, r: T) -> Self
	where
		f32: Cast<M>,
	{
		let a = f32::to(a);
		self.mul(1. - a).sum(r.mul(a))
	}
}
impl<S: TupleMath<T, A> + TupleMath<f32, A>, T: TupleMath<Self, A> + TupleMath<f32, A>, A> TupleMathExtensions<T, A> for S {}

macro_rules! impl_math {
	() => {
		fn sum(self, r: T) -> Self {
			self.apply(r, |l, r| l + r)
		}
		fn sub(self, r: T) -> Self {
			self.apply(r, |l, r| l - r)
		}
		fn mul(self, r: T) -> Self {
			self.apply(r, |l, r| l * r)
		}
		fn div(self, r: T) -> Self {
			self.apply(r, |l, r| l / r)
		}
		fn fmin(self, r: T) -> Self {
			self.apply(r, |l, r| if l < r { l } else { r })
		}
		fn fmax(self, r: T) -> Self {
			self.apply(r, |l, r| if l > r { l } else { r })
		}
	};
}
macro_rules! impl_comparison {
	() => {
		fn ls(self, r: T) -> Self::B {
			self.apply(r, |l, r| l < r)
		}
		fn gt(self, r: T) -> Self::B {
			self.apply(r, |l, r| l > r)
		}
		fn le(self, r: T) -> Self::B {
			self.apply(r, |l, r| l <= r)
		}
		fn ge(self, r: T) -> Self::B {
			self.apply(r, |l, r| l >= r)
		}
		fn eps_eq(self, r: T) -> Self::B {
			self.apply(r, |l, r| l.eps_eq(r))
		}
		fn eps_eq_c(self, r: T, e: A) -> Self::B {
			self.apply(r, |l, r| l.eps_eq_c(r, &e))
		}
	};
}
macro_rules! impl_signed {
	() => {
		fn neg(self) -> Self {
			self.transform(|s| -s)
		}
		fn sgn(self) -> Self {
			self.transform(|s| A::to((s > A::to(0)) as i32) * A::to(2) - A::to(1))
		}
		fn round(self) -> Self {
			self.transform(|s| s.round())
		}
		fn abs(self) -> Self {
			self.transform(|s| s.abs())
		}
	};
}

//TODO trait alias
pub trait TupleMathReq<A>: O + Cast<i32> + Cast<A> + Add<Output = A> + Sub<Output = A> + Mul<Output = A> + Div<Output = A> + Sized {}
impl<A: O + Cast<i32> + Cast<A> + Add<Output = A> + Sub<Output = A> + Mul<Output = A> + Div<Output = A> + Sized> TupleMathReq<A> for A {}

impl<T, A, R> TupleApply<T, A, R> for (A, A)
where
	T: TupleArg2<A>,
{
	type AR = (R, R);
	fn apply<F: Fn(A, A) -> R>(self, r: T, op: F) -> Self::AR {
		let (l, r) = (self, r.get2());
		(op(l.0, r.0), op(l.1, r.1))
	}
	fn transform<F: Fn(A) -> A>(self, op: F) -> Self {
		(op(self.0), op(self.1))
	}
}
impl<T, A: TupleMathReq<A>> TupleMath<T, A> for (A, A)
where
	Self: TupleApply<T, A, A, AR = Self>,
{
	impl_math!();
}
impl<T, A: O + EpsilonEqual> TupleComparison<T, A> for (A, A)
where
	Self: TupleApply<T, A, bool, AR = vec2<bool>>,
{
	type B = vec2<bool>;
	impl_comparison!();
}
impl<A: TupleMathReq<A> + Rounding + Neg<Output = A>> TupleSigned<A> for (A, A)
where
	Self: TupleApply<Self, A, A, AR = Self>,
{
	impl_signed!();
}

impl<T, A, R> TupleApply<T, A, R> for (A, A, A)
where
	T: TupleArg3<A>,
{
	type AR = (R, R, R);
	fn apply<F: Fn(A, A) -> R>(self, r: T, op: F) -> Self::AR {
		let (l, r) = (self, r.get3());
		(op(l.0, r.0), op(l.1, r.1), op(l.2, r.2))
	}
	fn transform<F: Fn(A) -> A>(self, op: F) -> Self {
		(op(self.0), op(self.1), op(self.2))
	}
}
impl<T, A: TupleMathReq<A>> TupleMath<T, A> for (A, A, A)
where
	Self: TupleApply<T, A, A, AR = Self>,
{
	impl_math!();
}
impl<T, A: O + EpsilonEqual> TupleComparison<T, A> for (A, A, A)
where
	Self: TupleApply<T, A, bool, AR = vec3<bool>>,
{
	type B = vec3<bool>;
	impl_comparison!();
}
impl<A: TupleMathReq<A> + Rounding + Neg<Output = A>> TupleSigned<A> for (A, A, A)
where
	Self: TupleApply<Self, A, A, AR = Self>,
{
	impl_signed!();
}

impl<T, A, R> TupleApply<T, A, R> for (A, A, A, A)
where
	T: TupleArg4<A>,
{
	type AR = (R, R, R, R);
	fn apply<F: Fn(A, A) -> R>(self, r: T, op: F) -> Self::AR {
		let (l, r) = (self, r.get4());
		(op(l.0, r.0), op(l.1, r.1), op(l.2, r.2), op(l.3, r.3))
	}
	fn transform<F: Fn(A) -> A>(self, op: F) -> Self {
		(op(self.0), op(self.1), op(self.2), op(self.3))
	}
}
impl<T, A: TupleMathReq<A>> TupleMath<T, A> for (A, A, A, A)
where
	Self: TupleApply<T, A, A, AR = Self>,
{
	impl_math!();
}
impl<T, A: O + EpsilonEqual> TupleComparison<T, A> for (A, A, A, A)
where
	Self: TupleApply<T, A, bool, AR = vec4<bool>>,
{
	type B = vec4<bool>;
	impl_comparison!();
}
impl<A: TupleMathReq<A> + Rounding + Neg<Output = A>> TupleSigned<A> for (A, A, A, A)
where
	Self: TupleApply<Self, A, A, AR = Self>,
{
	impl_signed!();
}

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
impl<A, T: Cast<u32> + Copy> TupleArg2<A> for T
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
impl<A, T: Cast<u32> + Copy> TupleArg3<A> for T
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
impl<A, T: Cast<u32> + Copy> TupleArg4<A> for T
where
	(A, A, A, A): Cast<(T, T, T, T)>,
{
	fn get4(self) -> (A, A, A, A) {
		<(A, A, A, A)>::to((self, self, self, self))
	}
}

impl TupleAllAny for (bool, bool) {
	fn all(self) -> bool {
		self.0 && self.1
	}
	fn any(self) -> bool {
		self.0 || self.1
	}
}
impl TupleAllAny for (bool, bool, bool) {
	fn all(self) -> bool {
		self.0 && self.1 && self.2
	}
	fn any(self) -> bool {
		self.0 || self.1 || self.2
	}
}
impl TupleAllAny for (bool, bool, bool, bool) {
	fn all(self) -> bool {
		self.0 && self.1 && self.2 && self.3
	}
	fn any(self) -> bool {
		self.0 || self.1 || self.2 || self.3
	}
}

pub trait Rounding: Sized {
	fn round(self) -> Self {
		self
	}
	fn abs(self) -> Self {
		self
	}
}
impl Rounding for f16 {
	fn round(self) -> Self {
		f16::to(f32::to(self).round())
	}
	fn abs(self) -> Self {
		f16::to(f32::to(self).abs())
	}
}
impl Rounding for f32 {
	fn round(self) -> Self {
		self.round()
	}
	fn abs(self) -> Self {
		self.abs()
	}
}
impl Rounding for f64 {
	fn round(self) -> Self {
		self.round()
	}
	fn abs(self) -> Self {
		self.abs()
	}
}
impl Rounding for i8 {
	fn abs(self) -> Self {
		self.abs()
	}
}
impl Rounding for i16 {
	fn abs(self) -> Self {
		self.abs()
	}
}
impl Rounding for i32 {
	fn abs(self) -> Self {
		self.abs()
	}
}
impl Rounding for i64 {
	fn abs(self) -> Self {
		self.abs()
	}
}
impl Rounding for i128 {
	fn abs(self) -> Self {
		self.abs()
	}
}
impl Rounding for isize {
	fn abs(self) -> Self {
		self.abs()
	}
}

pub trait EpsilonEqual {
	fn eps_eq(self, r: Self) -> bool;
	fn eps_eq_c(self, r: Self, e: &Self) -> bool;
}
impl EpsilonEqual for f16 {
	fn eps_eq(self, r: Self) -> bool {
		self.eps_eq_c(r, &f16::EPSILON)
	}
	fn eps_eq_c(self, r: Self, e: &Self) -> bool {
		let (l, r) = vec2::<f32>::to((self, r));
		(l - r).abs() <= f32::to(*e)
	}
}
impl EpsilonEqual for f32 {
	fn eps_eq(self, r: Self) -> bool {
		self.eps_eq_c(r, &f32::EPSILON)
	}
	fn eps_eq_c(self, r: Self, e: &Self) -> bool {
		(self - r).abs() <= *e
	}
}
impl EpsilonEqual for f64 {
	fn eps_eq(self, r: Self) -> bool {
		self.eps_eq_c(r, &f64::EPSILON)
	}
	fn eps_eq_c(self, r: Self, e: &Self) -> bool {
		(self - r).abs() <= *e
	}
}

impl<T: Cast<u32> + Default> TupleVecIdentity for vec2<T> {
	fn one() -> Self {
		Self::to((1, 1))
	}
}
impl<T: Cast<u32> + Default> TupleVecIdentity for vec3<T> {
	fn one() -> Self {
		Self::to((1, 1, 1))
	}
}
impl<T: Cast<u32> + Default> TupleVecIdentity for vec4<T> {
	fn one() -> Self {
		Self::to((1, 1, 1, 1))
	}
}
