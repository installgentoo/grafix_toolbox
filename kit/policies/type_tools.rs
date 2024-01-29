#[macro_export]
macro_rules! impl_trait_for {
	($trait: ty = $($types: ty),+) => {
		$(impl $trait for $types {})+
	};
}

#[macro_export]
macro_rules! type_name {
	($t: ty) => {{
		type_tools::short_type_name::<$t>()
	}};
}

#[macro_export]
macro_rules! type_size {
	($t: ty) => {{
		std::mem::size_of::<$t>()
	}};
}

pub fn short_type_name<T: ?Sized>() -> String {
	let mut str = std::any::type_name::<T>()
		.split('<')
		.map(|s| [s.split("::").last().unwrap_or(""), "<"].concat())
		.collect::<String>();
	str.pop();
	str
}
