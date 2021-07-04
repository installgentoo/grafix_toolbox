use super::atlas::Tile;
use crate::uses::{GL::tex::*, sync::io, *};

pub type uImage<S> = Image<S, u8>;
pub type fImage<S> = Image<S, f16>;

#[derive(Default, Debug)]
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
		i32::to(self.w)
	}
	fn h(&self) -> i32 {
		i32::to(self.h)
	}
	fn data(&self) -> &[F] {
		self.data.as_slice()
	}
}

impl<S: TexSize> uImage<S> {
	pub fn new<T: AsRef<[u8]>>(data: T) -> Res<Self> {
		let mut img = Reader::new(io::Cursor::new(data.as_ref()))
			.with_guessed_format()
			.map_err(|_| "Not an image fromat")?
			.decode()
			.map_err(|_| "Cannot decode image")?;
		imageops::flip_horizontal_in_place(&mut img);
		let ((w, h), data) = match S::TYPE {
			gl::RED => {
				let img = img.into_luma8();
				(img.dimensions(), img.pixels().flat_map(|image::Luma(p)| p).cloned().collect::<Vec<_>>())
			}
			gl::RGB => {
				let img = img.into_rgb8();
				(img.dimensions(), img.pixels().flat_map(|image::Rgb(p)| p).cloned().collect::<Vec<_>>())
			}
			gl::RGBA => {
				let img = img.into_rgba8();
				(img.dimensions(), img.pixels().flat_map(|image::Rgba(p)| p).cloned().collect::<Vec<_>>())
			}
			_ => ASSERT!(false, "Not impl"),
		};
		Ok(Self { w, h, data, s: Dummy })
	}
}

impl Image<RGB, f32> {
	pub fn new<T: AsRef<[u8]>>(data: T) -> Res<Self> {
		let img = io::BufReader::new(io::Cursor::new(data.as_ref()));
		let img = hdr::HdrDecoder::new(img).map_err(|_| "Cannot decode hdr image")?;
		let ((w, h), data) = {
			let m = img.metadata();
			let (w, h) = (m.width, m.height);
			let img = img.read_image_hdr().map_err(|_| "Cannot read hdr pixels")?;
			(
				(w, h),
				img.chunks(w as usize).rev().flat_map(|l| l.iter().flat_map(|image::Rgb(p)| p)).cloned().collect::<Vec<_>>(),
			)
		};
		Ok(Self { w, h, data, s: Dummy })
	}
}
use image::{codecs::hdr, imageops, io::Reader};
