use super::*;

pub fn if_ctrl<T>(m: Mod, t: T, f: T) -> T {
	if m.ctrl() { t } else { f }
}

pub async fn parse_text<'s>(text: &'s str, f: &Font, scale: f32, max_w: f32, split_by: impl Fn(char) -> bool) -> (f32, Vec<&'s str>, Vec<u32>) {
	let cap = text.len().or_map(|_| max_w / scale <= 1., |l| l / usize(max_w / scale));
	let (mut lnum, mut lsb, mut lines, mut wraps) = (1, 0., Vec::with_capacity(cap), Vec::with_capacity(cap));

	for mut l in text.lines() {
		if l.is_empty() {
			lines.push("");
			wraps.push(lnum);
			lnum += 1;
		}
		while !l.is_empty() {
			task::yield_now().await;
			let (head, tail) = {
				let (_, (head, tail)) = Text::substr(l, f, scale, max_w);
				match tail {
					"" => (head, tail),
					_ if head.is_empty() => {
						let second_char = l.utf8_len_at(1);
						l.split_at(second_char)
					}
					_ if tail.starts_with(&split_by) => (head, tail),
					_ => {
						/* Traditional   ^head.ends  V.map(|h| l[h..].char_indices().map(|(i, _)| h + i).nth(1)).flatten() */
						let h = head.rfind(&split_by).unwrap_or(head.len());
						if h > 0 { l.split_at(h) } else { (head, tail) }
					}
				}
			};
			let e = tail.is_empty();
			lsb = Text::lsb(head, f, scale).max(lsb);
			lines.push(head);
			wraps.push(lnum.or_def(e));
			lnum += u32(e);
			l = tail;
		}
	}
	(lsb, lines, wraps)
}

pub fn fit_line(line: &str, f: &Font, scale: f32, size: Vec2) -> (Vec2, f32) {
	let line_size = Text::size(line, f, scale).sum((Text::lsb(line, f, scale), 0.));
	ASSERT!(size.min_comp() >= 0., "Passed negative size {size:?} in gui");
	let r = size.div(line_size);
	let r = r.x().min(r.y() * 2.).min(1.);
	(size.sub(line_size.mul(r)).mul(0.5), scale * r)
}

pub fn visible_range(Surf { size: (_, h), .. }: Surf, scale: f32, skip: f32, len: usize) -> ulVec2 {
	if len < 1 {
		return (0, 0);
	}

	let start = usize(skip).min(len);
	let len = usize(skip.rem_euclid(scale) + h / scale).min(len - start).max(1);
	(start, len)
}

pub fn visible_norm(Surf { size: (_, h), .. }: Surf, vis_lines: usize, len: usize) -> Vec2 {
	let (v, l) = Vec2((vis_lines, len.max(1)));
	(h * v / l, 1. / (l - v).max(1.))
}

pub fn line_pos(lines: &impl TextLineAt, f: &Font, scale: f32, Surf { size: (_, h), .. }: Surf, skip: f32, lnum: usize, lsb: f32) -> Vec2 {
	let (line, _) = lines.get(isize(lnum));
	let l = Text::lsb(line, f, scale);
	let lnum = f32(isize(lnum) - isize(skip)) + 1. - skip.fract();
	(lsb - l, h - lnum * scale)
}

pub type Caret = ilVec2;
pub mod caret {
	pub async fn serialise(lines: impl Iterator<Item = &DynamicStr>, c: Caret) -> isize {
		let range = u::collect_range(lines, (0, 0), c).await;
		let x = range.chars().count();
		isize(x)
	}

	pub fn set(l: &impl TextLineAt, c: Caret, o: Caret) -> Caret {
		set_async(l, c, o).pipe(task::block_on)
	}
	pub async fn set_async(lines: &impl TextLineAt, (x, y): Caret, (ox, oy): Caret) -> Caret {
		let last_pos = |y| {
			let (line, nl) = lines.get(y);
			(chars(line) - isize(!nl)).max(0)
		};

		let max_y = lines.max_y();
		let mut y = (oy + y).clamp(0, max_y);
		let mut x = ox + x.clamp(0, last_pos(y));
		loop {
			task::yield_now().await;
			if x < 0 {
				y -= 1;
				if y < 0 {
					return (0, 0);
				}

				x = last_pos(y) + x + 1;
				continue;
			}

			let max_x = last_pos(y);
			if x <= max_x {
				return (x, y);
			}

			y += 1;
			if y > max_y {
				return (max_x, max_y);
			}

			x = x - max_x - 1;
		}
	}

	pub fn idx(lines: &impl TextLineAt, at: Caret, o: Caret) -> usize {
		let (x, y) = set(lines, at, o);
		let (line, _) = lines.get(y);
		line.utf8_len_at(x)
	}

	pub fn at_pos(lines: &impl TextLineAt, f: &Font, scale: f32, start: usize, p: Vec2) -> Caret {
		let (x, y) = p.mul((1, -1));
		let y = isize(start) + isize(y / scale + 1.);
		let (line, _) = lines.get(y);
		let x = Text::substr(line, f, scale, x).pipe(|(_, (line, _))| chars(line));
		set(lines, (x, y), (0, 0))
	}

	pub fn adv(lines: &impl TextLineAt, f: &Font, scale: f32, (x, y): Caret, pad: f32) -> f32 {
		let (line, nl) = lines.get(y);
		let past_end = x >= chars(line) && nl;
		let x = usize(if past_end { x + 1 } else { x });
		let adv = Text::adv_at(line, f, scale, x);
		adv - pad.or_def(past_end && !line.is_empty())
	}

	pub fn sort(caret: Caret, select: Caret) -> (Caret, Caret) {
		let ord = (caret.y() > select.y()).or_val(caret.y() != select.y(), || caret.x() > select.x());
		let (beg, end) = (select, caret).or_val(ord, || (caret, select));
		(beg, end)
	}

	use super::*;
}

pub trait TextLineAt {
	fn get(&self, y: isize) -> (&str, bool);
	fn max_y(&self) -> isize {
		0
	}
}
impl TextLineAt for CachedStr {
	fn get(&self, _: isize) -> (&str, bool) {
		(self, true)
	}
}
impl TextLineAt for Box<[&str]> {
	fn get(&self, y: isize) -> (&str, bool) {
		if self.is_empty() {
			return Def();
		}

		let y = y.clamp(0, self.max_y());
		(self.at(y), false)
	}
	fn max_y(&self) -> isize {
		isize(self.last_idx())
	}
}
impl TextLineAt for Vec<DynamicStr> {
	fn get(&self, y: isize) -> (&str, bool) {
		if self.is_empty() {
			return Def();
		}

		let y = y.clamp(0, self.max_y());
		let l = self.at(y);
		match l {
			R(l) => (l, false),
			RN(l, _) => (l, true),
			P(l) => (l, true),
		}
	}
	fn max_y(&self) -> isize {
		isize(self.last_idx())
	}
}
impl TextLineAt for VerVec<DynamicStr> {
	fn get(&self, y: isize) -> (&str, bool) {
		if self.is_empty() {
			return Def();
		}

		let y = y.clamp(0, self.max_y());
		let l = self.at(y);
		match l {
			R(l) => (l, false),
			RN(l, _) => (l, true),
			P(l) => (l, true),
		}
	}
	fn max_y(&self) -> isize {
		isize(self.last_idx())
	}
}

pub async fn collect_range(lines: impl Iterator<Item = &DynamicStr>, c1: Caret, c2: Caret) -> String {
	let mut lines = lines.peekable();
	if lines.peek().is_none() || c1 == c2 {
		return Def();
	}

	let ((b_x, b_y), (e_x, e_y)) = mat2(caret::sort(c1, c2));
	let (dist, mut text) = (e_y - b_y, String::new());

	let partial = |text: &mut String, l: &DynamicStr, b, e| {
		let mut line: String = Def();
		l.write_self(&mut line);
		text.push_str(line.utf8_slice(b..e));
	};

	let mut lines = lines.skip(b_y);

	if dist == 0 {
		partial(&mut text, lines.next().fail(), b_x, e_x);
		return text;
	}

	partial(&mut text, lines.next().fail(), b_x, usize::MAX);

	for _ in 0..dist - 1 {
		async {
			lines.next().fail().write_self(&mut text);
		}
		.await
	}

	partial(&mut text, lines.next().fail(), 0, e_x);

	text
}

pub fn replace_range(lines: &mut VerVec<DynamicStr>, new: &str, c1: Caret, c2: Caret) {
	if lines.is_empty() {
		return *lines = vec![P(new.into())].into();
	}

	let ((b_x, b_y), (e_x, e_y)) = caret::sort(c1, c2);

	let (mut beg, mut end) = Def();
	lines.at(b_y).write_self(&mut beg);
	lines.at(e_y).write_self(&mut end);
	let new = [beg.utf8_slice(..b_x), new, end.utf8_slice(e_x..)].concat();
	*lines = lines.replace(b_y, [P(new.into())]);
	if usize(b_y) == lines.last_idx() {
		return;
	}

	let (b_y, e_y) = (b_y, e_y).sum(1).fmin((lines.last_idx(), lines.len()));
	*lines = lines.remove(b_y..e_y);
}

#[derive(Debug)]
pub enum DynamicStr {
	R(Astr),
	RN(Astr, u32),
	P(Astr),
}
impl DynamicStr {
	pub fn new((l, n): (&str, u32)) -> Self {
		let l = l.into();
		if n == 0 { R(l) } else { RN(l, n) }
	}
	pub fn as_clipped_str(&self, f: &Font, scale: f32, max_w: f32) -> &str {
		match &self {
			R(l) | RN(l, _) => l,
			P(l) => Text::substr(l, f, scale, max_w + scale).pipe(|(_, (head, _))| head),
		}
	}
	pub fn write_self(&self, to: &mut String) {
		match self {
			R(l) => to.push_str(l),
			RN(l, _) => {
				to.push_str(l);
				to.push('\n');
			}
			P(l) => to.push_str(l),
		}
	}
	pub fn lnum(&self) -> Option<String> {
		match self {
			R(_) => None,
			RN(_, n) => n.to_string().pipe(Some),
			P(_) => {
				const S: [&str; 4] = ["|", "/", "-", "\\"];
				let s = LocalStatic!(usize);
				*s = (*s + 1) % 40;
				S[*s / 10].to_string().pipe(Some)
			}
		}
	}
}

fn chars(l: &str) -> isize {
	isize(l.utf8_count())
}
use DynamicStr::*;
