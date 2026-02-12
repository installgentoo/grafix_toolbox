#[cfg(not(feature = "adv_fs"))]
pub mod ser {}
#[cfg(feature = "adv_fs")]
pub mod ser {
	pub use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

	pub fn from_vec<R: de::DeserializeOwned>(v: impl Borrow<[u8]>) -> Res<R> {
		Ok(serde_json::from_slice(v.borrow()).fail())
	}

	pub fn to_vec<T: Serialize>(v: T) -> Res<Vec<u8>> {
		serde_json::to_vec(&v).res()
	}

	pub fn make_true() -> bool {
		true
	}

	pub mod as_byte_slice {
		pub fn serialize<T, S: Serializer>(v: &[T], s: S) -> Result<S::Ok, S::Error> {
			let (_, b, _) = unsafe { v.align_to::<u8>() };
			let b = faster_hex::hex_string(b);
			(v.len(), b).serialize(s)
		}
		pub fn deserialize<'de, T: Copy + Default, D: Deserializer<'de>>(d: D) -> Result<Box<[T]>, D::Error> {
			let (l, s) = <(usize, String)>::deserialize(d)?;
			let s = s.as_bytes();

			if s.len() != l * type_size::<T>() * 2 {
				Err(de::Error::custom(format!("Cannot deserialize [Hex; {}] into [{}; {l}]", s.len(), type_name::<T>())))?
			}

			let mut aligned = vec![T::default(); l];
			{
				let a = unsafe { std::slice::from_raw_parts_mut(aligned.as_mut_ptr() as *mut u8, s.len() / 2) };
				faster_hex::hex_decode(s, a).explain_err(|| "Hex decode failed").map_err(de::Error::custom)?;
			}

			Ok(aligned.into())
		}
		use super::*;
	}

	use crate::lib::*;
}
