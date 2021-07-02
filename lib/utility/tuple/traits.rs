use crate::{impl_trait_for, uses::*};

pub trait TupleAllAny {
	fn all(self) -> bool;
	fn any(self) -> bool;
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

pub trait TupleVecIdentity: Default {
	fn one() -> Self;
	fn zero() -> Self {
		Def()
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

pub trait Round: Sized {
	fn round(self) -> Self {
		self
	}
	fn abs(self) -> Self {
		self
	}
}
impl_trait_for!(Round, u8, u16, u32, u64, u128, usize);
macro_rules! rounding {
	($t: ty) => {
		impl Round for $t {
			fn abs(self) -> Self {
				self.abs()
			}
		}
	};
}
rounding!(i8);
rounding!(i16);
rounding!(i32);
rounding!(i64);
rounding!(i128);
rounding!(isize);
impl Round for f16 {
	fn round(self) -> Self {
		f16::to(f32::to(self).round())
	}
	fn abs(self) -> Self {
		f16::to(f32::to(self).abs())
	}
}
macro_rules! rounding_f {
	($t: ty) => {
		impl Round for $t {
			fn round(self) -> Self {
				self.round()
			}
			fn abs(self) -> Self {
				self.abs()
			}
		}
	};
}
rounding_f!(f32);
rounding_f!(f64);

pub trait Pow<T> {
	fn power(self, _: T) -> Self;
}
macro_rules! pow {
	($t: ty) => {
		impl<T> Pow<T> for $t
		where
			u32: Cast<T>,
		{
			fn power(self, r: T) -> Self {
				self.pow(u32::to(r))
			}
		}
	};
}
pow!(i8);
pow!(i16);
pow!(i32);
pow!(i64);
pow!(i128);
pow!(isize);
pow!(u8);
pow!(u16);
pow!(u32);
pow!(u64);
pow!(u128);
pow!(usize);
impl<T> Pow<T> for f16
where
	i32: Cast<T>,
{
	fn power(self, r: T) -> Self {
		f16::to(f32::to(self).powi(i32::to(r)))
	}
}
macro_rules! powi {
	($t: ty) => {
		impl<T> Pow<T> for $t
		where
			i32: Cast<T>,
		{
			fn power(self, r: T) -> Self {
				self.powi(i32::to(r))
			}
		}
	};
}
powi!(f32);
powi!(f64);

pub trait EucMod<T> {
	fn euc_mod(self, _: T) -> Self;
}
macro_rules! euc_mod {
	($t: ty) => {
		impl<T> EucMod<T> for $t
		where
			Self: Cast<T>,
		{
			fn euc_mod(self, r: T) -> Self {
				self.rem_euclid(Self::to(r))
			}
		}
	};
}
euc_mod!(i8);
euc_mod!(i16);
euc_mod!(i32);
euc_mod!(i64);
euc_mod!(i128);
euc_mod!(isize);
euc_mod!(u8);
euc_mod!(u16);
euc_mod!(u32);
euc_mod!(u64);
euc_mod!(u128);
euc_mod!(usize);
impl<T> EucMod<T> for f16
where
	f32: Cast<T>,
{
	fn euc_mod(self, r: T) -> Self {
		f16::to(self.to_f32().rem_euclid(f32::to(r)))
	}
}
euc_mod!(f32);
euc_mod!(f64);

pub trait Precise {
	fn mix(self, a: f32, r: Self) -> Self;
	fn root(self) -> Self;
	fn is_zero(self) -> bool;
}
macro_rules! sqrt {
	($t: ty) => {
		impl Precise for $t {
			fn mix(self, a: f32, r: Self) -> Self {
				Self::to(f32::to(self) * (1. - a) + f32::to(r) * a)
			}
			fn root(self) -> Self {
				Self::to(f32::to(self).sqrt())
			}
			fn is_zero(self) -> bool {
				f32::to(self) == 0.
			}
		}
	};
}
sqrt!(i8);
sqrt!(i16);
sqrt!(i32);
sqrt!(i64);
sqrt!(i128);
sqrt!(isize);
sqrt!(u8);
sqrt!(u16);
sqrt!(u32);
sqrt!(u64);
sqrt!(u128);
sqrt!(usize);
sqrt!(f16);
sqrt!(f32);
impl Precise for f64 {
	fn mix(self, a: f32, r: Self) -> Self {
		let a = f64::to(a);
		self * (1. - a) + r * a
	}
	fn root(self) -> Self {
		self.sqrt()
	}
	fn is_zero(self) -> bool {
		self == 0.
	}
}
