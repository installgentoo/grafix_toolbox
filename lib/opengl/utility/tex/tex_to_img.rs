use crate::uses::{GL::tex::*, *};

impl<S: TexSize, F: TexFmt, RS, RF> From<&Tex2d<RS, RF>> for Image<S, F> {
	fn from(tex: &Tex2d<RS, RF>) -> Self {
		let ((w, h), data) = (uVec2((tex.param.w, tex.param.h)), tex.Save::<S, F>(0));
		Self { w, h, data, s: Dummy }
	}
}
impl<S: TexSize, F: TexFmt, RS, RF> From<Tex2d<RS, RF>> for Image<S, F> {
	fn from(tex: Tex2d<RS, RF>) -> Self {
		(&tex).into()
	}
}

impl<S: TexSize, F: TexFmt, T: Borrow<Image<S, F>>> From<T> for Tex2d<S, F> {
	fn from(img: T) -> Self {
		let img = img.borrow();
		Tex2d::new((img.w, img.h), &img.data)
	}
}
impl<S: TexSize, F: TexFmt> Tex2d<S, F> {
	pub fn from_type<RS: TexSize, RF: TexFmt>(img: &Image<RS, RF>) -> Self {
		let mut t = Tex2d::new_empty((img.w, img.h));
		t.UpdateCustom::<RS, RF, _>(&img.data);
		t
	}
}

impl<S: TexSize, F: TexFmt> From<&[&Cube<S, F>]> for CubeTex<S, F> {
	fn from(mips: &[&Cube<S, F>]) -> Self {
		let w = i32(mips[0][0].w);
		let l = TexParam::mip_levels(w);
		let p = TexParam { w, h: w, d: 1, l };
		ASSERT!(i32(mips.len()) <= l, "Given {} images, but only {} mip levels", mips.len(), l);
		let l = mips.len();

		let mut t = CubeTex::new_empty((l, w, w));

		mips.iter().enumerate().for_each(|(l, cube)| {
			cube.iter().enumerate().for_each(|(n, i)| {
				debug_assert!({
					let (_w, _h, _) = uVec3(p.dim(l));
					ASSERT!(_w == i.w && _h == i.h, "Mip size at level {} is {:?}, must be {:?}", l, (_w, _h), (i.w, i.h));
					true
				});

				t.Update((&i.data, l, 0, 0, n));
			})
		});
		t
	}
}
impl<S: TexSize, F: TexFmt> From<&Vec<Cube<S, F>>> for CubeTex<S, F> {
	fn from(m: &Vec<Cube<S, F>>) -> Self {
		let m: Vec<_> = m.iter().collect();
		m.as_slice().into()
	}
}
impl<S: TexSize, F: TexFmt> From<Vec<Cube<S, F>>> for CubeTex<S, F> {
	fn from(m: Vec<Cube<S, F>>) -> Self {
		m.into()
	}
}
impl<S: TexSize, F: TexFmt> From<&Cube<S, F>> for CubeTex<S, F> {
	fn from(m: &Cube<S, F>) -> Self {
		[m][..].into()
	}
}
type Cube<S, F> = [Image<S, F>; 6];
