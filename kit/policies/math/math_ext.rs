pub use ops::Neg;

trait_alias!(
	pub Number,
	Cast<i32>
		+ ops::Add<Output = Self>
		+ ops::Mul<Output = Self>
		+ ops::Div<Output = Self>
		+ EucMod<Self>
		+ Pow<Self>
		+ Precise
		+ Round
);

pub trait EpsEq: Copy + PartialOrd + ops::Sub<Output = Self> {
	#[inline(always)]
	fn eps_eq(self, r: Self) -> bool {
		self == r
	}
	#[inline(always)]
	fn trsh_eq(self, r: Self, t: Self) -> bool {
		let d = if self >= r { self - r } else { r - self };
		d < t || d.eps_eq(t)
	}
}
impl_trait_for!(EpsEq = u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
macro_rules! impl_eps_eq {
	($(($t: ty, $i: ty, $b: literal)),+) => {
		$(impl EpsEq for $t {
			#[inline(always)]
			fn eps_eq(self, r: Self) -> bool {
				let (l, o) = (self, Self::default());

				if l == r {
					return true;
				}

				if l == o || r == o {
					return (l - r).abs() <= Self::EPSILON * Self::to($b);
				}

				if (l < o) != (r < o) {
					return false;
				}

				let (l, r) = (Self::to_bits(l) << 1 >> 1, Self::to_bits(r) << 1 >> 1);
				unsafe { (l as $i).unchecked_sub(r as $i) }.abs() <= $b
			}
		})+
	}
}
impl_eps_eq!((f16, i16, 2), (f32, i32, 4), (f64, i64, 8));

pub trait Precise: EpsEq + Default {
	fn mix(self, r: Self, a: f32) -> Self;
	fn root(self) -> Self;
	#[inline(always)]
	fn is_zero(&self) -> bool {
		self.eps_eq(<_>::default())
	}
}
macro_rules! impl_sqrt {
	($($t: ty),+) => {
		$(impl Precise for $t {
			#[inline(always)]
			fn mix(self, r: Self, a: f32) -> Self {
				Self::to(f32(self) * (1. - a) + f32(r) * a)
			}
			#[inline(always)]
			fn root(self) -> Self {
				Self::to(f32(self).sqrt())
			}
		})+
	}
}
impl_sqrt!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, usize, isize);
impl Precise for f64 {
	#[inline(always)]
	fn mix(self, r: Self, a: f32) -> Self {
		let a = f64(a);
		self * (1. - a) + r * a
	}
	#[inline(always)]
	fn root(self) -> Self {
		self.sqrt()
	}
}

pub trait Round: Sized {
	#[inline(always)]
	fn round(self) -> Self {
		self
	}
	#[inline(always)]
	fn abs(self) -> Self {
		self
	}
}
impl_trait_for!(Round = u8, u16, u32, u64, u128, usize);
macro_rules! impl_abs {
	($($t: ty),+) => {
		$(impl Round for $t {
			#[inline(always)]
			fn abs(self) -> Self {
				self.abs()
			}
		})+
	}
}
impl_abs!(i8, i16, i32, i64, i128, isize);
impl Round for f16 {
	#[inline(always)]
	fn round(self) -> Self {
		f16(f32(self).round())
	}
	#[inline(always)]
	fn abs(self) -> Self {
		f16(f32(self).abs())
	}
}
macro_rules! impl_abs_f {
	($($t: ty),+) => {
		$(impl Round for $t {
			#[inline(always)]
			fn round(self) -> Self {
				self.round()
			}
			#[inline(always)]
			fn abs(self) -> Self {
				self.abs()
			}
		})+
	}
}
impl_abs_f!(f32, f64);

pub trait Pow<T> {
	fn power(self, _: T) -> Self;
}
macro_rules! impl_pow {
	($($t: ty),+) => {
		$(impl<T> Pow<T> for $t
		where
			u32: Cast<T>,
		{
			#[inline(always)]
			fn power(self, r: T) -> Self {
				self.pow(u32(r))
			}
		})+
	}
}
impl_pow!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl<T> Pow<T> for f16
where
	i32: Cast<T>,
{
	#[inline(always)]
	fn power(self, r: T) -> Self {
		f16(f32(self).powi(i32(r)))
	}
}
macro_rules! impl_powi {
	($($t: ty),+) => {
		$(impl<T> Pow<T> for $t
		where
			i32: Cast<T>,
		{
			#[inline(always)]
			fn power(self, r: T) -> Self {
				self.powi(i32(r))
			}
		})+
	}
}
impl_powi!(f32, f64);

pub trait EucMod<T> {
	fn euc_mod(self, _: T) -> Self;
}
macro_rules! impl_euc_mod {
	($($t: ty),+) => {
		$(impl<T> EucMod<T> for $t
		where
			Self: Cast<T>,
		{
			#[inline(always)]
			fn euc_mod(self, r: T) -> Self {
				self.rem_euclid(Self::to(r))
			}
		})+
	}
}
impl_euc_mod!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, usize, isize);
impl<T> EucMod<T> for f16
where
	f32: Cast<T>,
{
	#[inline(always)]
	fn euc_mod(self, r: T) -> Self {
		f16(self.to_f32().rem_euclid(f32(r)))
	}
}

use {super::pre::*, std::ops};
