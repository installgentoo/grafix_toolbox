use crate::lib::*;

#[cfg(not(feature = "profiling"))]
#[macro_export]
macro_rules! TIMER {
	(GL, $_: ident, $b: block) => {{ $b }};
	($_: ident, $b: block) => {{ $b }};
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
		pub fn Start(name: STR) {
			let mut lock = map();
			let t = lock.remove(name).unwrap_or_else(Timer::new::<$t>);
			lock.insert(name, t.start(name));
		}
		pub fn Stop(name: STR) {
			let mut lock = map();
			let t = lock.remove(name).unwrap_or_else(Timer::new::<$t>);
			lock.insert(name, t.stop(name));
		}
		pub fn Print(name: &str) {
			let t = map().remove(name).explain_err(|| format!("No timer {name:?}")).fail();
			print_impl(name, t);
		}
		pub fn PrintAll() {
			let mut all = map().drain().collect_vec();
			all.sort_unstable_by_key(|&(k, _)| k);
			all.into_iter().for_each(|(n, t)| print_impl(n, t));
		}

		fn print_impl(name: &str, t: Timer) {
			let (t, i) = t.get_res(name);
			let format = move || {
				for (v, f) in [(1_000_000_000, " s"), (1_000_000, " ms"), (1_000, " us")] {
					let frame = i * v;
					if t >= frame {
						return format!("{:.3} {}", f64(t) / f64(frame), f);
					}
				}
				return format!("{:.3} ns", f64(t) / f64(i));
			};
			PRINT!("Timer {name:?}: {} |{i}", format());
		}
		fn map<'s>() -> MutexGuard<'s, HashMap<STR, Timer>> {
			LazyStatic!(HashMap<STR, Timer>, {
				logger::Logger::shutdown_hook(PrintAll);
				Def()
			})
		}

		use super::*;
		use GenericTimer as Timer;
	}
}}
PROFILER!(CPU, CPUTimerStopped);
PROFILER!(GPU, GLTimer);

enum GenericTimer {
	Started(Started),
	Done(Done),
}
impl GenericTimer {
	fn new<T: New>() -> Self {
		T::boxed_new().pipe(Self::Done)
	}
	fn start(self, _name: &str) -> Self {
		use GenericTimer::*;
		match self {
			Done(done) => done.start().pipe(Started),
			Started { .. } => ERROR!("Timer {_name:?} already started"),
		}
	}
	fn stop(self, _name: &str) -> Self {
		use GenericTimer::*;
		match self {
			Started(started) => started.stop().pipe(Done),
			Done { .. } => ERROR!("Timer {_name:?} not started"),
		}
	}
	fn get_res(self, _name: &str) -> (u128, u128) {
		use GenericTimer::*;
		match self {
			Done(done) => done.get_res(),
			Started(_) => ERROR!("Timer {_name:?} not stopped"),
		}
	}
}

type Started = Box<dyn Stop>;
type Done = Box<dyn Start>;
trait Start: SendS {
	fn start(self: Box<Self>) -> Started;
	fn get_res(self: Box<Self>) -> (u128, u128);
}
trait Stop: SendS {
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
		crate::GL!(gl::BeginQuery(gl::TIME_ELAPSED, self.o.obj));
		self
	}
	fn get_res(self: Box<Self>) -> (u128, u128) {
		let _ = mem::ManuallyDrop::new(self.o);
		(u128(self.total), self.iters)
	}
}
impl Stop for GLTimer {
	fn stop(self: Box<Self>) -> Done {
		crate::GL!(gl::EndQuery(gl::TIME_ELAPSED));
		let mut res = 0;
		crate::GL!(gl::GetQueryObjecti64v(self.o.obj, gl::QUERY_RESULT, &mut res));
		let Self { o, total, iters } = *self;
		Self { o, total: total + res, iters: iters + 1 }.pipe(Box)
	}
}
impl New for GLTimer {
	fn boxed_new() -> Done {
		Self::default().pipe(Box)
	}
}
unsafe impl Send for GLTimer {}

#[derive(Default)]
struct CPUTimerStopped {
	total: time::Duration,
	iters: u128,
}
impl Start for CPUTimerStopped {
	fn start(self: Box<Self>) -> Started {
		let Self { total, iters } = *self;
		CPUTimerStarted { total, iters, stamp: time::Instant::now() }.pipe(Box)
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
		CPUTimerStopped { total: total + stamp.elapsed(), iters: iters + 1 }.pipe(Box)
	}
}
impl New for CPUTimerStopped {
	fn boxed_new() -> Done {
		Self::default().pipe(Box)
	}
}
