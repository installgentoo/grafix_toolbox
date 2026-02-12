use crate::{lib::*, math::*};
use std::hash::Hash;

pub trait Utf8Len {
	fn utf8_len_at<T>(&self, idx: T) -> usize
	where
		usize: Cast<T>;
	fn utf8_count(&self) -> usize;
	fn utf8_slice(&self, r: impl uRange) -> &str;
	fn slice(&self, r: impl uRange) -> &str;
}
impl Utf8Len for str {
	fn utf8_len_at<T>(&self, char_idx: T) -> usize
	where
		usize: Cast<T>,
	{
		let char_idx = usize(char_idx);
		self.char_indices()
			.enumerate()
			.take(char_idx + 1)
			.last()
			.map_or(0, |(n, (i, _))| if n == char_idx { i } else { self.len() })
	}
	fn utf8_count(&self) -> usize {
		self.chars().count()
	}
	fn utf8_slice(&self, r: impl uRange) -> &str {
		let (b, e) = r.get_range();
		let beg = self.utf8_len_at(b);
		if e == usize::MAX {
			return &self[beg..];
		}

		let end = self[beg..].utf8_len_at(e - b);
		&self[beg..beg + end]
	}
	fn slice(&self, r: impl uRange) -> &str {
		let (b, e) = r.get_range().fmin(self.len());
		&self[b..e]
	}
}

pub trait CountItems<T: Eq + Hash>: Sized + Iterator<Item = T> {
	fn map_count(self) -> HashMap<T, usize> {
		let mut map = HashMap::new();
		for i in self {
			*map.entry(i).or_default() += 1;
		}
		map
	}
}
impl<T: Eq + Hash, V: Sized + Iterator<Item = T>> CountItems<T> for V {}

pub trait CollectVec<T>: Sized + Iterator<Item = T> {
	fn collect_vec(self) -> Vec<T> {
		self.collect()
	}
	fn collect_box(self) -> Box<[T]> {
		self.collect()
	}
	fn collect_arr<const N: usize>(self) -> [T; N] {
		let vec = self.collect_vec();
		vec.try_into()
			.map_err(|v: Vec<_>| format!("Cannot collect [_][..{}] into [{}; {N}]", v.len(), type_name::<T>()))
			.fail()
	}
}
impl<V: Sized + Iterator<Item = T>, T> CollectVec<T> for V {}
