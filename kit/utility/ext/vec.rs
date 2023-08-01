use crate::uses::*;

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

pub trait CountItems<T: Eq + hash::Hash>: Sized + Iterator<Item = T> {
	fn map_count(self) -> HashMap<T, usize> {
		let mut map = HashMap::<_, usize>::new();
		for i in self {
			*map.entry(i).or_default() += 1;
		}
		map
	}
}
impl<T: Eq + hash::Hash, V: Sized + Iterator<Item = T>> CountItems<T> for V {}
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

pub trait FasterIndex<T> {
	fn at<I>(&self, idx: I) -> &T
	where
		usize: Cast<I>;
	fn at_mut<I>(&mut self, idx: I) -> &mut T
	where
		usize: Cast<I>;
}
macro_rules! impl_faster_index {
	() => {
		fn at<I>(&self, idx: I) -> &T
		where
			usize: Cast<I>,
		{
			let i = usize(idx);
			#[cfg(debug_assertions)]
			{
				&self[i]
			}
			#[cfg(not(debug_assertions))]
			{
				unsafe { self.get_unchecked(i) }
			}
		}
		fn at_mut<I>(&mut self, idx: I) -> &mut T
		where
			usize: Cast<I>,
		{
			let i = usize(idx);
			#[cfg(debug_assertions)]
			{
				&mut self[i]
			}
			#[cfg(not(debug_assertions))]
			{
				unsafe { self.get_unchecked_mut(i) }
			}
		}
	};
}
impl<T> FasterIndex<T> for Vec<T> {
	impl_faster_index!();
}
impl<T, const L: usize> FasterIndex<T> for [T; L] {
	impl_faster_index!();
}
impl<T> FasterIndex<T> for [T] {
	impl_faster_index!();
}

pub trait LastIdx {
	fn last_idx(&self) -> usize;
}
impl LastIdx for &str {
	fn last_idx(&self) -> usize {
		self.len().max(1) - 1
	}
}
impl<T> LastIdx for &[T] {
	fn last_idx(&self) -> usize {
		self.len().max(1) - 1
	}
}
impl<T> LastIdx for Vec<T> {
	fn last_idx(&self) -> usize {
		self.as_slice().last_idx()
	}
}
