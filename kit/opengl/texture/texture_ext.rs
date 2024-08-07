use super::{args::*, format::*, spec::*, *};

derive_common_VAL! {
pub struct TexParam {
	pub w: i32,
	pub h: i32,
	pub d: i32,
	pub l: i32,
}}
impl TexParam {
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
		ASSERT!(i32(lvl) < self.l, "GL Texture level {lvl} out of bounds, only has {} levels", self.l);
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
	pub fn mip_levels(w: impl MipsArgs) -> i32 {
		let w = w.getm();
		1 + i32(f64(w).log2())
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
	($t: ty, $arg_n: ident, $arg_u: ident) => {
		impl<S: TexSize, F: TexFmt> Tex<S, F, $t> {
			pub fn new(args_n: impl $arg_n, args_u: impl $arg_u<F>) -> Self {
				let mut tex = Self::new_empty(args_n);
				tex.Update(args_u);
				tex
			}
			pub fn new_empty(args: impl $arg_n) -> Self {
				let (tex, f, s) = (Object::new(), Dummy, Dummy);
				let fmt = normalize_internal_fmt(get_internal_fmt::<S, F>());
				let check = |_m, _l| {
					ASSERT!(
						_l > 0 && _l <= TexParam::mip_levels(_m),
						"GL Texture can only have {} levels, asked for {_l}",
						TexParam::mip_levels(_m)
					);
				};
				macro_rules! tex_new {
					(NewArgs1) => {{
						let (levels, w) = args.get1();
						check(w, levels);
						GL!(glTextureStorage1D(<$t>::TYPE, tex.obj, levels, fmt, w));
						TexParam { w, h: 1, d: 1, l: levels }
					}};
					(NewArgs2) => {{
						let (levels, w, h) = args.get2();
						check((w, h), levels);
						GL!(glTextureStorage2D(<$t>::TYPE, tex.obj, levels, fmt, w, h));
						TexParam { w, h, d: 1, l: levels }
					}};
					(NewArgs3) => {{
						let (levels, w, h, d) = args.get3();
						check((w, h, d), levels);
						GL!(glTextureStorage3D(<$t>::TYPE, tex.obj, levels, fmt, w, h, d));
						TexParam { w, h, d, l: levels }
					}};
				}
				let param = tex_new!($arg_n);
				Self { param, tex, unit: Def(), s, f }
			}
			pub fn Update(&mut self, args: impl $arg_u<F>) {
				self.UpdateCustom::<S, F, _>(args);
			}
			pub fn UpdateCustom<RS: TexSize, RF: TexFmt, T: $arg_u<RF>>(&mut self, args: T) {
				let mip_size = |lvl, _len| {
					ASSERT!(
						_len <= self.param.size(u32(lvl)) * usize(S::SIZE),
						"GL Texture data out of bounds at level {lvl}, size should be {}, given {_len}",
						self.param.size(u32(lvl)) * usize(S::SIZE)
					);
					self.param.dim(lvl)
				};
				GL::PixelStoreUnpack::Set(1);
				macro_rules! tex_new {
					(UpdArgs1) => {{
						let (data, lvl, x, len) = args.geta1();
						let (w, _, _) = mip_size(lvl, len);
						GL!(glTextureSubImage1D(<$t>::TYPE, self.tex.obj, lvl, x, w, RS::TYPE, RF::TYPE, data));
					}};
					(UpdArgs2) => {{
						let (data, lvl, x, y, len) = args.geta2();
						let (w, h, _) = mip_size(lvl, len);
						GL!(glTextureSubImage2D(<$t>::TYPE, self.tex.obj, lvl, x, y, w, h, RS::TYPE, RF::TYPE, data));
					}};
					(UpdArgs3) => {{
						let (data, lvl, x, y, z, len) = args.geta3();
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
	pub fn gen_mips(self) -> Self {
		ASSERT!(self.param.l > 1, "Texture {} was allocated with a single mip level", self.tex.obj);
		GL!(glGenMipmaps(T::TYPE, self.tex.obj));
		self
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
impl_tex!(GL_TEXTURE_1D, NewArgs1, UpdArgs1);
impl_tex!(GL_TEXTURE_2D, NewArgs2, UpdArgs2);
impl_tex!(GL_TEXTURE_3D, NewArgs3, UpdArgs3);
impl_tex!(GL_TEXTURE_1D_ARRAY, NewArgs2, UpdArgs2);
impl_tex!(GL_TEXTURE_2D_ARRAY, NewArgs3, UpdArgs3);
impl_tex!(GL_TEXTURE_CUBE_MAP, NewArgs2, UpdArgs3);
impl_tex!(GL_TEXTURE_CUBE_MAP_ARRAY, NewArgs3, UpdArgs3);

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
