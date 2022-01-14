use crate::uses::*;

#[cfg(feature = "adv_fs")]
trait_set! { pub trait TrivialBound = 'static + Debug + Default + Copy + PartialEq + Serialize + DeserializeOwned }
#[cfg(not(feature = "adv_fs"))]
trait_set! { pub trait TrivialBound = 'static + Debug + Default + Copy + PartialEq }

macro_rules! derive_common_VAL {
	($($t: tt)+) => {
		#[derive(Debug, Default, Clone, Copy, PartialEq)]
		#[cfg_attr(feature = "adv_fs", derive(Serialize, Deserialize))]
		$($t)+
	};
}

macro_rules! derive_common_OBJ {
	($($t: tt)+) => {
		#[derive(Debug, Default, Clone)]
		#[cfg_attr(feature = "adv_fs", derive(Serialize, Deserialize))]
		$($t)+
	};
}
