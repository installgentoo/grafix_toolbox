use crate::uses::Sync::sync::{Mutex, Once};
use crate::uses::{time::*, *};
use crate::GL;

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

macro_rules! Profiler {
	($n: ident, $t: ty) => {
		pub mod $n {
			use super::*;
			use GenericTimer as Timer;

			pub fn Start(name: Str) {
				static INIT: Once = Once::new();
				INIT.call_once(move || {
					logging::Logger::AddPostmortem(PrintAll);
				});
				let mut lock = EXPECT!(map().lock());
				let t = lock.remove(name).unwrap_or_else(Timer::new::<$t>);
				lock.insert(name, t.start(name));
			}
			pub fn Stop(name: Str) {
				let mut lock = EXPECT!(map().lock());
				let t = lock.remove(name).unwrap_or_else(Timer::new::<$t>);
				lock.insert(name, t.stop(name));
			}
			pub fn Print(name: Str) {
				let t = EXPECT!(EXPECT!(map().lock()).remove(name), "No timer '{}'", name);
				print_impl(name, t);
			}
			pub fn PrintAll() {
				let mut all: Vec<_> = EXPECT!(map().lock()).drain().collect();
				all.sort_by(|(a, _), (b, _)| a.cmp(b));
				all.into_iter().for_each(|(n, t)| print_impl(n, t));
			}

			fn print_impl(name: Str, t: Timer) {
				let (t, i) = t.get_res(name);
				let format = move || {
					for step in &[(1_000_000_000, " s"), (1_000_000, " ms"), (1_000, " us")] {
						let frame = i * step.0;
						if t >= frame {
							return CONCAT!(&(f64::to(t) / f64::to(frame)).to_string(), step.1);
						}
					}
					return CONCAT!(&(f64::to(t) / f64::to(i)).to_string(), " ns");
				};
				PRINT!("Timer '{}': {} |{}", name, format(), i);
			}
			fn map() -> &'static mut Mutex<HashMap<Str, Timer>> {
				static INIT: Once = Once::new();
				static mut MAP: Option<Mutex<HashMap<&str, Timer>>> = None;
				unsafe {
					INIT.call_once(|| {
						MAP = Some(Mutex::new(HashMap::new()));
					});
					MAP.as_mut().unwrap()
				}
			}
		}
	};
}

Profiler!(CPU, CPUTimerStopped);
Profiler!(GPU, GLTimer);

enum GenericTimer {
	Started(Started),
	Done(Done),
}
impl GenericTimer {
	fn new<T: New>() -> Self {
		GenericTimer::Done(T::boxed_new())
	}
	fn start(self, name: Str) -> Self {
		use GenericTimer::*;
		match self {
			Done(done) => GenericTimer::Started(done.start()),
			Started { .. } => ASSERT!(false, "Timer '{}' already started", name),
		}
	}
	fn stop(self, name: Str) -> Self {
		use GenericTimer::*;
		match self {
			Started(started) => GenericTimer::Done(started.stop()),
			Done { .. } => ASSERT!(false, "Timer '{}' not started", name),
		}
	}
	fn get_res(self, name: Str) -> (u128, u128) {
		use GenericTimer::*;
		match self {
			Done(done) => done.get_res(),
			Started(_) => ASSERT!(false, "Timer '{}' not stopped", name),
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
		GLCheck!(gl::BeginQuery(gl::TIME_ELAPSED, self.o.obj));
		Box::new(*self)
	}
	fn get_res(self: Box<Self>) -> (u128, u128) {
		let _ = mem::ManuallyDrop::new(self.o);
		(u128::to(self.total), self.iters)
	}
}
impl Stop for GLTimer {
	fn stop(self: Box<Self>) -> Done {
		GLCheck!(gl::EndQuery(gl::TIME_ELAPSED));
		let mut res = 0;
		GLCheck!(gl::GetQueryObjecti64v(self.o.obj, gl::QUERY_RESULT, &mut res));
		let Self { o, total, iters } = *self;
		Box::new(Self {
			o,
			total: total + res,
			iters: iters + 1,
		})
	}
}
impl New for GLTimer {
	fn boxed_new() -> Done {
		Box::new(Self::default())
	}
}

#[derive(Default)]
struct CPUTimerStopped {
	total: Duration,
	iters: u128,
}
impl Start for CPUTimerStopped {
	fn start(self: Box<Self>) -> Started {
		let Self { total, iters } = *self;
		Box::new(CPUTimerStarted {
			total,
			iters,
			stamp: Instant::now(),
		})
	}
	fn get_res(self: Box<Self>) -> (u128, u128) {
		(self.total.as_nanos(), self.iters)
	}
}
struct CPUTimerStarted {
	total: Duration,
	iters: u128,
	stamp: Instant,
}
impl Stop for CPUTimerStarted {
	fn stop(self: Box<Self>) -> Done {
		let Self { total, iters, stamp } = *self;
		Box::new(CPUTimerStopped {
			total: total + stamp.elapsed(),
			iters: iters + 1,
		})
	}
}
impl New for CPUTimerStopped {
	fn boxed_new() -> Done {
		Box::new(Self::default())
	}
}
