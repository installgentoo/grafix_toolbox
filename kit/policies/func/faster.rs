use crate::lib::*;

pub trait CharAsStr {
	fn as_str(&self) -> STR;
}
impl CharAsStr for char {
	fn as_str(&self) -> STR {
		LocalStatic!(HashMap<char, Str>).entry(*self).or_insert_with(|| self.to_string().into())
	}
}

pub trait UnwrapValid<R> {
	fn valid(self) -> R;
}
pub trait ValidOption<R> {
	fn as_valid(&self) -> &R;
	fn mut_valid(&mut self) -> &mut R;
}
impl<R> ValidOption<R> for Option<R> {
	#[inline(always)]
	fn as_valid(&self) -> &R {
		self.as_ref().valid()
	}
	#[inline(always)]
	fn mut_valid(&mut self) -> &mut R {
		self.as_mut().valid()
	}
}
impl<R> UnwrapValid<R> for Option<R> {
	#[inline(always)]
	fn valid(self) -> R {
		#[cfg(debug_assertions)]
		{
			self.explain_err(|| "Invalid").fail()
		}
		#[cfg(not(debug_assertions))]
		{
			unsafe { self.unwrap_unchecked() }
		}
	}
}
impl<R, E: Display> UnwrapValid<R> for Result<R, E> {
	#[inline(always)]
	fn valid(self) -> R {
		#[cfg(debug_assertions)]
		{
			self.explain_err(|| "Invalid").fail()
		}
		#[cfg(not(debug_assertions))]
		{
			unsafe { self.unwrap_unchecked() }
		}
	}
}

pub trait FasterIndex<T> {
	fn last_idx(&self) -> usize;
	fn at<I>(&self, idx: I) -> &T
	where
		usize: Cast<I>;
	fn at_mut<I>(&mut self, idx: I) -> &mut T
	where
		usize: Cast<I>;
}
impl<T> FasterIndex<T> for [T] {
	fn last_idx(&self) -> usize {
		self.len().max(1) - 1
	}
	#[inline(always)]
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
	#[inline(always)]
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
}
