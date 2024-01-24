pub use {
	apply::*,
	math::{Tuple2Geometry, TupleComparison, TupleMath, TupleSelf, TupleSigned},
	swizzle::s::*,
};
pub trait TupleAllAny {
	fn all(self) -> bool;
	fn any(self) -> bool;
}
pub trait TupleIdentity: Copy + Default {
	fn one() -> Self;
	fn zero() -> Self {
		Self::default()
	}
}

mod apply;
mod args;
mod math;
mod swizzle;

use {super::*, args::*};

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
