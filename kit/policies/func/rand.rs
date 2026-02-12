pub use fastrand::Rng;

pub trait GenRange<T> {
	fn in_range<S: RngExtensions<T>>(&mut self, range: S) -> T;
}
impl<T> GenRange<T> for Rng {
	fn in_range<S: RngExtensions<T>>(&mut self, range: S) -> T {
		range.random(self)
	}
}

macro_rules! impl_rand_f {
	($($t: ty),+) => {
		$(impl RngExtensions<$t> for ops::Range<$t> {
			fn random(self, r: &mut Rng) -> $t {
				(self.start, self.end).random(r)
			}
		})+
	};
}
impl_rand_f!(f16, f32, f64);

macro_rules! impl_rand_i {
	($($t: ident),+) => {
		$(impl RngExtensions<$t> for ops::Range<$t> {
			fn random(self, r: &mut Rng) -> $t {
				r.$t(self.start..self.end)
			}
		})+
	};
}
impl_rand_i!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

pub trait RngExtensions<T> {
	fn random(self, r: &mut Rng) -> T;
}
impl RngExtensions<f16> for hVec2 {
	fn random(self, r: &mut Rng) -> f16 {
		f16(Vec2(self).random(r))
	}
}
impl RngExtensions<f32> for Vec2 {
	fn random(self, r: &mut Rng) -> f32 {
		r.f32() * (self.1 - self.0) - self.0
	}
}
impl RngExtensions<f64> for dVec2 {
	fn random(self, r: &mut Rng) -> f64 {
		r.f64() * (self.1 - self.0) - self.0
	}
}

use crate::lib::*;
