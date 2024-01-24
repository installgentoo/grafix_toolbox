#![allow(unused_imports)]
pub use s::*;

#[cfg(feature = "rng")]
mod s {
	use {super::super::math::*, std::ops::Range};

	pub use fastrand::Rng;
	pub use simdnoise::NoiseBuilder as noise;

	pub trait GenRange<T> {
		fn in_range<S: RngExtensions<T>>(&mut self, range: S) -> T;
	}
	impl<T> GenRange<T> for Rng {
		fn in_range<S: RngExtensions<T>>(&mut self, range: S) -> T {
			range.random(self)
		}
	}
	macro_rules! gen_range_f {
		($t: ident) => {
			impl RngExtensions<$t> for Range<$t> {
				fn random(self, r: &mut Rng) -> $t {
					(self.start, self.end).random(r)
				}
			}
		};
	}
	gen_range_f!(f16);
	gen_range_f!(f32);
	gen_range_f!(f64);
	macro_rules! gen_range_i {
		($t: ident) => {
			impl RngExtensions<$t> for Range<$t> {
				fn random(self, r: &mut Rng) -> $t {
					r.$t(self.start..self.end)
				}
			}
		};
	}
	gen_range_i!(i8);
	gen_range_i!(u8);
	gen_range_i!(i16);
	gen_range_i!(u16);
	gen_range_i!(i32);
	gen_range_i!(u32);
	gen_range_i!(i64);
	gen_range_i!(u64);
	gen_range_i!(i128);
	gen_range_i!(u128);
	gen_range_i!(isize);
	gen_range_i!(usize);

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
}
#[cfg(not(feature = "rng"))]
mod s {}
