use crate::uses::*;

macro_rules! APPLICATOR {
	($n: ident, $($t: ident),+) => {
		pub trait $n<$($t),+> {
			fn apply(&self, _: unsafe fn($($t),+)) {}
		}
		#[allow(unused_parens)]
		impl<$($t: Copy + GLPrimitive),+> $n<$($t),+> for ($($t),+) {
			fn apply(&self, func: unsafe fn($($t),+)) {
				let ($($t),+) = self;
				GLCheck!(func($(*$t),+));
			}
		}
	};
}

APPLICATOR!(UnpackTuple0, t);
APPLICATOR!(UnpackTuple1, t0, t1);
APPLICATOR!(UnpackTuple2, t0, t1, t2);
APPLICATOR!(UnpackTuple3, t0, t1, t2, t3);
APPLICATOR!(UnpackTuple4, t0, t1, t2, t3, t4);
APPLICATOR!(UnpackTuple5, t0, t1, t2, t3, t4, t5);

pub fn states_map() -> &'static mut HashMap<usize, (bool, bool)> {
	UnsafeOnce!(HashMap<usize, (bool, bool)>, { HashMap::new() })
}

macro_rules! FUNC {
	($m: ident, $p: ident, $($t: ident),+) => {
pub struct $p;
#[allow(unused_parens)]
impl $p {
	fn state() -> &'static mut ($($t),+) {
		static mut STATE: ($($t),+) = ($($t::ZERO),+);
		unsafe { &mut STATE }
	}
	fn saved_state() -> &'static mut ($($t),+) {
		static mut STATE: ($($t),+) = ($($t::ZERO),+);
		unsafe { &mut STATE }
	}

	pub fn Set(state: ($($t),+)) {
		let last_s = Self::state();
		debug_assert!({
			states_map().entry($m::$p as *const () as usize).or_insert_with(|| { ASSERT!(state != *last_s, "First call to GL::{}::Set() must not have all zeros as arguments", stringify!($p)); (false, false) });
			true
		});
		if *last_s != state {
			*last_s = state;
			state.apply($m::$p);
			DEBUG!("Set {}::{}({:?})", stringify!($m), stringify!($p), state);
		}
	}

	pub fn Save() {
		debug_assert!({
			let (valid, _) = EXPECT!(states_map().get_mut(&($m::$p as *const () as usize)), "GL::{}::Set() must be called at least once", stringify!($p));
			*valid = true;
			true
		});
		*Self::saved_state() = *Self::state();
	}

	pub fn Restore() {
		ASSERT!(
			{
				let (valid, _) = states_map().entry($m::$p as *const () as usize).or_insert((false, false));
				let r = *valid;
				*valid = false;
				r
			},
			"GL::{}::Restore() call not paired with Set()",
			stringify!($p)
		);
		let state = Self::state();
		let prev_s = Self::saved_state();
		if *state != *prev_s {
			*state = *prev_s;
			state.apply($m::$p);
			DEBUG!("Restored {}::{}({:?})", stringify!($m), stringify!($p), state);
		}
	}
}
}
}

pub trait State {
	fn gl_enable(t: GLenum) {
		GLCheck!(gl::Enable(t))
	}
	fn gl_disable(t: GLenum) {
		GLCheck!(gl::Disable(t))
	}
}

pub fn overflow_map() -> &'static mut HashMap<GLenum, i32> {
	UnsafeOnce!(HashMap<GLenum, i32>, { HashMap::new() })
}

macro_rules! SWITCH {
	($p: ident) => {
		impl State for $p {}
		SWITCH_IMPL!($p, 0);
	};

	($p: ident, DEFAULT_TRUE) => {
		impl State for $p {}
		SWITCH_IMPL!($p, 18446744073709551615);
	};

	($p: ident, $e: expr, $d: expr, $i: literal) => {
		impl State for $p {
			fn gl_enable(_: GLenum) {
				GLCheck!($e)
			}
			fn gl_disable(_: GLenum) {
				GLCheck!($d)
			}
		}
		SWITCH_IMPL!($p, $i);
	};
	($p: ident, $e: expr, $d: expr) => {
		SWITCH!($p, $e, $d, 0);
	};
	($p: ident, $e: expr, $d: expr, DEFAULT_TRUE) => {
		SWITCH!($p, $e, $d, 18446744073709551615);
	};
}

macro_rules! SWITCH_IMPL {
	($p: ident, $i: literal) => {
		pub struct $p;
		impl $p {
			fn state() -> &'static mut u64 {
				static mut STATE: u64 = $i;
				unsafe { &mut STATE }
			}

			pub fn Enable() {
				let state = Self::state();
				if (*state & 1u64) != 1u64 {
					Self::gl_enable(gl::$p);
					DEBUG!("Enabled {}::{}", stringify!($m), stringify!($p));
					*state |= 1u64;
				}
			}

			pub fn Disable() {
				let state = Self::state();
				if (*state & 1u64) != 0u64 {
					Self::gl_disable(gl::$p);
					DEBUG!("Disabled {}::{}", stringify!($m), stringify!($p));
					*state &= !1u64;
				}
			}

			pub fn Save() {
				debug_assert!({
					let v = overflow_map().entry(gl::$p).or_insert(0);
					*v = 64.min(*v + 1);
					true
				});

				let state = Self::state();
				*state = (*state & 1u64) | (*state << 1);
			}

			pub fn Restore() {
				ASSERT!(
					{
						let v = overflow_map().entry(gl::$p).or_insert(0);
						*v -= 1;
						*v >= 0
					},
					"No state for GL::{} in stack",
					stringify!($p)
				);

				let state = Self::state();
				let s = *state & 1u64;
				*state >>= 1;
				if s != (*state & 1u64) {
					DEBUG!("Restored {}::{}({})", stringify!($m), stringify!($p), s);
					if *state == 0u64 {
						Self::gl_disable(gl::$p);
					} else {
						Self::gl_enable(gl::$p);
					}
				}
			}
		}
	};
}

pub trait GLPrimitive {}
impl GLPrimitive for u8 {}
impl GLPrimitive for i8 {}
impl GLPrimitive for u16 {}
impl GLPrimitive for i16 {}
impl GLPrimitive for u32 {}
impl GLPrimitive for i32 {}
impl GLPrimitive for u64 {}
impl GLPrimitive for i64 {}
impl GLPrimitive for f32 {}
impl GLPrimitive for f64 {}
impl GLPrimitive for usize {}
impl GLPrimitive for isize {}
