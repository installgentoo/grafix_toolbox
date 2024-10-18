use super::{args::*, format::*, spec::*, *};

derive_common_VAL! {
pub struct TexParam {
	pub w: i32,
	pub h: i32,
	pub d: i32,
	pub l: i32,
}}
impl TexParam {
	pub fn gen_mips(mut self) -> Self {
		self.l = self.mips_max();
		self
	}
	pub fn size<T>(self, lvl: T) -> usize
	where
		u32: Cast<T>,
	{
		let (w, h, d) = ulVec3(self.dim(lvl));
		w * h * d
	}
	pub fn dim<T>(self, lvl: T) -> iVec3
	where
		u32: Cast<T>,
	{
		let lvl = u32(lvl);
		ASSERT!(i32(lvl) < self.l, "Texture level {lvl} out of bounds, only has {} levels", self.l);
		self.dim_unchecked(lvl)
	}
	pub fn dim_unchecked(self, lvl: u32) -> iVec3 {
		let (lvl, Self { w, h, d, .. }) = (u32(lvl), self);
		if lvl == 0 {
			return (w, h, d);
		}
		let div = |v| 1.max(i32((f64(v) / f64(2_u32.pow(lvl))).floor()));
		(div(w), div(h), div(d))
	}
	pub fn mips_max(&self) -> i32 {
		let &Self { w, h, d, .. } = self;
		let w = w.max(h).max(d);
		1 + i32(f64(w).log2())
	}
	pub fn validate(self) -> Self {
		let (_l, _m) = (self.l, self.mips_max());
		ASSERT!(_l > 0 && _l <= _m, "Texture dimensions allow 1 to {} levels, asked for {_l}", _m);
		self
	}
}

#[derive(Default, Debug)]
pub struct Tex<S, F, T: TexType> {
	pub param: TexParam,
	tex: Object<Texture<T>>,
	unit: Cell<u32>,
	s: Dummy<S>,
	f: Dummy<F>,
}
macro_rules! impl_tex {
	($t: ty, $dim: ident, $arg_u: ident) => {
		impl<S: TexSize, F: TexFmt> Tex<S, F, $t> {
			pub fn new<D>(dimensions: D, args_u: impl $arg_u<F>) -> Self
			where
				$dim: Cast<D>,
			{
				let mut tex = Self::new_empty(dimensions, 1);
				tex.Update(args_u);
				tex
			}
			pub fn new_mips<D>(dimensions: D, args_u: impl $arg_u<F>) -> Self
			where
				$dim: Cast<D>,
			{
				let mut tex = Self::new_empty(dimensions, -1);
				tex.Update(args_u);
				GL!(glGenMipmaps(<$t>::TYPE, tex.tex.obj));
				tex
			}
			pub fn new_empty<D, M>(dim: D, mip_levels: M) -> Self
			where
				$dim: Cast<D>,
				i16: Cast<M>,
			{
				let (tex, f, s, l) = (Object::new(), Dummy, Dummy, i16(mip_levels) as i32);
				let fmt = normalize_internal_fmt(get_internal_fmt::<S, F>());
				macro_rules! tex_new {
					(i32) => {{
						let w = i32(dim);
						let p = TexParam { w, h: 1, d: 1, l };
						let p = if l > 0 { p.validate() } else { p.gen_mips() };
						GL!(glTextureStorage1D(<$t>::TYPE, tex.obj, p.l, fmt, w));
						p
					}};
					(iVec2) => {{
						let (w, h) = iVec2(dim);
						let p = TexParam { w, h, d: 1, l };
						let p = if l > 0 { p.validate() } else { p.gen_mips() };
						GL!(glTextureStorage2D(<$t>::TYPE, tex.obj, p.l, fmt, w, h));
						p
					}};
					(iVec3) => {{
						let (w, h, d) = iVec3(dim);
						let p = TexParam { w, h, d, l };
						let p = if l > 0 { p.validate() } else { p.gen_mips() };
						GL!(glTextureStorage3D(<$t>::TYPE, tex.obj, p.l, fmt, w, h, d));
						p
					}};
				}
				let param = tex_new!($dim);
				Self { param, tex, unit: Def(), s, f }
			}
			pub fn Update(&mut self, args: impl $arg_u<F>) {
				self.UpdateCustom::<S, F, _>(args);
			}
			pub fn UpdateCustom<RS: TexSize, RF: TexFmt, T: $arg_u<RF>>(&mut self, args: T) {
				let mip_size = |lvl, _len| {
					ASSERT!(
						_len <= self.param.size(u32(lvl)) * usize(S::SIZE),
						"Texture data out of bounds at level {lvl}, size should be {}, given {_len}",
						self.param.size(u32(lvl)) * usize(S::SIZE)
					);
					self.param.dim(lvl)
				};
				GL::PixelStoreUnpack::Set(1);
				macro_rules! tex_new {
					(UpdArgs1) => {{
						let (data, lvl, x, len) = args.get1();
						let (w, _, _) = mip_size(lvl, len);
						GL!(glTextureSubImage1D(<$t>::TYPE, self.tex.obj, lvl, x, w, RS::TYPE, RF::TYPE, data));
					}};
					(UpdArgs2) => {{
						let (data, lvl, x, y, len) = args.get2();
						let (w, h, _) = mip_size(lvl, len);
						GL!(glTextureSubImage2D(<$t>::TYPE, self.tex.obj, lvl, x, y, w, h, RS::TYPE, RF::TYPE, data));
					}};
					(UpdArgs3) => {{
						let (data, lvl, x, y, z, len) = args.get3();
						let (w, h, d) = mip_size(lvl, len);
						GL!(glTextureSubImage3D(<$t>::TYPE, self.tex.obj, lvl, x, y, z, w, h, d, RS::TYPE, RF::TYPE, data));
					}};
				}
				tex_new!($arg_u);
			}
		}
	};
}
impl<S, F, T: TexType> Tex<S, F, T> {
	pub fn obj(&self) -> u32 {
		self.tex.obj
	}
	pub fn Save<RS: TexSize, RF: TexFmt>(&self, lvl: u32) -> Box<[RF]> {
		ASSERT!(T::TYPE != gl::TEXTURE_CUBE_MAP && T::TYPE != gl::TEXTURE_CUBE_MAP_ARRAY, "unimpl");
		let size = self.param.size(lvl) * usize(RS::SIZE);
		let v = vec![Def(); size].into_boxed_slice();
		let size = i32(size * type_size::<RF>());
		GL::PixelStorePack::Set(1);
		GL!(glGetTexture(T::TYPE, self.tex.obj, i32(lvl), RS::TYPE, RF::TYPE, size, v.as_ptr() as *mut GLvoid));
		v
	}
	pub fn Bind<'l>(&'l self, samp: &'l Sampler) -> TextureBinding<T> {
		let unit = self.unit.take();
		let (b, u) = TextureBinding::new(&self.tex, samp, unit);
		self.unit.set(u);
		b
	}
}
impl_tex!(GL_TEXTURE_1D, i32, UpdArgs1);
impl_tex!(GL_TEXTURE_2D, iVec2, UpdArgs2);
impl_tex!(GL_TEXTURE_3D, iVec3, UpdArgs3);
impl_tex!(GL_TEXTURE_1D_ARRAY, iVec2, UpdArgs2);
impl_tex!(GL_TEXTURE_2D_ARRAY, iVec3, UpdArgs3);
impl_tex!(GL_TEXTURE_CUBE_MAP, iVec2, UpdArgs3);
impl_tex!(GL_TEXTURE_CUBE_MAP_ARRAY, iVec3, UpdArgs3);

#[derive(Debug)]
pub struct TextureBinding<'l, T> {
	t: Dummy<&'l T>,
	pub u: u32,
}
impl<'l, T: TexType> TextureBinding<'l, T> {
	pub fn new(o: &'l Object<Texture<T>>, samp: &'l Sampler, hint: u32) -> (Self, u32) {
		let u = TexState::Bind::<T>(o.obj, samp.obj, hint);
		(Self { t: Dummy, u }, u)
	}
}
impl<T: TexType> Clone for TextureBinding<'_, T> {
	fn clone(&self) -> Self {
		let &Self { t, u } = self;
		TexState::Clone(u);
		Self { t, u }
	}
}
impl<T> Drop for TextureBinding<'_, T> {
	fn drop(&mut self) {
		TexState::Unbind(self.u);
	}
}
