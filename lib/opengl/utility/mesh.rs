use crate::uses::*;
use crate::GL::{buffer::*, spec::*};

pub mod Screen {
	use super::*;
	struct Model {
		vao: Vao<u8>,
		xyuv: AttrArr<i8>,
	}
	pub fn Draw() {
		UnsafeOnce!(Model, {
			#[rustfmt::skip]
			let xyuv = AttrArr::new(&[ -1_i8, -1, 0, 0,  3, -1, 2, 0,  -1, 3, 0, 2 ][..]);
			let mut vao = Vao::new();
			vao.AttribFmt(&xyuv, (0, 4));
			Model { vao, xyuv }
		})
		.vao
		.Bind()
		.DrawUnindexed(3);
	}
	pub fn Prepare() {
		GLEnable!(DEPTH_TEST, BLEND, MULTISAMPLE, DEPTH_WRITEMASK);
		GL::BlendFunc::Set((gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
		GL::DepthFunc::Set(gl::LESS);
	}
}

pub mod Skybox {
	use super::*;
	struct Model {
		vao: Vao<u8>,
		idx: IdxArr<u8>,
		xyz: AttrArr<i8>,
	}
	pub fn Draw() {
		UnsafeOnce!(Model, {
			#[rustfmt::skip]
			let idx = IdxArr::new(&[ 0, 1, 3,  3, 1, 2,
									 4, 5, 7,  7, 5, 6,
									 0, 1, 4,  4, 1, 5,
									 3, 2, 7,  7, 2, 6,
									 2, 1, 6,  6, 1, 5,
									 3, 7, 0,  0, 7, 4, ][..]);
			let (n, p) = (-1_i8, 1_i8);
			#[rustfmt::skip]
			let xyz = AttrArr::new(&[ n, p, p,  p, p, p,  p, p, n,  n, p, n,
									  n, n, p,  p, n, p,  p, n, n,  n, n, n ][..]);
			let mut vao = Vao::new();
			vao.BindIdxs(&idx);
			vao.AttribFmt(&xyz, (0, 3));
			Model { vao, idx, xyz }
		})
		.vao
		.Bind()
		.Draw(36);
	}
}

pub struct Mesh<I: IdxType, C: AttrType, T: AttrType, N: AttrType> {
	pub vao: Vao<I>,
	pub buff: (IdxArr<I>, AttrArr<C>, Option<AttrArr<T>>, AttrArr<N>),
	pub draw: (u32, GLenum),
}
impl<I: IdxType, C: AttrType, T: AttrType, N: AttrType> Mesh<I, C, T, N> {
	pub fn Draw(&mut self) {
		self.vao.Bind().Draw(self.draw);
	}
	pub fn new(args: impl MeshArgs<I, C, T, N>) -> Self {
		let (idx, xyz, uv, norm, mode) = args.get();

		let draw = (u32::to(idx.len()), mode);

		let idx = IdxArr::new(idx);
		let xyz = AttrArr::new(xyz);
		let norm = AttrArr::new(norm);

		let mut vao = Vao::new();
		vao.BindIdxs(&idx);

		let uv = uv.map(|uv| {
			let uv = AttrArr::new(uv);
			vao.AttribFmt(&uv, (1, 2));
			uv
		});
		vao.AttribFmt(&xyz, (0, 3));
		vao.AttribFmt(&norm, (2, 3));
		let buff = (idx, xyz, uv, norm);

		Self { vao, buff, draw }
	}
}

pub trait MeshArgs<I, C, T, N> {
	fn get(&self) -> (&[I], &[C], Option<&[T]>, &[N], GLenum);
}
impl<I: IdxType, C: AttrType, T: AttrType, N: AttrType, SI: AsRef<[I]>, SC: AsRef<[C]>, ST: AsRef<[T]>, SN: AsRef<[N]>> MeshArgs<I, C, T, N> for (SI, SC, ST, SN, GLenum) {
	fn get(&self) -> (&[I], &[C], Option<&[T]>, &[N], GLenum) {
		(self.0.as_ref(), self.1.as_ref(), Some(self.2.as_ref()), self.3.as_ref(), self.4)
	}
}
impl<I: IdxType, C: AttrType, N: AttrType, SI: AsRef<[I]>, SC: AsRef<[C]>, SN: AsRef<[N]>> MeshArgs<I, C, f16, N> for (SI, SC, SN, GLenum) {
	fn get(&self) -> (&[I], &[C], Option<&[f16]>, &[N], GLenum) {
		(self.0.as_ref(), self.1.as_ref(), None, self.2.as_ref(), self.3)
	}
}
