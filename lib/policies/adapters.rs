use crate::uses::*;

type WArgs = (i32, i32, u32, u32);
pub trait WINSize {
	fn get(self) -> WArgs;
}
impl<A, B, C, D> WINSize for (A, B, C, D)
where
	i32: Cast<A> + Cast<B>,
	u32: Cast<C> + Cast<D>,
{
	fn get(self) -> WArgs {
		<_>::to(self)
	}
}
impl<A, B> WINSize for (A, B)
where
	u32: Cast<A> + Cast<B>,
{
	fn get(self) -> WArgs {
		(0, 0, u32(self.0), u32(self.1))
	}
}
