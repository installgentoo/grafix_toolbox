use crate::lib::*;

macro_rules! APPLICATOR {
	($n: ident, $($t: ident),+) => {
		pub trait $n<$($t),+> {
			#![allow(dead_code)]
			fn apply(&self, _: unsafe fn($($t),+));
		}
		#[allow(unused_parens)]
		impl<$($t: GLPrimitive),+> $n<$($t),+> for ($($t),+) {
			fn apply(&self, func: unsafe fn($($t),+)) {
				let ($($t),+) = self;
				GL!(func($(*$t),+));
			}
		}
	}
}

APPLICATOR!(UnpackTuple0, t);
APPLICATOR!(UnpackTuple1, t0, t1);
APPLICATOR!(UnpackTuple2, t0, t1, t2);
APPLICATOR!(UnpackTuple3, t0, t1, t2, t3);
APPLICATOR!(UnpackTuple4, t0, t1, t2, t3, t4);
APPLICATOR!(UnpackTuple5, t0, t1, t2, t3, t4, t5);

pub fn states_map() -> &'static mut HashMap<usize, (bool, bool)> {
	LocalStatic!(HashMap<usize, (bool, bool)>)
}

macro_rules! FUNC {
($m: ident, $n: ident, $($t: ty),+) => {
	pub struct $n;
	#[allow(unused_parens)]
	impl $n {
		fn state() -> &'static mut ($($t),+) {
			LocalStatic!(($($t),+), { ($(0 as $t),+) })
		}
		fn saved_state() -> &'static mut ($($t),+) {
			LocalStatic!(($($t),+), { ($(0 as $t),+) })
		}

		pub fn Set(state: ($($t),+)) {
			let last_s = Self::state();
			debug_assert!({
				states_map().entry($m::$n as *const () as usize).or_insert_with(|| { ASSERT!(state != *last_s, "First call to GL::{}::Set() must not have all zeros as arguments", stringify!($n)); (false, false) });
				true
			});
			if *last_s != state {
				*last_s = state;
				state.apply($m::$n);
				DEBUG!("Set GL::{}({state:?})", stringify!($n));
			}
		}

		pub fn Save() {
			debug_assert!({
				let (valid, _) = states_map().get_mut(&($m::$n as *const () as usize)).explain_err(|| format!("GL::{}::Save() with default state, nothing to save", stringify!($n))).fail();
				*valid = true;
				true
			});
			*Self::saved_state() = *Self::state();
		}

		pub fn Restore() {
			ASSERT!({
					let (valid, _) = states_map().entry($m::$n as *const () as usize).or_insert((false, false));
					let r = *valid;
					*valid = false;
					r
				},
				"GL::{}::Restore() call not paired with Set()",
				stringify!($n)
			);
			let state = Self::state();
			let prev_s = Self::saved_state();
			if state != prev_s {
				*state = *prev_s;
				state.apply($m::$n);
				DEBUG!("Restored GL::{}({state:?})", stringify!($n));
			}
		}
	}
}}

pub trait State {
	fn gl_enable(t: GLenum) {
		GL!(gl::Enable(t))
	}
	fn gl_disable(t: GLenum) {
		GL!(gl::Disable(t))
	}
}

pub fn overflow_map() -> &'static mut HashMap<GLenum, i32> {
	LocalStatic!(HashMap<GLenum, i32>)
}

macro_rules! SWITCH {
	($n: ident) => {
		impl State for $n {}
		SWITCH_IMPL!($n, 0);
	};
	($n: ident, DEFAULT_TRUE) => {
		impl State for $n {}
		SWITCH_IMPL!($n, 18446744073709551615);
	};
	($n: ident, $e: expr, $d: expr, $i: literal) => {
		impl State for $n {
			fn gl_enable(_: GLenum) {
				GL!($e)
			}
			fn gl_disable(_: GLenum) {
				GL!($d)
			}
		}
		SWITCH_IMPL!($n, $i);
	};
	($n: ident, $e: expr, $d: expr) => {
		SWITCH!($n, $e, $d, 0);
	};
	($n: ident, $e: expr, $d: expr, DEFAULT_TRUE) => {
		SWITCH!($n, $e, $d, 18446744073709551615);
	};
}

macro_rules! SWITCH_IMPL {
	($n: ident, $i: literal) => {
		pub struct $n;
		impl $n {
			fn state() -> &'static mut u64 {
				LocalStatic!(u64, { $i })
			}

			pub fn Enable() {
				let state = Self::state();
				if (*state & 1u64) != 1u64 {
					Self::gl_enable(gl::$n);
					DEBUG!("Enabled GL::{}", stringify!($n));
					*state |= 1u64;
				}
			}

			pub fn Disable() {
				let state = Self::state();
				if (*state & 1u64) != 0u64 {
					Self::gl_disable(gl::$n);
					DEBUG!("Disabled GL::{}", stringify!($n));
					*state &= !1u64;
				}
			}

			pub fn Save() {
				debug_assert!({
					let v = overflow_map().entry(gl::$n).or_insert(0);
					*v = 64.min(*v + 1);
					true
				});

				let state = Self::state();
				*state = (*state & 1u64) | (*state << 1);
			}

			pub fn Restore() {
				ASSERT!(
					{
						let v = overflow_map().entry(gl::$n).or_insert(0);
						*v -= 1;
						*v >= 0
					},
					"No state for GL::{} in stack",
					stringify!($n)
				);

				let state = Self::state();
				let s = *state & 1u64;
				*state >>= 1;
				if s != (*state & 1u64) {
					DEBUG!("Restored GL::{}({s})", stringify!($n));
					if *state == 0u64 {
						Self::gl_disable(gl::$n);
					} else {
						Self::gl_enable(gl::$n);
					}
				}
			}
		}
	};
}

pub trait GLPrimitive: Copy {}
impl_trait_for!(GLPrimitive = u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, usize, isize);
