use super::{args::*, format::*, *};

#[derive_as_val]
pub struct TexParam {
	pub w: i32,
	pub h: i32,
	pub d: i32,
	pub l: i32,
}
impl TexParam {
	pub fn gen_mips(mut self) -> Self {
		self.l = self.mips_max();
		self
	}
	pub fn size<A>(&self, lvl: A) -> usize
	where
		u32: Cast<A>,
	{
		let (w, h, d) = ulVec3(self.dim(lvl));
		w * h * d
	}
	pub fn dim<A>(&self, lvl: A) -> iVec3
	where
		u32: Cast<A>,
	{
		let lvl = u32(lvl);
		ASSERT!(i32(lvl) < self.l, "GL texture level {lvl} oob, max {}", self.l);
		self.dim_unchecked(lvl)
	}
	pub fn dim_unchecked(&self, lvl: u32) -> iVec3 {
		let (lvl, Self { w, h, d, .. }) = (u32(lvl), *self);
		if lvl == 0 {
			return (w, h, d);
		}
		let div = |v| 1.max(i32(f64(v) / f64(2_u32.pow(lvl))));
		(div(w), div(h), div(d))
	}
	pub fn mips_max(&self) -> i32 {
		let Self { w, h, d, .. } = *self;
		let w = w.max(h).max(d);
		1 + i32(f64(w).log2())
	}
	pub fn validate(self) -> Self {
		let (_l, _m) = (self.l, self.mips_max());
		ASSERT!(_l > 0 && _l <= _m, "GL texture level {_l} unsound, max {}", _m);
		self
	}
}

#[derive(Debug)]
pub struct Tex<S, F, T: TexType> {
	t: Dummy<(S, F)>,
	param: TexParam,
	tex: Obj<TextureT<T>>,
	unit: Cell<u32>,
}
macro_rules! impl_tex {
	($t: ty, $dim: ident, $arg_u: ident) => {
		impl<S: TexSize, F: TexFmt> Tex<S, F, $t> {
			pub fn none() -> Self {
				Self::new(1, &<[F; 4]>::to([255, 0, 0, 0])[..S::SIZE])
			}
			pub fn new<D>(dimensions: D, args_u: impl $arg_u<F>) -> Self
			where
				$dim: Cast<D>,
			{
				Self::new_empty(dimensions, 1).tap(|t| t.Update(args_u))
			}
			pub fn new_mips<D>(dimensions: D, args_u: impl $arg_u<F>) -> Self
			where
				$dim: Cast<D>,
			{
				Self::new_empty(dimensions, -1)
					.tap(|t| t.Update(args_u))
					.tap(|t| GL!(glGenMipmaps(<$t>::TYPE, t.tex.obj)))
			}
			pub fn new_empty<D, M>(dim: D, mip_levels: M) -> Self
			where
				$dim: Cast<D>,
				i16: Cast<M>,
			{
				let (fmt, tex, l) = (get_internal_fmt::<S, F>().pipe(normalize_internal_fmt), Obj::new(), i16(mip_levels) as i32);
				macro_rules! tex_new {
					(i32) => {{
						let w = i32(dim);
						let p = TexParam { w, h: 1, d: 1, l };
						let p = if l > 0 { p.validate() } else { p.gen_mips() };
						GL!(glTextureStorage1D(<$t>::TYPE, tex.obj, p.l, fmt, w));
						p
					}};
					(iVec2) => {{
						let (w, h) = vec2(dim);
						let p = TexParam { w, h, d: 1, l };
						let p = if l > 0 { p.validate() } else { p.gen_mips() };
						GL!(glTextureStorage2D(<$t>::TYPE, tex.obj, p.l, fmt, w, h));
						p
					}};
					(iVec3) => {{
						let (w, h, d) = vec3(dim);
						let p = TexParam { w, h, d, l };
						let p = if l > 0 { p.validate() } else { p.gen_mips() };
						GL!(glTextureStorage3D(<$t>::TYPE, tex.obj, p.l, fmt, w, h, d));
						p
					}};
				}
				let param = tex_new!($dim);
				Self { t: Dummy, param, tex, unit: Def() }
			}
			pub fn Update(&mut self, args: impl $arg_u<F>) {
				self.UpdateCustom::<S, F, _>(args);
			}
			pub fn UpdateCustom<RS: TexSize, RF: TexFmt, T: $arg_u<RF>>(&mut self, args: T) {
				let mip_size = |lvl, _len| {
					ASSERT!(
						_len <= self.param.size(u32(lvl)) * S::SIZE,
						"GL texture data {_len} at level {lvl} oob, len {}",
						self.param.size(u32(lvl)) * S::SIZE
					);
					self.param.dim(lvl)
				};
				GL::PixelStoreUnpack::Set(1);
				macro_rules! tex_new {
					(UpdArgs1) => {{
						let (data, lvl, x, len) = args.get1();
						let (w, ..) = mip_size(lvl, len);
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
	pub fn param(&self) -> &TexParam {
		&self.param
	}
	pub fn whdl(&self) -> iVec4 {
		let TexParam { w, h, d, l } = self.param;
		vec4((w, h, d, l))
	}
	pub fn obj(&self) -> u32 {
		self.tex.obj
	}
	pub fn Save<RS: TexSize, RF: TexFmt>(&self, lvl: u32) -> Box<[RF]> {
		ASSERT!(T::TYPE != gl::TEXTURE_CUBE_MAP && T::TYPE != gl::TEXTURE_CUBE_MAP_ARRAY, "CUBE NOT IMPL");
		let size = self.param.size(lvl) * RS::SIZE;
		let v = vec![Def(); size].into_boxed_slice();
		let size = i32(size * type_size::<RF>());
		GL::PixelStorePack::Set(1);
		GL!(glGetTexture(T::TYPE, self.tex.obj, i32(lvl), RS::TYPE, RF::TYPE, size, v.as_ptr() as *mut GLvoid));
		v
	}
	pub fn Bind<'l>(&'l self, samp: &'l Sampler) -> TextureBind<'l, T> {
		let unit = self.unit.take();
		let (b, u) = TextureBind::new(&self.tex, samp, unit);
		self.unit.set(u);
		b
	}
}
impl<S, F, T: TexType> Eq for Tex<S, F, T> {}
impl<S, F, T: TexType> PartialEq for Tex<S, F, T> {
	fn eq(&self, r: &Self) -> bool {
		self.tex == r.tex
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
pub struct TextureBind<'l, T> {
	t: Dummy<&'l T>,
	pub u: u32,
}
impl<'l, T: TexType> TextureBind<'l, T> {
	fn new(o: &'l Obj<TextureT<T>>, s: &'l Sampler, hint: u32) -> (Self, u32) {
		let u = TexState::Bind::<T>(o.obj, s.0.obj, hint);
		(Self { t: Dummy, u }, u)
	}
}
impl<T: TexType> Clone for TextureBind<'_, T> {
	fn clone(&self) -> Self {
		let Self { t, u } = *self;
		TexState::Clone(u);
		Self { t, u }
	}
}
impl<T> Drop for TextureBind<'_, T> {
	fn drop(&mut self) {
		TexState::Unbind(self.u);
	}
}
