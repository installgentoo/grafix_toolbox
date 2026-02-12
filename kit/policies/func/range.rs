use crate::lib::*;

pub trait uRange {
	fn get_range(&self) -> ulVec2;
}

macro_rules! impl_range {
	($($t: ty),+) => {
		$(impl uRange for $t {
			#[inline(always)]
			fn get_range(&self) -> ulVec2 {
				range(self)
			}
		})+
	}
}
macro_rules! impl_range_type {
	($($t: ty),+) => {
		$(impl_range!(ops::Range<$t>, ops::RangeFrom<$t>, ops::RangeInclusive<$t>, ops::RangeTo<$t>, ops::RangeToInclusive<$t>);)+
	}
}
impl_range_type!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

macro_rules! impl_single {
	($($t: ty),+) => {
		$(impl uRange for $t {
			#[inline(always)]
			fn get_range(&self) -> ulVec2 {
				range(&(*self..*self + 1))
			}
		})+
	}
}
impl_single!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl uRange for ops::RangeFull {
	#[inline(always)]
	fn get_range(&self) -> ulVec2 {
		range::<usize>(self)
	}
}

fn range<T: Copy>(r: &impl ops::RangeBounds<T>) -> ulVec2
where
	usize: Cast<T>,
{
	use ops::Bound::{Excluded as E, Included as I, Unbounded as U};
	(
		match r.start_bound() {
			U => usize::MIN,
			I(&i) => usize(i),
			E(&i) => usize(i) + 1,
		},
		match r.end_bound() {
			U => usize::MAX,
			I(&i) => usize(i) + 1,
			E(&i) => usize(i),
		},
	)
}
