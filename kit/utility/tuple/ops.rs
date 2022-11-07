use super::args::*;
use crate::uses::*;

pub trait TupleAllAny {
	fn all(self) -> bool;
	fn any(self) -> bool;
}

pub trait TupleIdentity: Copy + Default {
	fn one() -> Self;
	fn zero() -> Self {
		Def()
	}
}

pub trait Tuple2Geometry<A> {
	fn rotate(self, deg: A) -> Self;
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

impl<T: ToZero> TupleIdentity for vec2<T> {
	fn one() -> Self {
		Self::to((1, 1))
	}
}
impl<T: ToZero> TupleIdentity for vec3<T> {
	fn one() -> Self {
		Self::to((1, 1, 1))
	}
}
impl<T: ToZero> TupleIdentity for vec4<T> {
	fn one() -> Self {
		Self::to((1, 1, 1, 1))
	}
}
impl<T: ToZero, const N: usize> TupleIdentity for [T; N]
where
	Self: Default,
{
	fn one() -> Self {
		Self::to([1; N])
	}
}
trait_set! { pub trait ToZero = Cast<u32> + Copy + Default }

impl<T: Tuple2<f32>, A> Tuple2Geometry<A> for T
where
	f32: Cast<A>,
	Self: Cast<glm::Vec2>,
{
	fn rotate(self, deg: A) -> Self {
		let rad = std::f32::consts::FRAC_PI_2 * f32(deg);
		let rot = na::Rotation2::new(rad);
		Self::to(rot * glm::Vec2::to(self.get()))
	}
}
