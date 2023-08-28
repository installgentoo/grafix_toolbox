pub type CompileArgs<'a> = (C<'a>, Option<C<'a>>, C<'a>);
pub trait ShaderArgs<'a> {
	fn get(self) -> CompileArgs<'a>;
}
impl<'a, V: Into<C<'a>>, P: Into<C<'a>>> ShaderArgs<'a> for (V, P) {
	fn get(self) -> CompileArgs<'a> {
		let (v, p) = self;
		(v.into(), None, p.into())
	}
}
impl<'a, V: Into<C<'a>>, G: Into<C<'a>>, P: Into<C<'a>>> ShaderArgs<'a> for (V, G, P) {
	fn get(self) -> CompileArgs<'a> {
		let (v, g, p) = self;
		(v.into(), Some(g.into()), p.into())
	}
}

pub trait PureShaderArgs<'a>: ShaderArgs<'a> {}
impl_trait_for!(PureShaderArgs<'_> = (I, I), (I, I, I));

type I = super::InlineShader;
pub type C<'a> = std::borrow::Cow<'a, str>;
