use super::img::*;
use crate::uses::{serde_impl::*, GL::tex::*, *};

impl<S: TexSize, F: TexFmt> Serialize for Tex<GL_TEXTURE_2D, S, F> {
	fn serialize<SE: Serializer>(&self, serializer: SE) -> Result<SE::Ok, SE::Error> {
		ASSERT!(self.param.l == 1, "Not impl mips");
		Image::<S, F>::from(self).serialize(serializer)
	}
}
impl<'de, S: TexSize, F: TexFmt> Deserialize<'de> for Tex<GL_TEXTURE_2D, S, F> {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		Ok(Image::<S, F>::deserialize(deserializer)?.into())
	}
}

impl<S: TexSize, F: TexFmt> Serialize for Image<S, F> {
	fn serialize<SE: Serializer>(&self, serializer: SE) -> Result<SE::Ok, SE::Error> {
		serializer.serialize_bytes(&self.to_bytes())
	}
}
impl<'de, S: TexSize, F: TexFmt> Deserialize<'de> for Image<S, F> {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct V<S, F>(Dummy<S>, Dummy<F>);
		impl<'de, S: TexSize, F: TexFmt> Visitor<'de> for V<S, F> {
			type Value = Image<S, F>;

			fn expecting(&self, formatter: &mut Formatter) -> FmtRes {
				formatter.write_str("Image bytes")
			}
			fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
				Ok(Self::Value::from_bytes(v))
			}
		}

		deserializer.deserialize_bytes(V::<S, F>(Dummy, Dummy))
	}
}

impl<S: TexSize, F: TexFmt> Image<S, F> {
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut v = vec![];
		let Self { w, h, data, .. } = self;
		let w: [_; 4] = w.to_le_bytes();
		let h: [_; 4] = h.to_le_bytes();
		let (_, d, _) = unsafe { data.align_to() };
		v.extend(w.iter().chain(&h).chain(d));
		v
	}
	pub fn from_bytes(v: &[u8]) -> Self {
		let w = u32::from_le_bytes(v[0..4].try_into().unwrap());
		let h = u32::from_le_bytes(v[4..8].try_into().unwrap());
		let data = unsafe { v[8..].align_to() }.1.to_vec();
		Self { w, h, data, s: Dummy }
	}
}
