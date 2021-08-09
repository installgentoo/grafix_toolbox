#![allow(clippy::len_without_is_empty, clippy::wrong_self_convention)]
mod apply;
mod args;
mod math;
mod ops;
mod swizzle;
mod traits;

pub use {
	apply::*,
	math::{TupleComparison, TupleMath, TupleSelf, TupleSigned},
	ops::{Tuple2Geometry, TupleAllAny, TupleVecIdentity},
	swizzle::s::*,
};
