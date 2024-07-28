use super::*;
use GL::spec::*;

pub trait AnyMesh {
	fn Draw(&self);
	fn to_trait(self) -> Box<dyn AnyMesh>;
}
impl<I: IdxType, C: AttrType, T: AttrType, N: AttrType> AnyMesh for Mesh<I, C, T, N> {
	fn Draw(&self) {
		self.vao.Bind().Draw(self.draw);
	}
	fn to_trait(self) -> Box<dyn AnyMesh> {
		Box(self)
	}
}
pub struct Mesh<I, C, T, N> {
	pub vao: Vao<I>,
	pub buff: (IdxArr<I>, AttrArr<C>, Option<AttrArr<T>>, AttrArr<N>),
	pub draw: (u32, GLenum),
}
impl<I: IdxType, C: AttrType, T: AttrType, N: AttrType> Mesh<I, C, T, N> {
	pub fn new(args: impl MeshArgs<I, C, T, N>) -> Self {
		let (idx, xyz, uv, norm, mode) = args.get();

		let draw = (u32(idx.len()), mode);

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

type MArgs<'s, I, C, T, N> = (&'s [I], &'s [C], Option<&'s [T]>, &'s [N], GLenum);
pub trait MeshArgs<I, C, T, N> {
	fn get(&self) -> MArgs<'_, I, C, T, N>;
}
impl<I: IdxType, C: AttrType, T: AttrType, N: AttrType, SI: AsRef<[I]>, SC: AsRef<[C]>, ST: AsRef<[T]>, SN: AsRef<[N]>> MeshArgs<I, C, T, N> for (SI, SC, ST, SN, GLenum) {
	fn get(&self) -> MArgs<'_, I, C, T, N> {
		(self.0.as_ref(), self.1.as_ref(), Some(self.2.as_ref()), self.3.as_ref(), self.4)
	}
}
impl<I: IdxType, C: AttrType, N: AttrType, SI: AsRef<[I]>, SC: AsRef<[C]>, SN: AsRef<[N]>> MeshArgs<I, C, f16, N> for (SI, SC, SN, GLenum) {
	fn get(&self) -> MArgs<'_, I, C, f16, N> {
		(self.0.as_ref(), self.1.as_ref(), None, self.2.as_ref(), self.3)
	}
}
