use crate::lib::*;

#[derive(Debug)]
pub struct LazyCellLock<'l, T: SendS> {
	l: &'l mut LazyCell<T>,
}
impl<T: SendS> LazyCellLock<'_, T> {
	pub fn is_busy(&self) -> bool {
		(&self.l.t).bind().is_some()
	}
	pub fn set(&mut self, f: impl FnOnce(&mut T)) {
		let _ = self.as_mut().tap(|s| {
			f(s);
		});
	}
	pub fn update<F: Fut<T>>(&mut self, f: impl FnOnce(&'static T) -> F + SendS) {
		let LazyCell { v, t } = self.l;
		let v = v.weak();
		*t = task::Runtime()
			.spawn(async move || {
				let v = unsafe { v.as_ref() };
				f(v).await
			})
			.pipe(Some)
			.into();
	}
	pub fn pipe<'s, R: 's>(&'s self, f: impl FnOnce(&'s T) -> R) -> R {
		self.as_ref().pipe(f)
	}
}
impl<T: SendS> AsRef<T> for LazyCellLock<'_, T> {
	fn as_ref(&self) -> &T {
		self
	}
}
impl<T: SendS> AsMut<T> for LazyCellLock<'_, T> {
	fn as_mut(&mut self) -> &mut T {
		let Self { l: LazyCell { v, t } } = self;

		if t.bind().is_some() {
			let t = t.take().valid();
			task::Runtime().cancel(t);
		}

		unsafe { &mut *v.ptr() }
	}
}
impl<T: SendS> ops::Deref for LazyCellLock<'_, T> {
	type Target = T;

	fn deref(&self) -> &T {
		unsafe { self.l.v.as_ref() }
	}
}

#[derive(Default)]
pub struct LazyCell<T: SendS> {
	v: Ptr<T>,
	t: Cell<Option<Task<T>>>,
}
impl<T: Default + SendS> LazyCell<T> {
	pub fn lazy<F: Fut<T>>(f: impl FnOnce() -> F + SendS) -> Self {
		let t = task::Runtime().spawn(f).pipe(Some).into();
		Self { v: Def(), t }
	}
}
impl<T: SendS> LazyCell<T> {
	pub fn new(v: T) -> Self {
		Self { v: Box(v).into(), t: Def() }
	}
	pub fn with<F: Fut<T>>(v: T, f: impl FnOnce(&'static T) -> F + SendS) -> Self {
		Self::new(v).tap(|s| s.lock().update(f))
	}
	pub fn lock(&mut self) -> LazyCellLock<T> {
		self.as_ref();
		LazyCellLock { l: self }
	}
	pub fn changed(&self) -> bool {
		(&self.t).bind().as_ref().is_some_and(|t| t.is_ready())
	}
}
impl<T: SendS> AsRef<T> for LazyCell<T> {
	fn as_ref(&self) -> &T {
		self
	}
}
impl<T: SendS> ops::Deref for LazyCell<T> {
	type Target = T;

	fn deref(&self) -> &T {
		let Self { v, t } = self;

		if self.changed() {
			let mut t = t.take().valid();
			let t = task::Runtime().finish_ref(&mut t);
			unsafe { *v.ptr() = t }
		}

		unsafe { v.as_ref() }
	}
}
impl<T: Debug + SendS> Debug for LazyCell<T> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		f.debug_tuple("LazyCell").field(&**self).finish()
	}
}
impl<T: SendS> Drop for LazyCell<T> {
	fn drop(&mut self) {
		self.lock().as_mut();
	}
}

#[derive(Default)]
pub struct Effect<T> {
	f: Cell<Option<EffectFn<T>>>,
}
impl<T> Effect<T> {
	pub fn apply(&self, v: &mut T) {
		if let Some(f) = self.f.take() {
			f(v)
		}
	}
}
impl<T, F: FnOnce(&mut T) + SendS> From<F> for Effect<T> {
	fn from(f: F) -> Self {
		let f: EffectFn<T> = Box(f);
		let f = Some(f).into();
		Self { f }
	}
}
impl<T: Debug> Debug for Effect<T> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		if (&self.f).bind().is_none() {
			write!(f, "noop")
		} else {
			write!(f, "Fn({})", type_name::<T>())
		}
	}
}
unsafe impl<T> Send for Effect<T> {}
unsafe impl<T> Sync for Effect<T> {}

type EffectFn<T> = Box<dyn FnOnce(&mut T) + Send>;
