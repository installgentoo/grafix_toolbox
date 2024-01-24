use crate::lib::*;

type Args = (i32, [f32; 4]);
pub trait ClearArgs {
	fn get(self) -> Args;
}
impl<R, G, B, A> ClearArgs for (u32, (R, G, B, A))
where
	Vec4: Cast<(R, G, B, A)>,
{
	fn get(self) -> Args {
		let (r, g, b, a) = Vec4(self.1);
		(i32(self.0), [r, g, b, a])
	}
}
impl<R, G, B, A> ClearArgs for (R, G, B, A)
where
	Vec4: Cast<(R, G, B, A)>,
{
	fn get(self) -> Args {
		(0, self).get()
	}
}
impl<C: Copy> ClearArgs for (u32, C)
where
	f32: Cast<C>,
{
	fn get(self) -> Args {
		let v = self.1;
		(self.0, (v, v, v, v)).get()
	}
}
impl<C: Copy> ClearArgs for C
where
	f32: Cast<C>,
{
	fn get(self) -> Args {
		(0, self).get()
	}
}

type CArgs = ((f32, f32, f32, f32), f32);
pub trait ColorDepthArg {
	fn getc(self) -> CArgs;
}
impl<R, G, B, A, D> ColorDepthArg for ((R, G, B, A), D)
where
	f32: Cast<D>,
	Vec4: Cast<(R, G, B, A)>,
{
	fn getc(self) -> CArgs {
		(Vec4(self.0), f32(self.1))
	}
}
impl<R, G, B, A> ColorDepthArg for (R, G, B, A)
where
	Vec4: Cast<(R, G, B, A)>,
{
	fn getc(self) -> CArgs {
		(self, 1.).getc()
	}
}
impl<C: Copy, D> ColorDepthArg for (C, D)
where
	f32: Cast<C> + Cast<D>,
{
	fn getc(self) -> CArgs {
		let v = self.0;
		((v, v, v, v), self.1).getc()
	}
}
impl<C: Copy> ColorDepthArg for C
where
	f32: Cast<C>,
{
	fn getc(self) -> CArgs {
		(self, 1.).getc()
	}
}
