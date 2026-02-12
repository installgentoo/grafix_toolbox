use crate::lib::*;

#[derive(Debug)]
pub struct FeedLock<'l, T> {
	s: &'l mut State<T>,
}
impl<T: SendS> FeedLock<'_, Option<T>> {
	pub fn try_take(self) -> Option<T> {
		let (Load(v, _) | Quit(v)) = self.s else { unreachable!() };
		mem::take(v)?.into()
	}
}
impl<T: SendS> AsRef<T> for FeedLock<'_, T> {
	fn as_ref(&self) -> &T {
		self
	}
}
impl<T: SendS> ops::Deref for FeedLock<'_, T> {
	type Target = T;

	fn deref(&self) -> &T {
		let (Load(v, _) | Quit(v)) = &*self.s else { unreachable!() };
		v
	}
}

pub struct Feed<T> {
	s: Cell<State<T>>,
}
impl<T: SendS> Feed<T> {
	pub fn with(val: T, stream: impl Stream<Item = T> + SendS) -> Self {
		let Self { s } = Self::new(stream);
		let Init(t) = s.into_inner() else { unreachable!() };
		Self { s: Load(val, t).into() }
	}
	pub fn lazy<F: Fut<T>>(f: impl FnOnce() -> F + SendS) -> Self {
		Self::new(stream::once(async move { f().await }))
	}
	pub fn new(stream: impl Stream<Item = T> + SendS) -> Self {
		let task = task::Runtime().spawn({
			let mut s = Box::pin(stream) as Source<T>;
			async || {
				let r = s.next().await;
				(r, s)
			}
		});
		Self { s: Init(task).into() }
	}
	pub fn lock(&mut self) -> FeedLock<T> {
		self.load();
		FeedLock { s: self.s.get_mut() }
	}
	pub fn try_lock(&mut self) -> Option<FeedLock<T>> {
		check_and_load(false, self);
		if matches!((&self.s).bind(), Init(_)) {
			None?
		}
		FeedLock { s: self.s.get_mut() }.into()
	}
	fn load(&mut self) {
		let force = matches!((&self.s).bind(), Init(_));
		check_and_load(force, self);
	}
	pub fn take(mut self) -> T {
		self.load();
		let s = self.s.take();
		let (Load(v, _) | Quit(v)) = s else { unreachable!() };
		v
	}
}

fn check_and_load<T: SendS>(blocking: bool, lazy: &mut Feed<T>) {
	let Feed { s } = lazy;

	let next = |blocking: bool, t: &mut Task<(Option<T>, Source<T>)>| {
		if !blocking && !t.is_ready() {
			None?
		}

		let (v, mut s) = task::Runtime().finish_ref(t);
		v.map(|v| {
			Load(
				v,
				task::Runtime().spawn({
					async || {
						let r = s.next().await;
						(r, s)
					}
				}),
			)
		})
		.pipe(Some)
	};

	let new_state = match s.take() {
		s @ Quit(_) => s,
		Init(mut t) => match next(blocking, &mut t) {
			None => Init(t),
			Some(Some(s)) => s,
			Some(None) => ERROR!("Source for Feed<{}> failed to start", type_name::<T>()),
		},
		Load(v, mut t) => match next(false, &mut t) {
			None => Load(v, t),
			Some(Some(s)) => s,
			Some(None) => Quit(v),
		},
	};

	s.set(new_state);
}

impl<T: Default> Default for Feed<T> {
	fn default() -> Self {
		Self { s: Quit(Def()).into() }
	}
}
impl<T: Debug> Debug for Feed<T> {
	fn fmt(&self, f: &mut Formatter) -> fmtRes {
		f.debug_tuple("Feed").field((&self.s).bind()).finish()
	}
}

#[derive(Debug)]
enum State<T> {
	Init(Task<(Option<T>, Source<T>)>),
	Load(T, Task<(Option<T>, Source<T>)>),
	Quit(T),
}
impl<T> Default for State<T> {
	fn default() -> Self {
		Task::new_uninit().pipe(Init)
	}
}
use State::*;

type Source<T> = stream::Boxed<T>;
