use crate::uses::*;

pub trait Utf8Len {
	fn utf8_len(&self) -> usize;
	fn len_at_char(&self, i: usize) -> usize;
}
impl Utf8Len for &str {
	fn utf8_len(&self) -> usize {
		self.chars().count()
	}
	fn len_at_char(&self, i: usize) -> usize {
		self.char_indices().skip(i).next().map_or_else(|| self.len(), |(i, _)| i)
	}
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

pub trait ResizeDefault {
	fn resize_def(&mut self, size: usize);
}
impl<T: Default> ResizeDefault for Vec<T> {
	fn resize_def(&mut self, size: usize) {
		if self.len() <= size {
			self.reserve(size);
			for _ in 0..size - self.len() {
				self.push(Def());
			}
		} else {
			self.truncate(size);
		}
	}
}

pub trait Retain_Mut<T> {
	fn retain_mut<F>(&mut self, f: F)
	where
		F: FnMut(&mut T) -> bool;
}
impl<T> Retain_Mut<T> for Vec<T> {
	fn retain_mut<F>(&mut self, mut f: F)
	where
		F: FnMut(&mut T) -> bool,
	{
		let len = self.len();
		let mut del = 0;
		{
			let v = &mut **self;

			for i in 0..len {
				if !f(&mut v[i]) {
					del += 1;
				} else if del > 0 {
					v.swap(i - del, i);
				}
			}
		}
		if del > 0 {
			self.truncate(len - del);
		}
	}
}
