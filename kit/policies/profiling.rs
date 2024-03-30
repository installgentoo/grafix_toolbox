use super::{ext::*, logging, math::*};
use crate::{stdlib::*, GL};
use std::sync::{Mutex, OnceLock};

#[cfg(not(feature = "profiling"))]
#[macro_export]
macro_rules! TIMER {
	(GL, $_: ident, $b: block) => {{
		$b
	}};
	($_: ident, $b: block) => {{
		$b
	}};
}

#[cfg(feature = "profiling")]
#[macro_export]
macro_rules! TIMER {
	(GL, $n: ident, $b: block) => {{
		profiling::GPU::Start(stringify!($n));
		let ret = $b;
		profiling::GPU::Stop(stringify!($n));
		ret
	}};
	($n: ident, $b: block) => {{
		profiling::CPU::Start(stringify!($n));
		let ret = $b;
		profiling::CPU::Stop(stringify!($n));
		ret
	}};
}

macro_rules! PROFILER {
	($n: ident, $t: ty) => {
		pub mod $n {
			use super::*;
			use GenericTimer as Timer;

			pub fn Start(name: STR) {
				let mut lock = map().lock().valid();
				let t = lock.remove(name).unwrap_or_else(Timer::new::<$t>);
				lock.insert(name, t.start(name));
			}
			pub fn Stop(name: STR) {
				let mut lock = map().lock().valid();
				let t = lock.remove(name).unwrap_or_else(Timer::new::<$t>);
				lock.insert(name, t.stop(name));
			}
			pub fn Print(name: &str) {
				let t = EXPECT!(map().lock().valid().remove(name), "No timer {name:?}");
				print_impl(name, t);
			}
			pub fn PrintAll() {
				let mut all = map().lock().valid().drain().collect_vec();
				all.sort_by(|(a, _), (b, _)| a.cmp(b));
				all.into_iter().for_each(|(n, t)| print_impl(n, t));
			}

			fn print_impl(name: &str, t: Timer) {
				let (t, i) = t.get_res(name);
				let format = move || {
					for step in &[(1_000_000_000, " s"), (1_000_000, " ms"), (1_000, " us")] {
						let frame = i * step.0;
						if t >= frame {
							return format!("{:.3} {}", f64(t) / f64(frame), step.1);
						}
					}
					return format!("{:.3} ns", f64(t) / f64(i));
				};
				PRINT!("Timer {name:?}: {} |{i}", format());
			}
			fn map() -> &'static mut Mutex<HashMap<STR, Timer>> {
				static mut MAP: OnceLock<Mutex<HashMap<&str, Timer>>> = OnceLock::new();
				unsafe {
					MAP.get_or_init(|| {
						logging::Logger::shutdown_hook(PrintAll);
						Def()
					});
					MAP.get_mut().valid()
				}
			}
		}
	};
}
PROFILER!(CPU, CPUTimerStopped);
PROFILER!(GPU, GLTimer);

enum GenericTimer {
	Started(Started),
	Done(Done),
}
impl GenericTimer {
	fn new<T: New>() -> Self {
		Self::Done(T::boxed_new())
	}
	fn start(self, _name: &str) -> Self {
		use GenericTimer::*;
		match self {
			Done(done) => Started(done.start()),
			Started { .. } => ASSERT!(false, "Timer {_name:?} already started"),
		}
	}
	fn stop(self, _name: &str) -> Self {
		use GenericTimer::*;
		match self {
			Started(started) => Done(started.stop()),
			Done { .. } => ASSERT!(false, "Timer {_name:?} not started"),
		}
	}
	fn get_res(self, _name: &str) -> (u128, u128) {
		use GenericTimer::*;
		match self {
			Done(done) => done.get_res(),
			Started(_) => ASSERT!(false, "Timer {_name:?} not stopped"),
		}
	}
}

type Started = Box<dyn Stop>;
type Done = Box<dyn Start>;
trait Start {
	fn start(self: Box<Self>) -> Started;
	fn get_res(self: Box<Self>) -> (u128, u128);
}
trait Stop {
	fn stop(self: Box<Self>) -> Done;
}
trait New {
	fn boxed_new() -> Done;
}

#[derive(Default)]
struct GLTimer {
	o: GL::Query,
	total: i64,
	iters: u128,
}
impl Start for GLTimer {
	fn start(self: Box<Self>) -> Started {
		crate::GLCheck!(gl::BeginQuery(gl::TIME_ELAPSED, self.o.obj));
		Box(*self)
	}
	fn get_res(self: Box<Self>) -> (u128, u128) {
		let _ = mem::ManuallyDrop::new(self.o);
		(u128(self.total), self.iters)
	}
}
impl Stop for GLTimer {
	fn stop(self: Box<Self>) -> Done {
		crate::GLCheck!(gl::EndQuery(gl::TIME_ELAPSED));
		let mut res = 0;
		crate::GLCheck!(gl::GetQueryObjecti64v(self.o.obj, gl::QUERY_RESULT, &mut res));
		let Self { o, total, iters } = *self;
		Box(Self { o, total: total + res, iters: iters + 1 })
	}
}
impl New for GLTimer {
	fn boxed_new() -> Done {
		Box(Self::default())
	}
}

#[derive(Default)]
struct CPUTimerStopped {
	total: time::Duration,
	iters: u128,
}
impl Start for CPUTimerStopped {
	fn start(self: Box<Self>) -> Started {
		let Self { total, iters } = *self;
		Box(CPUTimerStarted { total, iters, stamp: time::Instant::now() })
	}
	fn get_res(self: Box<Self>) -> (u128, u128) {
		(self.total.as_nanos(), self.iters)
	}
}
struct CPUTimerStarted {
	total: time::Duration,
	iters: u128,
	stamp: time::Instant,
}
impl Stop for CPUTimerStarted {
	fn stop(self: Box<Self>) -> Done {
		let Self { total, iters, stamp } = *self;
		Box(CPUTimerStopped { total: total + stamp.elapsed(), iters: iters + 1 })
	}
}
impl New for CPUTimerStopped {
	fn boxed_new() -> Done {
		Box(Self::default())
	}
}
