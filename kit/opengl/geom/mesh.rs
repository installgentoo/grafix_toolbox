use super::*;

type BuffMem = Box<dyn ArrObjLease>;
pub trait BuffArg {
	fn set<I: IdxType>(self, _: u32, _: &mut Vao<I>) -> BuffMem;
}
impl<A: AttrType> BuffArg for (u32, &[A]) {
	fn set<I: IdxType>(self, at: u32, v: &mut Vao<I>) -> BuffMem {
		let (dim, arr) = self;
		let arr = AttrArr::new((arr, 0));
		v.AttribFmt(&arr, (at, dim));
		Box(arr)
	}
}
impl BuffArg for () {
	fn set<I: IdxType>(self, _: u32, _: &mut Vao<I>) -> BuffMem {
		Box(())
	}
}

type VaoMem = Vec<BuffMem>;
pub trait VaoArgs {
	fn seta<I: IdxType>(self, _: &mut Vao<I>) -> VaoMem;
}
impl<A: BuffArg> VaoArgs for A {
	fn seta<I: IdxType>(self, v: &mut Vao<I>) -> VaoMem {
		let a = self.set(0, v);
		vec![a]
	}
}
impl<A1: BuffArg, A2: BuffArg> VaoArgs for (A1, A2) {
	fn seta<I: IdxType>(self, v: &mut Vao<I>) -> VaoMem {
		let (a1, a2) = self;
		let (a1, a2) = (a1.set(0, v), a2.set(1, v));
		vec![a1, a2]
	}
}
impl<A1: BuffArg, A2: BuffArg, A3: BuffArg> VaoArgs for (A1, A2, A3) {
	fn seta<I: IdxType>(self, v: &mut Vao<I>) -> VaoMem {
		let (a1, a2, a3) = self;
		let (a1, a2, a3) = (a1.set(0, v), a2.set(1, v), a3.set(2, v));
		vec![a1, a2, a3]
	}
}

pub struct Geometry<I> {
	vao: Vao<I>,
	_m: Box<[BuffMem]>,
}
impl<I: IdxType> Geometry<I> {
	pub fn new(idx: &[I], args: impl VaoArgs) -> Self {
		let mut vao = Vao::default();
		let mut m = args.seta(&mut vao);

		let idx = IdxArr::new(idx);
		vao.BindIdxs(&idx);
		m.push(Box(idx));

		Self { vao, _m: m.into() }
	}
	pub fn Draw(&self, draw: impl DrawArgs) {
		self.vao.Bind().Draw(draw);
	}
}

pub trait AnyMeshT {
	fn Draw(&self);
}
impl<I: IdxType> AnyMeshT for Mesh<I>
where
	(I, GLenum): DrawArgs,
{
	fn Draw(&self) {
		self.geom.Draw(self.draw);
	}
}
