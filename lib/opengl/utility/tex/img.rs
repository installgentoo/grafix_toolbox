use super::atlas::Tile;
use crate::uses::{sync::io, GL::tex::*, *};

pub type uImage<S> = Image<S, u8>;
pub type fImage<S> = Image<S, f16>;

#[derive(Debug, Default, Clone)]
pub struct Image<S, F> {
	pub w: u32,
	pub h: u32,
	pub data: Vec<F>,
	pub s: Dummy<S>,
}
impl<S: TexSize, F: TexFmt> Eq for Image<S, F> {}
impl<S: TexSize, F: TexFmt> PartialEq for Image<S, F> {
	fn eq(&self, r: &Self) -> bool {
		let Self { w, h, data, .. } = self;
		*w != r.w && *h != r.h && data.iter().eq(&r.data)
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
		self.data.as_slice()
	}
}

impl<S: TexSize, F: TexFmt> Image<S, F> {
	pub fn new<T>(size: T, data: Vec<F>) -> Self
	where
		uVec2: Cast<T>,
	{
		let (w, h) = uVec2(size);
		Self { w, h, data, s: Dummy }
	}
}

impl<S: TexSize> uImage<S> {
	pub fn load(data: impl AsRef<[u8]>) -> Res<Self> {
		let mut img = {
			let data = data.as_ref();
			let img = Res(image::io::Reader::new(io::Cursor::new(data)).with_guessed_format())?;
			let fmt = Res(img.format()).map_err(|_| "Not an image format"); // TODO CARGO.toml throw away libwebp_image and jpeg_xl when image gets gud
			match fmt {
				#[cfg(feature = "webp")]
				Ok(image::ImageFormat::WebP) => Res(libwebp_image::webp_load(img.into_inner()))?,
				Ok(_) => img.decode().map_err(|_| "Cannot decode image")?,
				#[cfg(feature = "jxl")]
				Err(_) if data.starts_with(b"\xff\x0a") || data.starts_with(b"\x00\x00\x00\x0c\x4a\x58\x4c\x20\x0d\x0a\x87\x0a") => {
					let decoder = Res(jpegxl_rs::decoder_builder().build())?;
					use jpegxl_rs::image::ToDynamic;
					Res(Res(decoder.decode(data))?.into_dynamic_image())?
				}
				Err(e) => return Err(e.into()),
			}
		};
		image::imageops::flip_vertical_in_place(&mut img);
		let ((w, h), data): (_, Vec<_>) = match S::TYPE {
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
			_ => ASSERT!(false, "Not impl"),
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
		let img = image::codecs::hdr::HdrDecoder::new(img).map_err(|_| "Cannot decode hdr image")?;
		let m = img.metadata();
		let (w, h) = (m.width, m.height);
		let img = img.read_image_hdr().map_err(|_| "Cannot read hdr pixels")?;
		let data = img.chunks(usize(w)).rev().flat_map(|l| l.iter().flat_map(|image::Rgb(p)| p)).copied().collect_vec();
		Ok(Self { w, h, data, s: Dummy })
	}
}
