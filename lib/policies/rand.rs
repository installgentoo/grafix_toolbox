use rand::SeedableRng;

pub use rand::distributions::{Distribution, Uniform as uni_dist};
pub use rand::seq::SliceRandom;
pub use rand::Rng as prelude_rng;
pub use rand_xorshift::XorShiftRng as Rng;
pub use simdnoise::NoiseBuilder as noise;

pub mod rng {
	use super::*;
	pub fn new() -> Rng {
		Rng::from_rng(rand::thread_rng()).unwrap()
	}
	pub fn seeded(s: u64) -> Rng {
		Rng::seed_from_u64(s)
	}
}
