pub use {apply::*, math::*, swizzle::s::*};

pub trait TupleAllAny {
	fn all(&self) -> bool;
	fn any(&self) -> bool;
}
pub trait TupleIdentity: Default + Copy {
	fn one() -> Self;
	#[inline(always)]
	fn zero() -> Self {
		Self::default()
	}
}

impl TupleAllAny for (bool, bool) {
	#[inline(always)]
	fn all(&self) -> bool {
		self.0 && self.1
	}
	#[inline(always)]
	fn any(&self) -> bool {
		self.0 || self.1
	}
}
impl TupleAllAny for (bool, bool, bool) {
	#[inline(always)]
	fn all(&self) -> bool {
		self.0 && self.1 && self.2
	}
	#[inline(always)]
	fn any(&self) -> bool {
		self.0 || self.1 || self.2
	}
}
impl TupleAllAny for (bool, bool, bool, bool) {
	#[inline(always)]
	fn all(&self) -> bool {
		self.0 && self.1 && self.2 && self.3
	}
	#[inline(always)]
	fn any(&self) -> bool {
		self.0 || self.1 || self.2 || self.3
	}
}

impl<T: ToZero> TupleIdentity for vec2<T> {
	#[inline(always)]
	fn one() -> Self {
		Self::to(1)
	}
}
impl<T: ToZero> TupleIdentity for vec3<T> {
	#[inline(always)]
	fn one() -> Self {
		Self::to(1)
	}
}
impl<T: ToZero> TupleIdentity for vec4<T> {
	#[inline(always)]
	fn one() -> Self {
		Self::to(1)
	}
}
impl<T: ToZero, const N: usize> TupleIdentity for [T; N]
where
	Self: Default,
{
	#[inline(always)]
	fn one() -> Self {
		[T::to(1); N]
	}
}

trait_alias!(ToZero, Cast<u32> + Default + Copy);

mod apply;
mod math;
mod swizzle;

use super::pre::*;
