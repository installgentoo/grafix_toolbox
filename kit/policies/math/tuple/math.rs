use super::{ext::*, *};

pub trait TupleMath<RA, A: Number>: TupleApply<RA, A, R<A> = Self> {
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
impl<S: TupleApply<RA, A, R<A> = Self>, RA, A: Number> TupleMath<RA, A> for S {}

pub trait TupleSelf<A: Number>: TupleMap<A, R<A> = Self> + TupleFold<A> + TupleMath<A, A> + TupleIdentity {
	fn round(self) -> Self {
		self.map(|v| v.round())
	}
	fn abs(self) -> Self {
		self.map(|v| v.abs())
	}
	fn sgn(self) -> Self {
		self.map(|v| A::to((v >= A::default()) as i32 * 2 - 1))
	}
	fn pow2(self) -> Self {
		self.map(|v| v * v)
	}
	fn mag(self) -> A {
		self.pow2().fold(|l, r| l + r).root()
	}
	fn norm(self) -> Self {
		let l = self.mag();
		if l.is_zero() {
			Self::default()
		} else {
			self.div(l)
		}
	}
}
impl<S: TupleMap<A, R<A> = Self> + TupleFold<A> + TupleMath<A, A> + TupleIdentity, A: Number> TupleSelf<A> for S {}

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

pub trait Tuple2Geometry<A> {
	fn rotate(self, deg: A) -> Self;
}
impl<T: Tuple2<f32>, A> Tuple2Geometry<A> for T
where
	f32: Cast<A>,
	Self: Cast<la::Vec2>,
{
	fn rotate(self, deg: A) -> Self {
		let rad = std::f32::consts::FRAC_PI_2 * f32(deg);
		let rot = la::na::Rotation2::new(rad);
		Self::to(rot * la::Vec2::to(self.get()))
	}
}
