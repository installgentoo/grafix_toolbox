use crate::lib::*;

#[derive(Debug)]
pub struct ArcSlice<T> {
	s: Arc<Box<[T]>>,
	range: ulVec2,
}
impl<T> ArcSlice<T> {
	pub fn is_unique(&self) -> bool {
		Arc::strong_count(&self.s) == 1
	}
	pub fn slice(&self, range: impl uRange) -> Self {
		let Self { s, range: (f_b, f_e) } = self;
		let (b, e) = range.get_range();
		let b = f_b + b;
		let e = f_e.or_val(e == usize::MAX, || f_b + e);
		Self { s: s.clone(), range: (b, e) }
	}
	pub fn try_take(self) -> Option<Vec<T>> {
		let Self { s, range: (b, e) } = self;
		let mut v = Arc::into_inner(s)?.into_vec();
		v.truncate(e);
		if b == 0 { Some(v) } else { v.split_off(b).pipe(Some) }
	}
	pub fn try_merge(self, r: Self) -> Result<Self, (Self, Self)> {
		if !self.is_unique() || !r.is_unique() {
			return Err((self, r));
		}

		let (Self { mut s, range: (b, e) }, mut r) = (self, r.try_take().fail());

		{
			let (mut t, s) = (Def(), Arc::get_mut(&mut s).fail());
			mem::swap(s, &mut t);
			let mut v = t.into_vec();
			v.truncate(e);
			let mut v = if b == 0 { v } else { v.split_off(b) };
			v.append(&mut r);
			let mut t = v.into();
			mem::swap(s, &mut t);
		}

		let range = (0, s.len());

		Self { s, range }.pipe(Ok)
	}
	pub fn try_join(&self, r: &Self) -> Option<Self> {
		let Self { ref s, range: (l_b, l_e) } = *self;
		let Self { s: ref r, range: (r_b, r_e) } = *r;

		if !Arc::ptr_eq(s, r) {
			None?
		}

		let range = match (l_e == r_b, r_e == l_b) {
			(true, _) => (l_b, r_e),
			(_, true) => (r_b, l_e),
			_ => None?,
		};

		Self { s: s.clone(), range }.pipe(Some)
	}
}
impl<T> AsRef<[T]> for ArcSlice<T> {
	fn as_ref(&self) -> &[T] {
		self
	}
}
impl<T> ops::Deref for ArcSlice<T> {
	type Target = [T];

	fn deref(&self) -> &[T] {
		let Self { ref s, range: (b, e) } = *self;
		&s[b..e]
	}
}
impl<T> Clone for ArcSlice<T> {
	fn clone(&self) -> Self {
		let Self { ref s, range } = *self;
		Self { s: s.clone(), range }
	}
}
impl<T: Debug> Display for ArcSlice<T> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		write!(f, "Arc{:?}", &**self)
	}
}
impl<T, S: Into<Box<[T]>>> From<S> for ArcSlice<T> {
	fn from(s: S) -> Self {
		let s = s.into();
		let (len, s) = (s.len(), Arc(s));
		Self { s, range: (0, len) }
	}
}
