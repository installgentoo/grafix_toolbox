use {super::arc_slice::*, crate::lib::*};

#[derive(Debug)]
pub struct VerVec<T> {
	slices: Vec<ArcSlice<T>>,
}

impl<T> VerVec<T> {
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
	pub fn len(&self) -> usize {
		self.slices.iter().map(|s| s.len()).sum()
	}
	pub fn last_idx(&self) -> usize {
		self.len().max(1) - 1
	}
	pub fn at<I>(&self, idx: I) -> &T
	where
		usize: Cast<I>,
	{
		let mut i = usize(idx);
		let at = i;

		let (s, i) = self
			.slices
			.iter()
			.find_map(|s| {
				let len = s.len();
				if i >= len {
					i -= len;
					None?
				}

				Some((s, i))
			})
			.explain_err(|| format!("VerVec access {at} oob, len {}", self.len()))
			.fail();

		unsafe { s.get_unchecked(i) }
	}
	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.slices.iter().flat_map(|s| &**s)
	}
	pub fn try_take(&mut self) -> Option<Vec<T>> {
		let Self { slices } = self;
		if slices.iter().any(|s| !s.is_unique()) {
			None?
		}

		slices.drain(..).flat_map(|s| s.try_take().valid()).collect_vec().pipe(Some)
	}
	pub fn compact(&mut self) {
		let Self { slices } = self;
		let len = slices.len();

		if len <= 1 {
			return;
		}

		let (mut c, mut s) = (slices.drain(..), Vec::with_capacity(len));

		let mut l = c.next().valid();
		for r in c {
			if let Some(c) = l.try_join(&r) {
				l = c;
				continue;
			}

			if l.is_unique() && r.is_unique() {
				let Ok(c) = l.try_merge(r) else { unreachable!() };
				l = c;
				continue;
			}

			s.push(l);
			l = r;
		}

		s.push(l);

		*slices = s;
	}
	pub fn remove(&self, r: impl uRange) -> Self {
		self.replace(r, [])
	}
	pub fn replace(&self, r: impl uRange, patch: impl Into<Box<[T]>>) -> Self {
		let ((b, e), len) = (r.get_range(), || self.len());
		let e = e.or_val(e != usize::MAX, len);
		ASSERT!(e <= len(), "VerVec replace {e} oob, len {}", len());

		let parent = &self.slices;
		let mut patch = patch.pipe(ArcSlice::from).pipe(Some).filter(|s| !s.is_empty());
		let (mut c_b, mut slices) = (0, Vec::with_capacity(parent.capacity()));

		for c in parent {
			let c_e = c_b + c.len();

			if c_e <= b || c_b > e {
				slices.push(c.clone());
			} else {
				if c_b < b {
					slices.push(c.slice(..b - c_b));
				}

				if b >= c_b
					&& b < c_e && let Some(p) = patch.take()
				{
					slices.push(p);
				}

				if e < c_e {
					slices.push(c.slice(e - c_b..));
				}
			}

			c_b = c_e;
		}

		if let Some(p) = patch.take() {
			slices.push(p);
		}

		Self { slices }
	}
}
impl<T: Clone> VerVec<T> {
	pub fn merge(&mut self) {
		if self.slices.len() <= 1 {
			return;
		}

		*self = self.slices.iter().map(|s| &**s).collect_vec().concat().pipe(Self::from);
	}
}
impl<T> Clone for VerVec<T> {
	fn clone(&self) -> Self {
		Self { slices: self.slices.clone() }
	}
}
impl<T> Default for VerVec<T> {
	fn default() -> Self {
		Self { slices: vec![] }
	}
}
impl<T: Debug> Display for VerVec<T> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		write!(f, "VerVec{:?}", self.iter().collect_vec())
	}
}
impl<T, S: Into<Box<[T]>>> From<S> for VerVec<T> {
	fn from(v: S) -> Self {
		let v: ArcSlice<_> = v.into().into();
		let slices = if v.is_empty() { vec![] } else { vec![v] };
		Self { slices }
	}
}
impl<T: Clone> From<VerVec<T>> for Vec<T> {
	fn from(mut v: VerVec<T>) -> Self {
		v.merge();

		if v.slices.is_empty() {
			return vec![];
		}

		v.slices.pop().valid().try_take().fail()
	}
}
