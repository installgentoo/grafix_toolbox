use crate::uses::*;

macro_rules! CONST {
	($n: ident, $t: ty) => {
		pub fn $n() -> $t {
			static mut DONE: bool = false;
			static mut RES: $t = <$t>::ZERO;
			unsafe {
				if DONE == true {
				} else {
					DONE = true;
					RES = <$t>::get(gl::$n)
				}
			}
			unsafe { RES }
		}
	};
}

pub trait Get {
	const ZERO: Self;
	fn get(_: GLenum) -> Self;
}
impl Get for GLbool {
	const ZERO: Self = 0;
	fn get(p: GLenum) -> Self {
		let mut r = Self::ZERO;
		GLCheck!(gl::GetBooleanv(p, &mut r));
		r
	}
}
impl Get for i32 {
	const ZERO: Self = 0;
	fn get(p: GLenum) -> Self {
		let mut r = Self::ZERO;
		GLCheck!(gl::GetIntegerv(p, &mut r));
		r
	}
}
impl Get for GLenum {
	const ZERO: Self = 0;
	fn get(_: GLenum) -> Self {
		unreachable!("No such GL function");
	}
}
impl Get for f32 {
	const ZERO: Self = 0.;
	fn get(p: GLenum) -> Self {
		let mut r = Self::ZERO;
		GLCheck!(gl::GetFloatv(p, &mut r));
		r
	}
}
impl Get for f64 {
	const ZERO: Self = 0.;
	fn get(p: GLenum) -> Self {
		let mut r = Self::ZERO;
		GLCheck!(gl::GetDoublev(p, &mut r));
		r
	}
}
