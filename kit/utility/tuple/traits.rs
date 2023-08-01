use crate::uses::*;

pub trait EpsEq: Copy + cmp::PartialOrd {
	fn eps_eq(self, r: Self) -> bool {
		self == r
	}
	fn trsh_eq(self, r: Self, _: &Self) -> bool {
		self == r
	}
}
impl_trait_for!(EpsEq = bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl EpsEq for f16 {
	fn eps_eq(self, r: Self) -> bool {
		self.trsh_eq(r, &f16::EPSILON)
	}
	fn trsh_eq(self, r: Self, e: &Self) -> bool {
		let (l, r) = Vec2((self, r));
		(l - r).abs() <= f32(*e)
	}
}
impl EpsEq for f32 {
	fn eps_eq(self, r: Self) -> bool {
		self.trsh_eq(r, &f32::EPSILON)
	}
	fn trsh_eq(self, r: Self, e: &Self) -> bool {
		(self - r).abs() <= *e
	}
}
impl EpsEq for f64 {
	fn eps_eq(self, r: Self) -> bool {
		self.trsh_eq(r, &f64::EPSILON)
	}
	fn trsh_eq(self, r: Self, e: &Self) -> bool {
		(self - r).abs() <= *e
	}
}

pub trait Round: Copy + cmp::PartialOrd {
	fn round(self) -> Self {
		self
	}
	fn abs(self) -> Self {
		self
	}
}
impl_trait_for!(Round = u8, u16, u32, u64, u128, usize);
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
		f16(f32(self).round())
	}
	fn abs(self) -> Self {
		f16(f32(self).abs())
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
				self.pow(u32(r))
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
		f16(f32(self).powi(i32(r)))
	}
}
macro_rules! powi {
	($t: ty) => {
		impl<T> Pow<T> for $t
		where
			i32: Cast<T>,
		{
			fn power(self, r: T) -> Self {
				self.powi(i32(r))
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
		f16(self.to_f32().rem_euclid(f32(r)))
	}
}
euc_mod!(f32);
euc_mod!(f64);

pub trait Precise {
	fn mix(self, a: f32, r: Self) -> Self;
	fn root(self) -> Self;
	#[allow(clippy::wrong_self_convention)]
	fn is_zero(self) -> bool;
}
macro_rules! sqrt {
	($t: ty) => {
		impl Precise for $t {
			fn mix(self, a: f32, r: Self) -> Self {
				Self::to(f32(self) * (1. - a) + f32(r) * a)
			}
			fn root(self) -> Self {
				Self::to(f32(self).sqrt())
			}
			fn is_zero(self) -> bool {
				f32(self) == 0.
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
		let a = f64(a);
		self * (1. - a) + r * a
	}
	fn root(self) -> Self {
		self.sqrt()
	}
	fn is_zero(self) -> bool {
		self == 0.
	}
}
