use crate::lib::*;

macro_rules! CONST {
	($n: ident, $t: ty) => {
		pub fn $n() -> $t {
			static mut DONE: bool = false;
			static mut RES: $t = 0 as $t;
			unsafe {
				if DONE == true {
				} else {
					(RES, DONE) = (<$t>::get(gl::$n), true);
				}
				RES
			}
		}
	};
}

pub trait Get {
	fn get(_: GLenum) -> Self;
}
macro_rules! impl_get {
	($t: ty, $f: ident) => {
		impl Get for $t {
			fn get(p: GLenum) -> Self {
				Def::<$t>().tap(|s| GL!(gl::$f(p, s)))
			}
		}
	};
}
impl_get!(GLbool, GetBooleanv);
impl_get!(i32, GetIntegerv);
impl_get!(f32, GetFloatv);
impl_get!(f64, GetDoublev);
