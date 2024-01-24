use super::super::logging;
use std::{collections::HashMap, hash::Hash};

pub trait Utf8Len {
	fn utf8_len(&self) -> usize;
	fn len_at_char(&self, idx: usize) -> usize;
}
impl Utf8Len for &str {
	fn utf8_len(&self) -> usize {
		self.chars().count()
	}
	fn len_at_char(&self, idx: usize) -> usize {
		self.char_indices().nth(idx).map_or_else(|| self.len(), |(i, _)| i)
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
	fn collect_arr<const N: usize>(self) -> [T; N] {
		let vec = self.collect_vec();
		ASSERT!(vec.len() == N, "Collecting into array of wrong length");
		unsafe { vec.try_into().unwrap_unchecked() }
	}
}
impl<V: Sized + Iterator<Item = T>, T> CollectVec<T> for V {}
