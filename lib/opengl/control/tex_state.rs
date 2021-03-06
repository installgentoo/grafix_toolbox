use super::{policy::*, universion::*};
use crate::uses::*;

struct Units {
	at: u32,
	len: u32,
	units: Vec<(u32, u32, u32)>,
}

pub mod TexState {
	use super::*;

	fn bound_units() -> &'static mut Units {
		UnsafeOnce!(Units, {
			let len = u32(GL::MAX_TEXTURE_IMAGE_UNITS().min(GL::MAX_COMBINED_TEXTURE_IMAGE_UNITS()));
			Units {
				at: 0,
				len,
				units: vec![(0, 0, 0); usize(len)],
			}
		})
	}
	fn garbage_collect<T: TexType>() -> u32 {
		let Units { at, len, units } = bound_units();

		let npos = 1 + *len;
		let mut empty = npos;
		for i in 0..*len {
			let (unit, _, counter) = units.at_mut(i);
			if *counter == 0 {
				DEBUG!("Unbing GL {} {} from unit {}", type_name!(Texture<T>), *unit, i);
				*unit = 0;
				if empty == npos {
					empty = i;
				}
			} else {
				*at = i;
			}
		}

		ASSERT!(empty != npos, "Ran out of GL texture units, {} available", len);
		if empty == npos {
			WARN!("Ran out of GL texture units({} available), returning rubbish", len);
			empty = 0;
		}

		empty
	}
	pub fn Unbind(u: u32) {
		let (_, _, counter) = bound_units().units.at_mut(u);
		*counter -= 1;
	}
	pub fn Bind<T: TexType>(obj: u32, s: u32, hint: u32) -> u32 {
		let Units { at, len, units } = bound_units();

		let (h_obj, samp, counter) = units.at_mut(hint);
		if *h_obj == obj && *samp == s {
			*counter += 1;
			return hint;
		}

		let npos = 1 + *len;
		let mut empty = npos;
		for i in 0..*at {
			let (unit, samp, counter) = units.at_mut(i);
			if *unit == obj && *samp == s {
				*counter += 1;
				return i;
			}
			if empty == npos && *unit == 0 {
				empty = i;
			}
		}

		if empty == npos {
			empty = *at;
			*at += 1;
		}

		if empty >= *len {
			empty = garbage_collect::<T>();
		}

		let (unit, samp, counter) = units.at_mut(empty);
		*counter += 1;
		*unit = obj;
		let u = empty;
		DEBUG!("Binding GL {} {} to unit {}", type_name!(Texture<T>), obj, u);
		GLCheck!(glBindTextureUnit(T::TYPE, u, obj));
		if *samp != s {
			*samp = s;
			DEBUG!("Binding GL {} {} to unit {}", type_name!(SamplObj), s, u);
			GLCheck!(gl::BindSampler(u, s));
		}
		DEBUG!("GL texture units: {:?}", units);
		u
	}
	pub fn BindAny<T: TexType>(obj: u32, hint: u32) -> u32 {
		let Units { at, len, units } = bound_units();

		let (h_obj, _, counter) = units.at_mut(hint);
		if *h_obj == obj {
			*counter += 1;
			return hint;
		}

		let npos = 1 + *len;
		let mut empty = npos;
		for i in 0..*at {
			let (unit, _, counter) = units.at_mut(i);
			if *unit == obj {
				*counter += 1;
				return i;
			}
			if empty == npos && *unit == 0 {
				empty = i;
			}
		}

		if empty == npos {
			empty = *at;
			*at += 1;
		}

		if empty >= *len {
			empty = garbage_collect::<T>();
		}

		let (unit, _, counter) = units.at_mut(empty);
		*counter += 1;
		*unit = obj;
		let u = empty;
		DEBUG!("Binding GL {} {} to unit {}", type_name!(Texture<T>), obj, u);
		GLCheck!(glBindTextureUnit(T::TYPE, u, obj));
		DEBUG!("GL texture units: {:?}", units);
		u
	}
	pub fn drop_tex(obj: u32) {
		let Units { at, len, units } = bound_units();
		for i in 0..*len {
			let (unit, _, _counter) = units.at_mut(i);
			if obj == *unit {
				ASSERT!(*_counter == 0, "Leakage in GL texture {} binding", obj);
				*unit = 0;
				if *at == i + 1 {
					*at = i;
				}
			}
		}
	}
	pub fn drop_samp(s: u32) {
		let Units { len, units, .. } = bound_units();
		for i in 0..*len {
			let (_, samp, _) = units.at_mut(i);
			if s == *samp {
				*samp = 0;
			}
		}
	}
}
