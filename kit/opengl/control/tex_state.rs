use super::{policy::*, universion::*, *};

struct Units {
	used: u32,
	len: u32,
	units: Vec<Unit>,
}
#[derive(Default, Debug, Clone)]
struct Unit {
	tex: u32,
	samp: u32,
	bind_count: u32,
}

pub mod TexState {
	use super::*;

	fn bound_units() -> &'static mut Units {
		LocalStatic!(Units, {
			let len = u32(GL::MAX_TEXTURE_IMAGE_UNITS().min(GL::MAX_COMBINED_TEXTURE_IMAGE_UNITS()));
			Units { used: 0, len, units: vec![Def(); usize(len)] }
		})
	}
	fn garbage_collect<T: TexType>() -> u32 {
		let Units { ref mut used, len, ref mut units } = *bound_units();

		let npos = 1 + len;
		let mut empty = npos;
		for i in 0..len {
			let Unit { ref mut tex, bind_count, .. } = *units.at_mut(i);
			if bind_count == 0 {
				DEBUG!("Unbing GL {} {tex} from unit {i}", type_name::<TextureT<T>>());
				*tex = 0;
				if empty == npos {
					empty = i;
				}
			} else {
				*used = i;
			}
		}

		if empty == npos {
			FAIL!({ empty = 0 }, "Ran out of GL texture units, {len} available");
		}

		empty
	}
	pub fn Unbind(u: u32) {
		let Unit { bind_count, .. } = bound_units().units.at_mut(u);
		*bind_count -= 1;
	}
	pub fn Clone(u: u32) {
		let Unit { bind_count, .. } = bound_units().units.at_mut(u);
		*bind_count += 1;
	}
	pub fn Bind<T: TexType>(obj: u32, s: u32, hint: u32) -> u32 {
		let Units { ref mut used, len, ref mut units } = *bound_units();

		let Unit { tex, samp, ref mut bind_count } = *units.at_mut(hint);
		if tex == obj && samp == s {
			*bind_count += 1;
			return hint;
		}

		let npos = 1 + len;
		let mut empty = npos;
		for i in 0..*used {
			let Unit { tex, samp, ref mut bind_count } = *units.at_mut(i);
			if tex == obj && samp == s {
				*bind_count += 1;
				return i;
			}
			if empty == npos && (tex == 0 || (tex == obj && samp == 0)) {
				empty = i;
			}
		}

		if empty == npos {
			empty = *used;
			*used += 1;
		}

		if empty >= len {
			empty = garbage_collect::<T>();
		}

		let Unit { tex, samp, bind_count } = units.at_mut(empty);
		*bind_count += 1;
		let u = empty;
		if *tex != obj {
			*tex = obj;
			DEBUG!("Binding GL {} {obj} to unit {u}", type_name::<TextureT<T>>());
			GL!(glBindTextureUnit(T::TYPE, u, obj));
		}
		if *samp != s {
			*samp = s;
			DEBUG!("Binding GL {} {s} to unit {u}", type_name::<SamplerT>());
			GL!(gl::BindSampler(u, s));
		}
		DEBUG!("GL texture units: {units:?}");
		u
	}
	pub fn BindAny<T: TexType>(obj: u32, hint: u32) -> u32 {
		let Units { ref mut used, len, ref mut units } = *bound_units();

		let Unit { tex, ref mut bind_count, .. } = *units.at_mut(hint);
		if tex == obj {
			*bind_count += 1;
			return hint;
		}

		let npos = 1 + len;
		let mut empty = npos;
		for i in 0..*used {
			let Unit { tex, ref mut bind_count, .. } = *units.at_mut(i);
			if tex == obj {
				*bind_count += 1;
				return i;
			}
			if empty == npos && tex == 0 {
				empty = i;
			}
		}

		if empty == npos {
			empty = *used;
			*used += 1;
		}

		if empty >= len {
			empty = garbage_collect::<T>();
		}

		let Unit { tex, bind_count, .. } = units.at_mut(empty);
		*bind_count += 1;
		*tex = obj;
		let u = empty;
		DEBUG!("Binding GL {} {obj} to unit {u}", type_name::<TextureT<T>>());
		GL!(glBindTextureUnit(T::TYPE, u, obj));
		DEBUG!("GL texture units: {units:?}");
		u
	}
	pub fn drop_tex(obj: u32) {
		let Units { ref mut used, len, ref mut units } = *bound_units();
		for i in 0..len {
			let Unit { tex, bind_count: _c, .. } = units.at_mut(i);
			if obj == *tex {
				ASSERT!(*_c == 0, "Leak in GL texture {obj} binding");
				*tex = 0;
				if *used == i + 1 {
					*used = i;
					while *used > 0 && units.at(*used).tex == 0 {
						*used -= 1;
					}
				}
			}
		}
	}
	pub fn drop_samp(s: u32) {
		let Units { len, ref mut units, .. } = *bound_units();
		for i in 0..len {
			let Unit { samp, .. } = units.at_mut(i);
			if s == *samp {
				*samp = 0;
			}
		}
	}
}
