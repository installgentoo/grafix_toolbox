use super::*;
use crate::ser::*;

impl<S: TexSize, F: TexFmt> Serialize for Tex2d<S, F> {
	fn serialize<SE: Serializer>(&self, s: SE) -> Result<SE::Ok, SE::Error> {
		ASSERT!(self.param.l == 1, "MIPS NOT IMPL");
		Image::<S, F>::from(self).serialize(s)
	}
}
impl<'de, S: TexSize, F: TexFmt> Deserialize<'de> for Tex2d<S, F> {
	fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
		Ok(Image::<S, F>::deserialize(d)?.into())
	}
}

impl<S: TexSize, F: TexFmt> Serialize for Image<S, F> {
	fn serialize<SE: Serializer>(&self, s: SE) -> Result<SE::Ok, SE::Error> {
		self.to_bytes().serialize(s)
	}
}
impl<'de, S: TexSize, F: TexFmt> Deserialize<'de> for Image<S, F> {
	fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
		Ok(Self::from_bytes(<&[u8]>::deserialize(d)?))
	}
}

impl<S: TexSize, F: TexFmt> Image<S, F> {
	pub fn to_bytes(&self) -> Box<[u8]> {
		let Self { w, h, data, .. } = self;
		let w: [_; 4] = w.to_le_bytes();
		let h: [_; 4] = h.to_le_bytes();
		let (_, d, _) = unsafe { data.align_to() };
		[&w, &h, d].concat().into()
	}
	pub fn from_bytes(v: &[u8]) -> Self {
		let w = u32::from_le_bytes(v[0..4].try_into().valid());
		let h = u32::from_le_bytes(v[4..8].try_into().valid());
		let data = unsafe { v[8..].align_to() }.1.into();
		Self { w, h, data, s: Dummy }
	}
}
