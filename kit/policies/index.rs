use super::math::*;

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
