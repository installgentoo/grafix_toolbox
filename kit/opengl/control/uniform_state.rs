use super::{policy::*, *};

struct Locations {
	used: u32,
	len: u32,
	locs: Vec<Loc>,
}
#[derive(Default, Debug, Clone)]
struct Loc {
	buf: u32,
	bind_count: u32,
}

impl ShdBuffType for Uniform {
	fn max_bindings() -> i32 {
		GL::MAX_UNIFORM_BUFFER_BINDINGS()
	}
	fn max_size() -> usize {
		usize(GL::MAX_UNIFORM_BLOCK_SIZE())
	}
}
impl ShdBuffType for ShdStorage {
	fn max_bindings() -> i32 {
		GL::MAX_SHADER_STORAGE_BUFFER_BINDINGS()
	}
	fn max_size() -> usize {
		usize(GL::MAX_SHADER_STORAGE_BLOCK_SIZE())
	}
}

impl<T: ShdBuffType> UniformState<T> {
	fn bound_locs() -> &'static mut Locations {
		LocalStatic!(Locations, {
			let len = u32(T::max_bindings());
			Locations { used: 0, len, locs: vec![Def(); usize(len)] }
		})
	}
	fn garbage_collect() -> u32 {
		let Locations { ref mut used, len, ref mut locs } = *Self::bound_locs();

		let npos = 1 + len;
		let mut empty = npos;
		for i in 0..len {
			let Loc { ref mut buf, bind_count } = *locs.at_mut(i);
			if bind_count == 0 {
				DEBUG!("Unbing GL {} buffer {buf} from location {i}", type_name::<T>());
				*buf = 0;
				if empty == npos {
					empty = i;
				}
			} else {
				*used = i;
			}
		}

		if empty == npos {
			FAIL!({ empty = 0 }, "Ran out of GL {} buffer locations, {len} available", type_name::<T>());
		}

		empty
	}
	pub fn Unbind(l: u32) {
		let Loc { bind_count, .. } = Self::bound_locs().locs.at_mut(l);
		*bind_count -= 1;
	}
	pub fn Clone(l: u32) {
		let Loc { bind_count, .. } = Self::bound_locs().locs.at_mut(l);
		*bind_count += 1;
	}
	pub fn Bind(obj: u32, hint: u32) -> u32 {
		let Locations { ref mut used, len, ref mut locs } = *Self::bound_locs();

		let Loc { buf, ref mut bind_count } = *locs.at_mut(hint);
		if buf == obj {
			*bind_count += 1;
			return hint;
		}

		let npos = 1 + len;
		let mut empty = npos;
		for i in 0..*used {
			let Loc { buf, ref mut bind_count } = *locs.at_mut(i);
			if buf == obj {
				*bind_count += 1;
				return i;
			}
			if empty == npos && buf == 0 {
				empty = i;
			}
		}

		if empty == npos {
			empty = *used;
			*used += 1;
		}

		if empty >= len {
			empty = Self::garbage_collect();
		}

		let Loc { buf, bind_count } = locs.at_mut(empty);
		*bind_count += 1;
		*buf = obj;
		let l = empty;
		DEBUG!("Binding GL {} buffer {obj} to location {l}", type_name::<T>());
		GL!(gl::BindBufferBase(T::TYPE, l, obj));
		DEBUG!("GL buffer binding locations: {locs:?}");
		l
	}
	pub fn BindLocation(obj: u32, l: u32) -> bool {
		let Locations { ref mut locs, len, .. } = *Self::bound_locs();

		let Loc { buf, bind_count } = locs.at_mut(l);
		if l >= len {
			FAIL!({ return false }, "Cannot bind GL {} buffer {obj} to location {l}, {len} available", type_name::<T>());
		}

		if *buf != 0 && *buf != obj {
			DEBUG!("Cannot bind GL {} buffer {obj} to location {l}, occupied by {buf}", type_name::<T>());
			return false;
		}

		*bind_count += 1;

		if *buf == obj {
			return true;
		}

		*buf = obj;
		DEBUG!("Binding GL {} buffer {obj} to location {l}", type_name::<T>());
		GL!(gl::BindBufferBase(T::TYPE, l, obj));
		DEBUG!("GL buffer binding locations: {locs:?}");
		true
	}
	pub fn drop(obj: u32) {
		let Locations { ref mut used, len, ref mut locs } = *Self::bound_locs();
		for i in 0..len {
			let Loc { buf, bind_count: _c } = locs.at_mut(i);
			if obj == *buf {
				ASSERT!(*_c == 0, "Leak in GL uniform buffer {obj} binding");
				*buf = 0;
				if *used == i + 1 {
					*used = i;
					while *used > 0 && locs.at(*used).buf == 0 {
						*used -= 1;
					}
				}
			}
		}
	}
}
pub struct UniformState<T>(Dummy<T>);
