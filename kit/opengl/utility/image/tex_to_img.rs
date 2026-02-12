use {super::*, GL::*};

#[cfg(feature = "adv_fs")]
mod _ser {
	use super::{ser::*, *};
	impl<S: TexSize, F: TexFmt> Serialize for Tex2d<S, F> {
		fn serialize<SE: Serializer>(&self, s: SE) -> Result<SE::Ok, SE::Error> {
			ASSERT!(self.param().l == 1, "MIPS NOT IMPL");
			Image::<S, F>::from(self).serialize(s)
		}
	}
	impl<'de, S: TexSize, F: TexFmt> Deserialize<'de> for Tex2d<S, F> {
		fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
			Ok(Image::<S, F>::deserialize(d)?.into())
		}
	}
}

impl<S: TexSize, F: TexFmt> Tex2d<S, F> {
	pub fn Cut(&self, region: iVec4) -> Tex2d<S, F> {
		let _s = self.whdl().xy();
		ASSERT!(region.gt(0).all() && region.zw().sub(region.xy()).ls(_s).all(), "Cutting invalid texture region");
		let img: Image<_, _> = self.into(); //TODO use glCopyTexSubImage2D, move to texture
		img.cut(vec4(region)).into()
	}
	pub fn Cast<RS: TexSize, RF: TexFmt>(&self, minification: i32) -> Tex2d<RS, RF> {
		let s = LeakyStatic!(Shader, { [vs_mesh__2d_screen, ps_mesh__2d_screen].pipe(Shader::pure) });
		let sampl = &Sampler::linear();

		GLSave!(BLEND);
		GLDisable!(BLEND);

		let out = self.whdl().xy().div(minification).fmax(1).pipe(Fbo::new);
		let t = self.Bind(sampl);
		let _ = Uniforms!(s, ("iTex", t));
		out.bind();
		Screen::Draw();

		GLRestore!(BLEND);
		out.tex
	}
}

impl<S: TexSize, F: TexFmt, RS, RF> From<&Tex2d<RS, RF>> for Image<S, F> {
	fn from(tex: &Tex2d<RS, RF>) -> Self {
		let ((w, h), data) = (uVec2(tex.whdl().xy()), tex.Save::<S, F>(0));
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
		Tex2d::new((img.w, img.h), &img.data[..])
	}
}
impl<S: TexSize, F: TexFmt> Tex2d<S, F> {
	pub fn from_type<RS: TexSize, RF: TexFmt>(img: &Image<RS, RF>) -> Self {
		Tex2d::new_empty((img.w, img.h), 1).tap(|t| t.UpdateCustom::<RS, RF, _>(&img.data[..]))
	}
}

impl<S: TexSize, F: TexFmt> From<&[&Cube<S, F>]> for CubeTex<S, F> {
	fn from(mips: &[&Cube<S, F>]) -> Self {
		let w = i32(mips[0][0].w);
		let p = TexParam { w, h: w, d: 1, l: i32(mips.len()) }.validate();
		CubeTex::new_empty((p.w, p.h), p.l).tap(|t| {
			mips.iter().enumerate().for_each(|(l, cube)| {
				cube.iter().enumerate().for_each(|(n, i)| {
					debug_assert!({
						let (_w, _h, _) = uVec3(p.dim(l));
						ASSERT!(_w == i.w && _h == i.h, "Mip size {:?} at level {l}, must be {:?}", (_w, _h), (i.w, i.h));
						true
					});

					t.Update((&i.data, l, 0, 0, n));
				})
			})
		})
	}
}
impl<S: TexSize, F: TexFmt> From<&[Cube<S, F>]> for CubeTex<S, F> {
	fn from(m: &[Cube<S, F>]) -> Self {
		m.iter().collect_vec().as_slice().into()
	}
}
impl<S: TexSize, F: TexFmt> From<&Cube<S, F>> for CubeTex<S, F> {
	fn from(m: &Cube<S, F>) -> Self {
		[m][..].into()
	}
}
type Cube<S, F> = [Image<S, F>; 6];
