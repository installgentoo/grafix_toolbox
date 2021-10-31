use crate::uses::*;

pub type CompileArgs = (CowStr, Option<CowStr>, CowStr);
pub trait ShdTypeArgs {
	fn get(self) -> CompileArgs;
}
impl<V: Into<CowStr>, P: Into<CowStr>> ShdTypeArgs for (V, P) {
	fn get(self) -> CompileArgs {
		(self.0.into(), None, self.1.into())
	}
}
impl<V: Into<CowStr>, G: Into<CowStr>, P: Into<CowStr>> ShdTypeArgs for (V, G, P) {
	fn get(self) -> CompileArgs {
		(self.0.into(), Some(self.1.into()), self.2.into())
	}
}

pub trait PureShdTypeArgs: ShdTypeArgs {}
impl_trait_for!(PureShdTypeArgs = (I, I), (I, I, I));
type I = super::InlineShader;
