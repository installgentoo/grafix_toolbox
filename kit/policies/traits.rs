use std::fmt::Debug;

#[macro_export]
macro_rules! impl_trait_for {
	($trait: ty = $($types: ty),+) => {
		$(impl $trait for $types {})+
	};
}

// TODO replace with trait aliases
macro_rules! trait_alias {
	($t: ident, $($b: tt)+ ) => {
		pub trait $t: $($b)+ {}
		impl<T: $($b)+> $t for T {}
	};
}

trait_alias!(SendStat, 'static + Send);

#[cfg(feature = "adv_fs")]
trait_alias!(TrivialBound, 'static + Debug + Default + Copy + PartialEq + serde::Serialize + serde::de::DeserializeOwned);
#[cfg(not(feature = "adv_fs"))]
trait_alias!(TrivialBound, 'static + Debug + Default + Copy + PartialEq);

macro_rules! derive_common_VAL {
	($($t: tt)+) => {
		#[derive(Default, Debug, Clone, Copy, PartialEq)]
		#[cfg_attr(feature = "adv_fs", derive(serde::Serialize, serde::Deserialize))]
		$($t)+
	};
}

macro_rules! derive_common_OBJ {
	($($t: tt)+) => {
		#[derive(Default, Debug, Clone)]
		#[cfg_attr(feature = "adv_fs", derive(serde::Serialize, serde::Deserialize))]
		$($t)+
	};
}
