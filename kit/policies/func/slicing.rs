pub fn split(s: &str, f: impl Fn(char) -> bool) -> (&str, &str) {
	let at = find(s, f);
	(&s[..at], &s[at..])
}

pub fn slice<'a>(args: impl SliceArgs<'a>) -> &'a str {
	let (beg, s, end) = args.get();
	&s[beg.min(s.len())..end.min(s.len())]
}

type Args<'a> = (usize, &'a str, usize);
pub trait SliceArgs<'a> {
	fn get(self) -> Args<'a>; // TODO replace with Pattern
}
impl<'a> SliceArgs<'a> for Args<'a> {
	fn get(self) -> Self {
		self
	}
}

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

impl<'a, F: Fn(char) -> bool> SliceArgs<'a> for (F, &'a String) {
	fn get(self) -> Args<'a> {
		(self.0, self.1 as &'a str).get()
	}
}
impl<'a, F: Fn(char) -> bool> SliceArgs<'a> for (&'a String, F) {
	fn get(self) -> Args<'a> {
		(self.0 as &'a str, self.1).get()
	}
}
impl<'a, F: Fn(char) -> bool> SliceArgs<'a> for (F, &'a String, F) {
	fn get(self) -> Args<'a> {
		(self.0, self.1 as &'a str, self.2).get()
	}
}

fn find(s: &str, f: impl Fn(char) -> bool) -> usize {
	s.find(f).unwrap_or(s.len())
}
