use {crate::lib::*, Copy as C};

type Args1 = (*const GLvoid, i32, i32, usize);
pub trait UpdArgs1<T> {
	fn get1(&self) -> Args1;
}
impl<T> UpdArgs1<T> for Args1 {
	fn get1(&self) -> Args1 {
		*self
	}
}
impl<S: AsRef<[T]>, T, L: C, X: C> UpdArgs1<T> for (S, L, X)
where
	i32: Cast<X> + Cast<L>,
{
	fn get1(&self) -> Args1 {
		let slice = self.0.as_ref();
		let (l, x) = vec2((self.1, self.2));
		(slice.as_ptr() as *const GLvoid, l, x, slice.len())
	}
}
impl<S: AsRef<[T]>, T, X: C> UpdArgs1<T> for (S, X)
where
	i32: Cast<X>,
{
	fn get1(&self) -> Args1 {
		(&self.0, 0, self.1).get1()
	}
}
impl<T> UpdArgs1<T> for &[T] {
	fn get1(&self) -> Args1 {
		(self, 0, 0).get1()
	}
}

type Args2 = (*const GLvoid, i32, i32, i32, usize);
pub trait UpdArgs2<T> {
	fn get2(&self) -> Args2;
}
impl<T> UpdArgs2<T> for Args2 {
	fn get2(&self) -> Args2 {
		*self
	}
}
impl<S: AsRef<[T]>, T, L: C, X: C, Y: C> UpdArgs2<T> for (S, L, X, Y)
where
	i32: Cast<X> + Cast<Y> + Cast<L>,
{
	fn get2(&self) -> Args2 {
		let slice = self.0.as_ref();
		let (l, x, y) = vec3((self.1, self.2, self.3));
		(slice.as_ptr() as *const GLvoid, l, x, y, slice.len())
	}
}
impl<S: AsRef<[T]>, T, X: C, Y: C> UpdArgs2<T> for (S, X, Y)
where
	i32: Cast<X> + Cast<Y>,
{
	fn get2(&self) -> Args2 {
		(&self.0, 0, self.1, self.2).get2()
	}
}
impl<S: AsRef<[T]>, T, L: C> UpdArgs2<T> for (S, L)
where
	i32: Cast<L>,
{
	fn get2(&self) -> Args2 {
		(&self.0, self.1, 0, 0).get2()
	}
}
impl<T> UpdArgs2<T> for &[T] {
	fn get2(&self) -> Args2 {
		(self, 0).get2()
	}
}

type Args3 = (*const GLvoid, i32, i32, i32, i32, usize);
pub trait UpdArgs3<T> {
	fn get3(&self) -> Args3;
}
impl<T> UpdArgs3<T> for Args3 {
	fn get3(&self) -> Args3 {
		*self
	}
}
impl<S: AsRef<[T]>, T, L: C, X: C, Y: C, Z: C> UpdArgs3<T> for (S, L, X, Y, Z)
where
	i32: Cast<X> + Cast<Y> + Cast<Z> + Cast<L>,
{
	fn get3(&self) -> Args3 {
		let slice = self.0.as_ref();
		let (l, x, y, z) = vec4((self.1, self.2, self.3, self.4));
		(slice.as_ptr() as *const GLvoid, l, x, y, z, slice.len())
	}
}
impl<S: AsRef<[T]>, T, X: C, Y: C, Z: C> UpdArgs3<T> for (S, X, Y, Z)
where
	i32: Cast<X> + Cast<Y> + Cast<Z>,
{
	fn get3(&self) -> Args3 {
		(&self.0, 0, self.1, self.2, self.3).get3()
	}
}
impl<S: AsRef<[T]>, T, L: C> UpdArgs3<T> for (S, L)
where
	i32: Cast<L>,
{
	fn get3(&self) -> Args3 {
		(&self.0, self.1, 0, 0, 0).get3()
	}
}
impl<T> UpdArgs3<T> for &[T] {
	fn get3(&self) -> Args3 {
		(self, 0).get3()
	}
}
