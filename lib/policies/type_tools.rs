#[macro_export]
macro_rules! impl_for_asref {
	($t: tt, $f: tt, $r: ty) => {
		impl<T, const L: usize> $t<T> for [T; L] {
			fn $f(&self) -> $r {
				(&self[..]).$f()
			}
		}
		impl<T> $t<T> for &Vec<T> {
			fn $f(&self) -> $r {
				(&self[..]).$f()
			}
		}
		impl<T> $t<T> for Vec<T> {
			fn $f(&self) -> $r {
				(&self).$f()
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
		.split("<")
		.map(|s| [s.split("::").collect::<Vec<_>>().iter().rev().take(1).map(|&s| s).collect::<String>(), '<'.into()].concat())
		.collect::<String>();
	str.pop();
	str
}
