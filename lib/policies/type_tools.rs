#[macro_export]
macro_rules! impl_for_asref {
	($t: tt, $met: tt, $ret: ty) => {
		impl<T, const L: usize> $t<T> for [T; L] {
			fn $met(&self) -> $ret {
				(&self[..]).$met()
			}
		}
		impl<T> $t<T> for &Vec<T> {
			fn $met(&self) -> $ret {
				(&self[..]).$met()
			}
		}
		impl<T> $t<T> for Vec<T> {
			fn $met(&self) -> $ret {
				(&self).$met()
			}
		}
	};
}
//TODO redo with specialization

#[macro_export]
macro_rules! type_name {
	($t: ty) => {
		type_tools::short_type_name::<$t>()
	};
}

#[macro_export]
macro_rules! type_size {
	($t: ty) => {
		std::mem::size_of::<$t>()
	};
}

pub fn short_type_name<T: ?Sized>() -> String {
	let mut str = std::any::type_name::<T>()
		.split('<')
		.map(|s| [s.split("::").collect::<Vec<_>>().iter().rev().take(1).copied().collect::<String>(), '<'.into()].concat())
		.collect::<String>();
	str.pop();
	str
}
