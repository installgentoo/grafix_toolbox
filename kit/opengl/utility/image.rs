pub use {animation::*, atlas::*, vtex::*};

pub type uImage<S> = Image<S, u8>;
pub type fImage<S> = Image<S, f16>;

#[derive_as_obj(Default, PartialEq)]
pub struct Image<S, F: TexFmt> {
	s: Dummy<S>,
	pub w: u32,
	pub h: u32,
	#[cfg_attr(feature = "adv_fs", serde(with = "ser::as_byte_slice"))]
	pub data: Box<[F]>,
}
impl<S: TexSize, F: TexFmt> Tile<F> for Image<S, F> {
	fn w(&self) -> i32 {
		i32(self.w)
	}
	fn h(&self) -> i32 {
		i32(self.h)
	}
	fn data(&self) -> &[F] {
		&self.data
	}
}
impl<S: TexSize, F: TexFmt> Image<S, F> {
	pub fn new<A>(size: A, data: impl Into<Box<[F]>>) -> Self
	where
		uVec2: Cast<A>,
	{
		let (w, h) = vec2(size);
		Self { s: Dummy, w, h, data: data.into() }
	}
	pub fn cut(&self, _region @ (x1, y1, x2, y2): ulVec4) -> Self {
		let Self { w, h, ref data, .. } = *self;
		ASSERT!(
			_region.gt(0).all() && _region.zw().gt(_region.xy()).all() && _region.zw().sub(_region.xy()).ls((w, h)).all(),
			"Cutting invalid image region"
		);
		let (w, s) = ulVec2((w, S::SIZE));
		let mut d = vec![];
		for y in y1..y2 {
			let b = (y * w + x1) * s;
			let e = b + (x2 - x1) * s;
			d.extend_from_slice(&data[b..e]);
		}
		let (w, h) = vec2((x2 - x1, y2 - y1));
		Self { s: Dummy, w, h, data: d.into() }
	}
}

mod animation;
mod atlas;
mod atlas_pack;
mod loading;
mod tex_to_img;
mod vtex;

use crate::{GL::tex::*, lib::*, math::*};
