#[macro_export]
macro_rules! impl_trait_for {
	($trait: ty = $($types: ty),+) => {
		$(impl $trait for $types {})+
	};
}

pub fn type_name<T: ?Sized>() -> String {
	let mut str = std::any::type_name::<T>()
		.split('<')
		.map(|s| [s.split("::").last().unwrap_or(""), "<"].concat())
		.collect::<String>();
	str.pop();
	str
}

#[macro_use]
pub mod derives;

#[macro_use]
pub mod pointer;
#[macro_use]
mod logging_def;

pub mod chksum;
pub mod event;
pub mod ext;
pub mod file;
pub mod index;
pub mod logging;
pub mod math;
pub mod profiling;
pub mod rand;
pub mod result;
pub mod slicing;
pub mod window;
