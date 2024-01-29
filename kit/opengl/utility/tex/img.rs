use super::atlas::Tile;
use crate::{lib::*, GL::tex::*};
use std::{io, path::Path};

pub type uImage<S> = Image<S, u8>;
pub type fImage<S> = Image<S, f16>;

#[derive(Debug, Default, Clone)]
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

impl<S: TexSize> uImage<S> {
	pub fn load(data: impl AsRef<[u8]>) -> Res<Self> {
		let mut img = {
			let data = data.as_ref();
			Res(image::io::Reader::new(io::Cursor::new(data)).with_guessed_format())?
				.decode()
				.map_err(|e| format!("Cannot decode image: {e:?}"))?
		};
		image::imageops::flip_vertical_in_place(&mut img);
		let ((w, h), data) = match S::TYPE {
			gl::RED => {
				let img = img.into_luma8();
				(img.dimensions(), img.pixels().flat_map(|image::Luma(p)| p).copied().collect())
			}
			gl::RGB => {
				let img = img.into_rgb8();
				(img.dimensions(), img.pixels().flat_map(|image::Rgb(p)| p).copied().collect())
			}
			gl::RGBA => {
				let img = img.into_rgba8();
				(img.dimensions(), img.pixels().flat_map(|image::Rgba(p)| p).copied().collect())
			}
			_ => ERROR!("Not impl"),
		};
		Ok(Self { w, h, data, s: Dummy })
	}
	pub fn save(&self, name: impl AsRef<Path>) {
		use image::ColorType::*;
		let t = match S::SIZE {
			1 => L8,
			2 => La8,
			3 => Rgb8,
			4 => Rgba8,
			_ => unreachable!(),
		};
		EXPECT!(image::save_buffer(name, &self.data, self.w, self.h, t));
	}
}

#[cfg(feature = "hdr")]
impl Image<RGB, f32> {
	pub fn load(data: impl AsRef<[u8]>) -> Res<Self> {
		let img = io::BufReader::new(io::Cursor::new(data.as_ref()));
		let img = image::codecs::hdr::HdrDecoder::new(img).map_err(|e| format!("Cannot decode hdr image: {e:?}"))?;
		let m = img.metadata();
		let (w, h) = (m.width, m.height);
		let img = img.read_image_hdr().map_err(|e| format!("Cannot read hdr pixels: {e:?}"))?;
		let data = img.chunks(usize(w)).rev().flat_map(|l| l.iter().flat_map(|image::Rgb(p)| p)).copied().collect();
		Ok(Self { w, h, data, s: Dummy })
	}
}
