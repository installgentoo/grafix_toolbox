pub use ops::Neg;

trait_alias!(
	pub Number,
	Cast<i32>
		+ Default
		+ ops::Add<Output = Self>
		+ ops::Sub<Output = Self>
		+ ops::Mul<Output = Self>
		+ ops::Div<Output = Self>
		+ EpsEq
		+ Round
		+ Pow<Self>
		+ EucMod<Self>
		+ Precise
);

pub trait EpsEq: Copy + cmp::PartialOrd {
	fn eps_eq(self, r: Self) -> bool {
		self == r
	}
	fn trsh_eq(self, r: Self, _: Self) -> bool {
		self == r
	}
}
impl_trait_for!(EpsEq = bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
impl EpsEq for f16 {
	fn eps_eq(self, r: Self) -> bool {
		self.trsh_eq(r, f16::EPSILON)
	}
	fn trsh_eq(self, r: Self, e: Self) -> bool {
		let (l, r) = Vec2((self, r));
		(l - r).abs() <= f32(e)
	}
}
impl EpsEq for f32 {
	fn eps_eq(self, r: Self) -> bool {
		self.trsh_eq(r, f32::EPSILON)
	}
	fn trsh_eq(self, r: Self, e: Self) -> bool {
		(self - r).abs() <= e
	}
}
impl EpsEq for f64 {
	fn eps_eq(self, r: Self) -> bool {
		self.trsh_eq(r, f64::EPSILON)
	}
	fn trsh_eq(self, r: Self, e: Self) -> bool {
		(self - r).abs() <= e
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
	($($t: ty),+) => {
		$(impl Round for $t {
			fn abs(self) -> Self {
				self.abs()
			}
		})+
	};
}
rounding!(i8, i16, i32, i64, i128, isize);
impl Round for f16 {
	fn round(self) -> Self {
		f16(f32(self).round())
	}
	fn abs(self) -> Self {
		f16(f32(self).abs())
	}
}
macro_rules! rounding_f {
	($($t: ty),+) => {
		$(impl Round for $t {
			fn round(self) -> Self {
				self.round()
			}
			fn abs(self) -> Self {
				self.abs()
			}
		})+
	};
}
rounding_f!(f32, f64);

pub trait Pow<T> {
	fn power(self, _: T) -> Self;
}
macro_rules! pow {
	($($t: ty),+) => {
		$(impl<T> Pow<T> for $t
		where
			u32: Cast<T>,
		{
			fn power(self, r: T) -> Self {
				self.pow(u32(r))
			}
		})+
	};
}
pow!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl<T> Pow<T> for f16
where
	i32: Cast<T>,
{
	fn power(self, r: T) -> Self {
		f16(f32(self).powi(i32(r)))
	}
}
macro_rules! powi {
	($($t: ty),+) => {
		$(impl<T> Pow<T> for $t
		where
			i32: Cast<T>,
		{
			fn power(self, r: T) -> Self {
				self.powi(i32(r))
			}
		})+
	};
}
powi!(f32, f64);

pub trait EucMod<T> {
	fn euc_mod(self, _: T) -> Self;
}
macro_rules! euc_mod {
	($($t: ty),+) => {
		$(impl<T> EucMod<T> for $t
		where
			Self: Cast<T>,
		{
			fn euc_mod(self, r: T) -> Self {
				self.rem_euclid(Self::to(r))
			}
		})+
	};
}
euc_mod!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, usize, isize);
impl<T> EucMod<T> for f16
where
	f32: Cast<T>,
{
	fn euc_mod(self, r: T) -> Self {
		f16(self.to_f32().rem_euclid(f32(r)))
	}
}

pub trait Precise: Default + PartialEq {
	fn mix(self, a: f32, r: Self) -> Self;
	fn root(self) -> Self;
	fn is_zero(&self) -> bool {
		self == &Default::default()
	}
}
macro_rules! sqrt {
	($($t: ty),+) => {
		$(impl Precise for $t {
			fn mix(self, a: f32, r: Self) -> Self {
				Self::to(f32(self) * (1. - a) + f32(r) * a)
			}
			fn root(self) -> Self {
				Self::to(f32(self).sqrt())
			}
		})+
	};
}
sqrt!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, usize, isize);
impl Precise for f64 {
	fn mix(self, a: f32, r: Self) -> Self {
		let a = f64(a);
		self * (1. - a) + r * a
	}
	fn root(self) -> Self {
		self.sqrt()
	}
}

use super::pre::*;
use std::{cmp, ops};
