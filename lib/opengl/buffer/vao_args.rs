use crate::uses::*;

type Args = (u32, u32, bool, u32, u32);
pub trait AttrFmtArgs {
	fn get(self) -> Args;
}
impl<P, O> AttrFmtArgs for (u32, u32, bool, P, O)
where
	u32: Cast<P> + Cast<O>,
{
	fn get(self) -> Args {
		(self.0, self.1, self.2, u32::to(self.3), u32::to(self.4))
	}
}

impl AttrFmtArgs for (u32, u32, bool) {
	fn get(self) -> Args {
		(self.0, self.1, self.2, 0, 0)
	}
}
impl AttrFmtArgs for (u32, u32) {
	fn get(self) -> Args {
		(self.0, self.1, false).get()
	}
}

type DArgs = (i32, usize, GLenum);
pub trait DrawArgs {
	fn get(self) -> DArgs;
}
impl<N, O> DrawArgs for (N, O, GLenum)
where
	i32: Cast<N>,
	usize: Cast<O>,
{
	fn get(self) -> DArgs {
		(i32::to(self.0), usize::to(self.1), self.2)
	}
}
impl<N> DrawArgs for (N, GLenum)
where
	i32: Cast<N>,
{
	fn get(self) -> DArgs {
		(self.0, 0, self.1).get()
	}
}
impl<N> DrawArgs for N
where
	i32: Cast<N>,
{
	fn get(self) -> DArgs {
		(self, gl::TRIANGLES).get()
	}
}

pub fn to_glbool(b: bool) -> GLbool {
	if b {
		gl::TRUE
	} else {
		gl::FALSE
	}
}
