use rand::SeedableRng;

pub use rand::distributions::{Distribution, Uniform as uni_dist};
pub use rand::seq::SliceRandom;
pub use rand::Rng as prelude_rng;
pub use rand_xorshift::XorShiftRng as Rng;

pub mod rng {
	use super::*;
	pub fn new() -> Rng {
		Rng::from_rng(rand::thread_rng()).unwrap()
	}
}
