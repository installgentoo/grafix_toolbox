pub mod ext {
	pub use super::func::{chksum, chksum::ref_UUID, file as FS, logger, rand, serde::ser, slicing};
	pub use super::{memory::Ptr, pre::lazy, profiling};
}

#[macro_export]
macro_rules! trait_alias {
	($p: vis $t: ident, $($b: tt)+) => {
		$p trait $t: $($b)+ {}
		impl<S: $($b)+> $t for S {} // TODO replace with trait aliases
	};
	($t: ident, $($b: tt)+) => {
		trait_alias!(pub(self) $t, $($b)+);
	};
}

pub mod pre {
	pub use super::func::{ext::*, faster::*, n_iter::*, range::*, result::*, vec::*};
	pub use super::{math::pre::*, types::*};

	pub fn type_name<T: ?Sized>() -> String {
		std::any::type_name::<T>()
			.split_inclusive(['<', '>', '(', ')', ','])
			.map(|s| if let Some(i) = s.rfind("::") { &s[i + 2..] } else { s })
			.collect()
	}

	trait_alias!(pub SendS, 'static + Send);

	#[cfg(feature = "adv_fs")]
	trait_alias!(pub TrivialBound, 'static + std::fmt::Debug + Default + Copy + PartialEq + serde::Serialize + serde::de::DeserializeOwned);
	#[cfg(not(feature = "adv_fs"))]
	trait_alias!(pub TrivialBound, 'static + std::fmt::Debug + Default + Copy + PartialEq);

	pub trait Fut<T>: Future<Output = T> + Send {} // TODO drop when AsyncFn is usable
	impl<T, S: Future<Output = T> + Send> Fut<T> for S {}
}

#[macro_export]
macro_rules! impl_trait_for {
	($trait: ty = $($types: ty),+) => {
		$(impl $trait for $types {})+
	};
}

#[macro_export]
macro_rules! map_variant {
	($t: pat = $e: expr => $do: expr) => {{ if let $t = $e { Some($do) } else { None } }};
}
#[macro_export]
macro_rules! or_map {
	($v: ident = $t: pat => $do: expr) => {{
		if let $t = $v {
			*$v = $do
		}
	}};
}

#[macro_use]
mod memory;
#[macro_use]
mod func;
#[macro_use]
mod types;

pub mod math;
pub mod profiling;
pub mod task;
