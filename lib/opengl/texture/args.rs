use crate::uses::{slicing::*, *};

pub trait MipsArgs {
	fn getm(self) -> i32;
}
impl<W, H, D> MipsArgs for (W, H, D)
where
	i32: Cast<W> + Cast<H> + Cast<D>,
{
	fn getm(self) -> i32 {
		let (w, h, d) = iVec3::to(self);
		w.max(h).max(d)
	}
}
impl<W, H> MipsArgs for (W, H)
where
	i32: Cast<W> + Cast<H>,
{
	fn getm(self) -> i32 {
		(self.0, self.1, 1).getm()
	}
}
impl<W> MipsArgs for W
where
	i32: Cast<W>,
{
	fn getm(self) -> i32 {
		(self, 1).getm()
	}
}

pub trait NewArgs1 {
	fn get1(self) -> iVec2;
}
impl<L, W> NewArgs1 for (L, W)
where
	i32: Cast<L> + Cast<W>,
{
	fn get1(self) -> iVec2 {
		iVec2::to(self)
	}
}
impl<W> NewArgs1 for W
where
	i32: Cast<W>,
{
	fn get1(self) -> iVec2 {
		(1, self).get1()
	}
}

pub trait NewArgs2 {
	fn get2(self) -> iVec3;
}
impl<L, W, H> NewArgs2 for (L, W, H)
where
	i32: Cast<L> + Cast<W> + Cast<H>,
{
	fn get2(self) -> iVec3 {
		iVec3::to(self)
	}
}
impl<W, H> NewArgs2 for (W, H)
where
	i32: Cast<W> + Cast<H>,
{
	fn get2(self) -> iVec3 {
		(1, self.0, self.1).get2()
	}
}

pub trait NewArgs3 {
	fn get3(self) -> iVec4;
}
impl<L, W, H, D> NewArgs3 for (L, W, H, D)
where
	i32: Cast<L> + Cast<W> + Cast<H> + Cast<D>,
{
	fn get3(self) -> iVec4 {
		iVec4::to(self)
	}
}
impl<W, H, D> NewArgs3 for (W, H, D)
where
	i32: Cast<W> + Cast<H> + Cast<D>,
{
	fn get3(self) -> iVec4 {
		(1, self.0, self.1, self.2).get3()
	}
}

type UArgs1 = (*const GLvoid, i32, i32, usize);
pub trait UpdArgs1<T> {
	fn geta1(self) -> UArgs1;
}
impl<T> UpdArgs1<T> for UArgs1 {
	fn geta1(self) -> Self {
		self
	}
}
impl<S: Sliceable<T>, T, L, X> UpdArgs1<T> for (S, L, X)
where
	i32: Cast<X> + Cast<L>,
{
	fn geta1(self) -> UArgs1 {
		let slice = self.0.slice();
		let (l, x) = iVec2::to((self.1, self.2));
		(slice.as_ptr() as *const GLvoid, l, x, slice.len())
	}
}
impl<S: Sliceable<T>, T, X> UpdArgs1<T> for (S, X)
where
	i32: Cast<X>,
{
	fn geta1(self) -> UArgs1 {
		(self.0, 0, self.1).geta1()
	}
}
impl<T> UpdArgs1<T> for &[T] {
	fn geta1(self) -> UArgs1 {
		(self, 0, 0).geta1()
	}
}
impl<T> UpdArgs1<T> for &Vec<T> {
	fn geta1(self) -> UArgs1 {
		(&self[..]).geta1()
	}
}

type UArgs2 = (*const GLvoid, i32, i32, i32, usize);
pub trait UpdArgs2<T> {
	fn geta2(self) -> UArgs2;
}
impl<T> UpdArgs2<T> for UArgs2 {
	fn geta2(self) -> Self {
		self
	}
}
impl<S: Sliceable<T>, T, L, X, Y> UpdArgs2<T> for (S, L, X, Y)
where
	i32: Cast<X> + Cast<Y> + Cast<L>,
{
	fn geta2(self) -> UArgs2 {
		let slice = self.0.slice();
		let (l, x, y) = iVec3::to((self.1, self.2, self.3));
		(slice.as_ptr() as *const GLvoid, l, x, y, slice.len())
	}
}
impl<S: Sliceable<T>, T, X, Y> UpdArgs2<T> for (S, X, Y)
where
	i32: Cast<X> + Cast<Y>,
{
	fn geta2(self) -> UArgs2 {
		(self.0, 0, self.1, self.2).geta2()
	}
}
impl<S: Sliceable<T>, T, L> UpdArgs2<T> for (S, L)
where
	i32: Cast<L>,
{
	fn geta2(self) -> UArgs2 {
		(self.0, self.1, 0, 0).geta2()
	}
}
impl<T> UpdArgs2<T> for &[T] {
	fn geta2(self) -> UArgs2 {
		(self, 0).geta2()
	}
}
impl<T> UpdArgs2<T> for &Vec<T> {
	fn geta2(self) -> UArgs2 {
		(&self[..]).geta2()
	}
}

type UArgs3 = (*const GLvoid, i32, i32, i32, i32, usize);
pub trait UpdArgs3<T> {
	fn geta3(self) -> UArgs3;
}
impl<T> UpdArgs3<T> for UArgs3 {
	fn geta3(self) -> Self {
		self
	}
}
impl<S: Sliceable<T>, T, L, X, Y, Z> UpdArgs3<T> for (S, L, X, Y, Z)
where
	i32: Cast<X> + Cast<Y> + Cast<Z> + Cast<L>,
{
	fn geta3(self) -> UArgs3 {
		let slice = self.0.slice();
		let (l, x, y, z) = iVec4::to((self.1, self.2, self.3, self.4));
		(slice.as_ptr() as *const GLvoid, l, x, y, z, slice.len())
	}
}
impl<S: Sliceable<T>, T, X, Y, Z> UpdArgs3<T> for (S, X, Y, Z)
where
	i32: Cast<X> + Cast<Y> + Cast<Z>,
{
	fn geta3(self) -> UArgs3 {
		(self.0, 0, self.1, self.2, self.3).geta3()
	}
}
impl<S: Sliceable<T>, T, L> UpdArgs3<T> for (S, L)
where
	i32: Cast<L>,
{
	fn geta3(self) -> UArgs3 {
		(self.0, self.1, 0, 0, 0).geta3()
	}
}
impl<T> UpdArgs3<T> for &[T] {
	fn geta3(self) -> UArgs3 {
		(self, 0).geta3()
	}
}
impl<T> UpdArgs3<T> for &Vec<T> {
	fn geta3(self) -> UArgs3 {
		(&self[..]).geta3()
	}
}
