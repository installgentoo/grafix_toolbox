use super::*;
use std::{io, path::Path};

type Load<'s> = Res<&'s [u8]>;
pub trait LoadArgs {
	fn get(&self) -> Load;
}
impl LoadArgs for &[u8] {
	fn get(&self) -> Load {
		Ok(self)
	}
}
impl<T: AsRef<[u8]>> LoadArgs for Res<T> {
	fn get(&self) -> Load {
		self.as_ref().map(<_>::as_ref).res()
	}
}

impl<S: TexSize> uImage<S> {
	pub fn load(data: impl LoadArgs) -> Res<Self> {
		let img = data
			.get()?
			.pipe(io::Cursor::new)
			.pipe(image::ImageReader::new)
			.with_guessed_format()
			.res()?
			.decode()
			.explain_err(|| "Cannot decode image")?;

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
		Self { w, h, data, s: Dummy }.pipe(Ok)
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
		let Self { w, h, ref data, .. } = *self;
		image::save_buffer(name, data, w, h, t).explain_err(|| "Cannot save image").warn();
	}
}

#[cfg(feature = "hdr")]
impl Image<RGB, f32> {
	pub fn load(data: impl LoadArgs) -> Res<Self> {
		let img = data
			.get()?
			.pipe(io::Cursor::new)
			.pipe(io::BufReader::new)
			.pipe(image::codecs::hdr::HdrDecoder::new)
			.explain_err(|| "Cannot decode hdr image")?
			.pipe(image::DynamicImage::from_decoder)
			.explain_err(|| "Cannot decode hdr pixels")?
			.tap(image::imageops::flip_vertical_in_place)
			.into_rgb32f();

		let ((w, h), data) = (img.dimensions(), img.pixels().flat_map(|image::Rgb(p)| p).copied().collect());
		Self { w, h, data, s: Dummy }.pipe(Ok)
	}
}
