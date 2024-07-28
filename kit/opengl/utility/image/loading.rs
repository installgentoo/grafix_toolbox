use super::*;
use std::{io, path::Path};

type Load<'s> = Res<&'s [u8]>;
pub trait LoadArgs {
	fn get(&self) -> Load<'_>;
}
impl LoadArgs for &[u8] {
	fn get(&self) -> Load<'_> {
		Ok(self)
	}
}
impl<T: AsRef<[u8]>> LoadArgs for Res<T> {
	fn get(&self) -> Load<'_> {
		match self.as_ref() {
			Ok(f) => Ok(f.as_ref()),
			Err(e) => Err(e.clone()),
		}
	}
}

impl<S: TexSize> uImage<S> {
	pub fn load(data: impl LoadArgs) -> Res<Self> {
		let mut img = {
			Res(image::ImageReader::new(io::Cursor::new(data.get()?)).with_guessed_format())?
				.decode()
				.explain_err(|| "Cannot decode image")?
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
		if let Err(e) = image::save_buffer(name, &self.data, self.w, self.h, t) {
			FAIL!("Cannot save image {e:?}");
		}
	}
}

#[cfg(feature = "hdr")]
impl Image<RGB, f32> {
	pub fn load(data: impl LoadArgs) -> Res<Self> {
		let img = io::BufReader::new(io::Cursor::new(data.get()?));
		let img = image::codecs::hdr::HdrDecoder::new(img).explain_err(|| format!("Cannot decode hdr image"))?;
		let mut img = image::DynamicImage::from_decoder(img).explain_err(|| format!("Cannot decode hdr pixels"))?;
		image::imageops::flip_vertical_in_place(&mut img);
		let img = img.into_rgb32f();
		let ((w, h), data) = (img.dimensions(), img.pixels().flat_map(|image::Rgb(p)| p).copied().collect());
		Ok(Self { w, h, data, s: Dummy })
	}
}
