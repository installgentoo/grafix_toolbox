use std::task::{Context, Poll, Poll::*};
use std::{cell::UnsafeCell, collections::VecDeque, future::Future, pin::Pin};

pub type Coro<T> = (&'static Executor, Fiber, *mut T);

#[macro_export]
macro_rules! YIELD {
	($c: ident) => {
		$c.1.waiter().await
	};
	($c: ident, $v: expr) => {
		unsafe { *$c.2 = $v };
		$c.1.waiter().await
	};
}
#[macro_export]
macro_rules! CORO {
	($c: ident, $b: expr) => {{
		$c.0.spawn($b)
	}};
}
#[macro_export]
macro_rules! CORO_FR {
	($c: ident, $b: expr) => {{
		$c.0.spawn_front($b)
	}};
}
#[macro_export]
macro_rules! CORO_REV {
	($c: ident, $b: expr) => {{
		let r = $c.0.spawn($b);
		let mut l = $c.0.len() - 1;
		while l < $c.0.len() {
			l += 1;
			$c.1.waiter().await;
		}
		r.get()
	}};
}

#[derive(Default)]
pub struct Executor {
	fibers: UnsafeCell<VecDeque<Pin<Box<dyn Future<Output = ()>>>>>,
}
unsafe impl Send for Executor {}
unsafe impl Sync for Executor {}
impl Executor {
	pub fn spawn<T: Default, F: Future<Output = ()> + 'static>(&self, f: impl FnOnce(Coro<T>) -> F) -> FiberRes<T> {
		let fibers = unsafe { &mut *self.fibers.get() };
		let ptr = Box::into_raw(Box::<T>::default());
		let res = FiberRes { ptr };
		fibers.push_back(Box::pin(f((forget(self), Fiber { state: UnsafeCell::new(Running) }, ptr))));
		res
	}
	pub fn spawn_front<T: Default, F: Future<Output = ()> + 'static>(&self, f: impl FnOnce(Coro<T>) -> F) -> FiberRes<T> {
		let fibers = unsafe { &mut *self.fibers.get() };
		let ptr = Box::into_raw(Box::<T>::default());
		let res = FiberRes { ptr };
		fibers.push_front(Box::pin(f((forget(self), Fiber { state: UnsafeCell::new(Running) }, ptr))));
		res
	}
	pub fn run(&self) {
		let fibers = unsafe { &mut *self.fibers.get() };
		let waker = nullwaker();
		let mut context = Context::from_waker(&waker);

		while let Some(mut fib) = fibers.pop_front() {
			match fib.as_mut().poll(&mut context) {
				Pending => {
					fibers.push_back(fib);
				}
				Ready(()) => {}
			}
		}
	}
	pub fn len(&self) -> usize {
		let fibers = unsafe { &mut *self.fibers.get() };
		fibers.len()
	}
	pub fn reverse_all(&self) {
		let fibers = unsafe { &mut *self.fibers.get() };
		let mut t = VecDeque::default();
		std::mem::swap(fibers, &mut t);
		*fibers = t.into_iter().rev().collect();
	}
}

pub struct Fiber {
	state: UnsafeCell<State>,
}
impl Fiber {
	pub fn waiter(&self) -> Waiter<'_> {
		Waiter { fib: self }
	}
}
pub struct Waiter<'s> {
	fib: &'s Fiber,
}
impl<'s> Future for Waiter<'s> {
	type Output = ();

	fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
		let state = unsafe { &mut *self.fib.state.get() };
		match state {
			Halted => {
				*state = Running;
				Ready(())
			}
			Running => {
				*state = Halted;
				Pending
			}
		}
	}
}

fn nullwaker() -> std::task::Waker {
	use std::task::*;
	const NULL_WAKER: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
	const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
	fn clone(_: *const ()) -> RawWaker {
		NULL_WAKER
	}
	fn wake(_: *const ()) {}
	fn wake_by_ref(_: *const ()) {}
	fn drop(_: *const ()) {}

	unsafe { Waker::from_raw(NULL_WAKER) }
}

enum State {
	Halted,
	Running,
}
use State::*;

fn forget<T>(v: &T) -> &'static T {
	unsafe { &*(v as *const _) }
}

pub struct FiberRes<T> {
	ptr: *mut T,
}
impl<T> FiberRes<T> {
	pub fn get(mut self) -> T {
		let p = self.ptr;
		self.ptr = std::ptr::null_mut();
		*unsafe { Box::from_raw(p) }
	}
}
impl<T> Drop for FiberRes<T> {
	fn drop(&mut self) {
		if !self.ptr.is_null() {
			let _ = unsafe { Box::from_raw(self.ptr) };
		}
	}
}
