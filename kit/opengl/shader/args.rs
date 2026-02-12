use super::*;

type Init<'s, W> = (&'s mut W, Load);
pub trait InitArgs<'s, W> {
	fn get(self) -> Init<'s, W>;
}
impl<'s, W: Window> InitArgs<'s, W> for &'s mut W {
	fn get(self) -> Init<'s, W> {
		(self, vec![])
	}
}
impl<'s, I: LoadArgs, W: Window> InitArgs<'s, W> for (&'s mut W, I) {
	fn get(self) -> Init<'s, W> {
		let (w, i) = self;
		(w, i.get())
	}
}

type Load = Vec<Astr>;
pub trait LoadArgs {
	fn get(self) -> Load;
}
impl LoadArgs for &str {
	fn get(self) -> Load {
		vec![Arc::from(self)]
	}
}
impl<T: Into<Astr>, const N: usize> LoadArgs for [T; N] {
	fn get(self) -> Load {
		self.into_iter().map(|a| a.into()).collect()
	}
}

type Compile = Box<[Str]>;
pub trait CompileArgs {
	fn get(self) -> Compile;
}
impl<A1: Into<Str>, A2: Into<Str>> CompileArgs for (A1, A2) {
	fn get(self) -> Compile {
		let (a1, a2) = self;
		[a1.into(), a2.into()].into()
	}
}
impl<A1: Into<Str>, A2: Into<Str>, A3: Into<Str>> CompileArgs for (A1, A2, A3) {
	fn get(self) -> Compile {
		let (a1, a2, a3) = self;
		[a1.into(), a2.into(), a3.into()].into()
	}
}
impl<A1: Into<Str>, A2: Into<Str>, A3: Into<Str>, A4: Into<Str>> CompileArgs for (A1, A2, A3, A4) {
	fn get(self) -> Compile {
		let (a1, a2, a3, a4) = self;
		[a1.into(), a2.into(), a3.into(), a4.into()].into()
	}
}
impl<A1: Into<Str>, A2: Into<Str>, A3: Into<Str>, A4: Into<Str>, A5: Into<Str>> CompileArgs for (A1, A2, A3, A4, A5) {
	fn get(self) -> Compile {
		let (a1, a2, a3, a4, a5) = self;
		[a1.into(), a2.into(), a3.into(), a4.into(), a5.into()].into()
	}
}
impl<const N: usize> CompileArgs for [InlineShader; N] {
	fn get(self) -> Compile {
		self.into_iter().map(|a| a.into()).collect_box()
	}
}
