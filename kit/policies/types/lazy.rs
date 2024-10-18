use crate::{lib::*, sync::*};
use std::pin;

#[derive(Clone)]
pub struct RcLazy<T>(Rc<Cell<Lazy<T>>>);
impl<T: SendStat + Default> RcLazy<T> {
	pub fn changed(&mut self) -> bool {
		unsafe { &mut *self.0.as_ptr() }.changed()
	}
	pub fn get(&mut self) -> &mut T {
		unsafe { &mut *self.0.as_ptr() }.get()
	}
}
impl<T> ops::Deref for RcLazy<T> {
	type Target = Lazy<T>;
	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0.as_ptr() }
	}
}
impl<T> ops::DerefMut for RcLazy<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		unsafe { &mut *self.0.as_ptr() }
	}
}
impl<T> From<Lazy<T>> for RcLazy<T> {
	fn from(s: Lazy<T>) -> Self {
		let s = Rc::new(Cell(s));
		Self(s)
	}
}

pub struct Lazy<T> {
	state: State<T>,
	loaded: Arc<AtomicBool>,
}
impl<T: SendStat + Default> Lazy<T> {
	pub fn new(stream: impl Stream<Item = T> + SendStat) -> Self {
		let loaded = Arc::new(AtomicBool::new(false));
		Self {
			state: Init(task::Runtime().spawn({
				let (l, mut s) = (loaded.clone(), Box::pin(stream) as pin::Pin<Box<dyn Stream<Item = T> + Send>>);
				async move {
					let r = s.next().await;
					l.store(true, Ordering::Relaxed);
					(r, s)
				}
			})),
			loaded,
		}
	}
	pub fn changed(&mut self) -> bool {
		if !self.loaded.load(Ordering::Relaxed) {
			return false;
		}

		check_and_load(false, self)
	}
	pub fn get(&mut self) -> &mut T {
		self.changed();

		if let Init(_) = self.state {
			check_and_load(true, self);
		}

		match &mut self.state {
			Loading(v_last, _) => v_last,
			Quit(v) => v,
			_ => unreachable!(),
		}
	}
	pub fn take(mut self) -> T {
		self.get();

		match self.state {
			Loading(v_last, _) => v_last,
			Quit(v) => v,
			_ => unreachable!(),
		}
	}
}

fn check_and_load<T: SendStat + Default>(blocking: bool, lazy: &mut Lazy<T>) -> bool {
	let Lazy { state, ref loaded } = lazy;

	let check_progress = |blocking: bool, t: &mut Task<(Option<T>, Source<T>)>| {
		let reload = |(v, mut s): (Option<_>, Source<T>)| {
			let l = loaded.clone();
			v.map(|v| {
				l.store(false, Ordering::Relaxed);
				Loading(
					v,
					task::Runtime().spawn({
						let l = loaded.clone();
						async move {
							let r = s.next().await;
							while l.load(Ordering::Relaxed) {
								task::sleep(time::Duration::from_millis(10)).await;
							}
							l.store(true, Ordering::Relaxed);
							(r, s)
						}
					}),
				)
			})
			.map(|v| (v, true))
		};

		if blocking || t.is_ready() {
			Some(reload(task::Runtime().finish_ref(t)))
		} else {
			None
		}
	};

	let (s, changed) = match mem::replace(state, Init(Task::new_uninit())) {
		s @ Quit(_) => {
			loaded.store(false, Ordering::Relaxed);
			(s, false)
		}
		Init(mut t) => {
			if let Some(v) = check_progress(blocking, &mut t) {
				v.unwrap_or_else(|| {
					FAIL!("Source for Lazy<{}> failed to start", type_name::<T>());
					(Quit(Def()), false)
				})
			} else {
				(Init(t), false)
			}
		}
		Loading(v_last, mut t) => {
			if let Some(v) = check_progress(false, &mut t) {
				v.unwrap_or_else(|| (Quit(v_last), false))
			} else {
				(Loading(v_last, t), false)
			}
		}
	};
	*state = s;

	changed
}

enum State<T> {
	Init(Task<(Option<T>, Source<T>)>),
	Loading(T, Task<(Option<T>, Source<T>)>),
	Quit(T),
}
use State::*;
type Source<T> = pin::Pin<Box<dyn Stream<Item = T> + Send>>;
