pub trait Sliceable<T> {
	fn slice(&self) -> &[T];
}
impl<T> Sliceable<T> for &[T] {
	fn slice(&self) -> &[T] {
		self
	}
}
impl<T> Sliceable<T> for &Vec<T> {
	fn slice(&self) -> &[T] {
		self.as_slice()
	}
}

pub fn split<F: Fn(char) -> bool>(s: &str, f: F) -> (&str, &str) {
	let at = find(s, f);
	(&s[..at], &s[at..])
}

pub fn slice<'a>(args: impl SliceArgs<'a>) -> &'a str {
	let (beg, s, end) = args.get();
	&s[beg.min(s.len())..end.min(s.len())]
}

type Args<'a> = (usize, &'a str, usize);
pub trait SliceArgs<'a> {
	fn get(self) -> Args<'a>;
}
impl<'a> SliceArgs<'a> for Args<'a> {
	fn get(self) -> Self {
		self
	}
}
impl<'a> SliceArgs<'a> for (usize, &'a str) {
	fn get(self) -> Args<'a> {
		(self.0, self.1, self.1.len())
	}
}
impl<'a> SliceArgs<'a> for (&'a str, usize) {
	fn get(self) -> Args<'a> {
		(0, self.0, self.1)
	}
}

///TODO replace with Pattern
impl<'a, F: Fn(char) -> bool> SliceArgs<'a> for (F, &'a str) {
	fn get(self) -> Args<'a> {
		let (f, s) = self;
		let at = find(s, f);
		(at, s, s.len())
	}
}
impl<'a, F: Fn(char) -> bool> SliceArgs<'a> for (&'a str, F) {
	fn get(self) -> Args<'a> {
		let (s, f) = self;
		let at = find(s, f);
		(0, s, at)
	}
}
impl<'a, F: Fn(char) -> bool> SliceArgs<'a> for (F, &'a str, F) {
	fn get(self) -> Args<'a> {
		let (f1, s, f2) = self;
		let beg = find(s, f1);
		let end = find(s, f2);
		(beg, s, end)
	}
}

fn find<F: Fn(char) -> bool>(s: &str, f: F) -> usize {
	match s.find(f) {
		Some(at) => at,
		None => s.len(),
	}
}
