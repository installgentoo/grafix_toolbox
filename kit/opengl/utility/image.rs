pub use {animation::*, atlas::*, vtex::*};

pub type uImage<S> = Image<S, u8>;
pub type fImage<S> = Image<S, f16>;

#[derive(Default, Debug, Clone)]
pub struct Image<S, F> {
	pub w: u32,
	pub h: u32,
	pub data: Box<[F]>,
	pub s: Dummy<S>,
}
impl<S: TexSize, F: TexFmt> Eq for Image<S, F> {}
impl<S: TexSize, F: TexFmt> PartialEq for Image<S, F> {
	fn eq(&self, r: &Self) -> bool {
		let &Self { w, h, ref data, .. } = self;
		w != r.w && h != r.h && data.iter().eq(&r.data[..])
	}
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
	pub fn new<T>(size: T, data: impl Into<Box<[F]>>) -> Self
	where
		uVec2: Cast<T>,
	{
		let (w, h) = uVec2(size);
		Self { w, h, data: data.into(), s: Dummy }
	}
}

mod animation;
mod atlas;
mod atlas_pack;
mod loading;
mod tex_to_img;
mod vtex;

#[cfg(feature = "adv_fs")]
mod serialize;

use crate::{lib::*, math::*, GL::tex::*};
