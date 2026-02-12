use super::*;

const SPLIT: fn(char) -> bool = |c: char| c.is_whitespace() || c.is_ascii_punctuation() || c == 's';

#[derive(Default, Debug)]
struct Popup {
	size: Vec2,
	text: Astr,
}
#[derive(Default, Debug)]
struct HyperKey {
	val: Option<Popup>,
	trie: HashMap<Str, Self>,
}
#[derive(Default, Debug)]
pub struct HyperDB {
	keys: HyperKey,
	max_keys: usize,
}
impl HyperDB {
	pub fn new(f: &Font, pairs: impl iter::IntoIterator<Item = (impl ToString, impl ToString)>) -> Self {
		let MAX_RATIO = 10.;

		let mut max_breaks = 0;
		let mut keys: HyperKey = Def();
		pairs.into_iter().for_each(|(k, v)| {
			let (k, v) = (k.to_string(), v.to_string());
			max_breaks = max_breaks.max(k.chars().filter(|&c| SPLIT(c)).count());
			let (w, h) = v.lines().fold(Vec2(0), |(x, y), l| {
				let (w, h) = Text::size(l, f, 1.);
				(w.max(x), h + y)
			});

			let size = (w, h).or_val(w < MAX_RATIO * h, || (w * h).sqrt().pipe(|s| (2., 0.5).mul(s)));

			let val = &mut k.split(SPLIT).fold(&mut keys, |t, k| t.trie.entry(k.to_lowercase().into()).or_default()).val;
			ASSERT!(val.is_none(), "Hyperdb key collision {k:?}");
			*val = Popup { size, text: v.into() }.pipe(Some);
		});
		Self { keys, max_keys: max_breaks * 2 + 1 }
	}
	fn get<'a>(&self, scale: f32, keys: impl iter::Iterator<Item = &'a str>) -> Option<(usize, Vec2, Astr)> {
		let mut trie = &self.keys.trie;
		for (n, k) in keys
			.map(|l| l.trim_matches(SPLIT))
			.enumerate()
			.take_while(|&(n, l)| n > 0 || !l.is_empty())
			.filter(|(_, l)| !l.is_empty())
		{
			let k = trie.get(&*k.to_lowercase())?;

			if let Some(Popup { size, text }) = k.val.as_ref() {
				return (n + 1, size.mul(scale), text.clone()).pipe(Some);
			}

			trie = &k.trie;
		}
		None
	}
}

#[derive(Default, Debug)]
pub struct HyperText {
	lsb: f32,
	size: Vec2,
	scale: f32,
	lines: Box<[STR]>,
	scrollbar: Slider,
	hovered: bool,
	last_pip: f32,
	batched: Box<[BatchedWords]>,
	popup: Option<(usize, Vec2, Box<Self>)>,
	pub text: CachedStr,
}
impl HyperText {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, layout @ Surf { pos, size }: Surf, scale: f32, db: &HyperDB) {
		let (SCR_PAD, POP_PAD) = (0.01, Vec2(0.2 * scale));
		let (id, s, font) = (ref_UUID(self), self, &t.font);

		if s.text.changed() || scale != s.scale || size != s.size {
			let (lsb, lines, _) = u::parse_text(&s.text, font, scale, size.x(), char::is_whitespace).pipe(task::block_on);
			(s.lsb, s.size, s.scale, s.last_pip) = (lsb, size, scale, f32::NAN);
			s.lines = unsafe { mem::transmute(lines.into_boxed_slice()) };
		}
		let Self {
			lsb,
			ref lines,
			ref mut scrollbar,
			ref mut hovered,
			ref mut last_pip,
			ref mut batched,
			ref mut popup,
			..
		} = *s;

		let (scrollable, p, (start, len)) = {
			let (start, len) = (1. - scrollbar.pip_pos, lines.len());
			let (_, l) = u::visible_range(layout, scale, 0., len);
			let skip = f32(len - l) * start;
			let p = move |x, n| u::line_pos(lines, font, scale, layout, skip, n, x + lsb);
			(len > l, p, u::visible_range(layout, scale, skip, len))
		};
		let (pip_size, adv) = u::visible_norm(layout, len, lines.len());

		if &scrollbar.pip_pos != last_pip {
			(*last_pip, *popup) = (scrollbar.pip_pos, None);
			let window = db.max_keys;
			let Continue = || Some(None);

			let words = lines
				.iter()
				.enumerate()
				.skip(start)
				.take(len + 1)
				.filter(|(_, l)| !l.is_empty())
				.flat_map(|(n, line)| {
					line.split_inclusive(SPLIT)
						.flat_map(|l| {
							l.rfind(SPLIT)
								.and_then(|i| vec![&l[..i], &l[i..]].pipe(Some).or_def(i > 0))
								.unwrap_or_else(|| vec![l])
						})
						.map(move |l| (n, l))
				})
				.chain(vec![(usize::MAX, ""); window].or_def(!lines.is_empty()))
				.collect_vec();

			*batched = words
				.windows(window)
				.scan((0, None, 0, 0, 0), move |(skip, tip, beg, end, prev_l), window| {
					let &(lnum, word) = window.at(0);
					let word_end = *end + word.len();
					let next_line = (0, word.len(), lnum);
					let next_word = (*end, word_end, lnum);
					let next_batch = || next_word.or_val(*prev_l == lnum, || next_line);

					let line = {
						let (beg, end, lnum) = (*beg, *end, *prev_l);
						move |tip| {
							let line = lines.at(lnum);
							let adv = line[..beg].utf8_count();
							let adv = Text::adv_at(line, font, scale, adv);
							let pos = p(adv, lnum);
							let line = unsafe { mem::transmute(&line[beg..end]) };
							Some((tip, pos, line))
						}
					};
					let normal = || line(None);

					if *skip > 0 {
						*skip -= 1;
						if *skip > 0 && *prev_l == lnum {
							*end = word_end;
							return Continue();
						}

						(*beg, *end, *prev_l) = next_batch();
						return line(tip.clone()).pipe(Some);
					}

					let keys = window.iter().map(|&(_, l)| l);

					let Some((n, s, t)) = db.get(scale, keys) else {
						if *prev_l == lnum {
							*end = word_end;
							return Continue();
						}

						(*beg, *end, *prev_l) = next_batch();
						return Some(normal());
					};

					(*skip, *tip) = (n, Some((s, t)));

					let normal = None.or_val(beg == end, normal);
					(*beg, *end, *prev_l) = next_batch();
					Some(normal)
				})
				.flatten()
				.collect();
		}

		r.draw(Rect {
			pos,
			size: layout.w_sub(f32(scrollable) * SCR_PAD).size,
			color: t.bg,
		});
		*hovered = r.hovered();

		let _c = r.clip(layout);

		let mut hover = false;
		batched.iter().enumerate().for_each(|(id, (tip, p, text))| {
			let pos = pos.sum(p);
			let mut draw_text = |color| r.draw(Text { pos, scale, color, text, font });

			let &Some((size, ref tip)) = tip else { return draw_text(t.text) };

			draw_text(t.highlight);
			let h = r.hovered();
			hover |= h;

			if !h || popup.as_ref().map(|(i, ..)| *i == id).unwrap_or(false) || child_hovered(popup) {
				return;
			}

			let at = r.mouse_pos();
			let side = at.sum(size).ls(at.sub(size).abs());
			let scale = scale * 1.05;
			let at = at.sum((0., scale - (at.y() - pos.y()).abs()));
			let nat = at.sub(size).sub((0., scale));
			let at = at.mul(side).sum(nat.mul(side.map(|s| !s)));
			*popup = (id, at, Self { size, text: (**tip).into(), ..Def() }.into()).pipe(Some);
		});

		let mut draw_scrollbar = |sc: &'s mut Slider| {
			if !scrollable {
				return sc.pip_pos = 1.;
			}

			let s = layout.xr(SCR_PAD).w(SCR_PAD);

			let sc = Cell::from_mut(sc);
			r.logic(
				layout,
				move |e, _, _| {
					let move_pip = |o: f32| sc.mutate(|s| s.pip_pos = (s.pip_pos + o * adv * f32(len)).clamp(0., 1.));
					match *e {
						Scroll { at, .. } => move_pip(at.y()),
						Keyboard { key, m } if m.pressed() => match key {
							Key::Up | Key::PageUp => move_pip(1.),
							Key::Down | Key::PageDown => move_pip(-1.),
							_ => return Pass,
						},
						_ => return Pass,
					}
					Accept
				},
				id,
			);

			sc.mutate(|sc| sc.draw(r, t, s, pip_size));
		};

		draw_scrollbar(scrollbar);

		if !hover && !child_hovered(popup) && timeout(true) {
			timeout(false);
			*popup = None;
		}

		if let Some((_, pos, p)) = popup {
			let s = Surf::new(*pos, p.size);
			r.unclipped(|r| {
				if p.popup.is_none() {
					let Surf { pos, size } = s.size_sub(POP_PAD.mul(-2));
					r.draw(Rect { pos, size, color: (0., 0., 0., 1.) })
				}
				p.draw(r, t, s.xy(POP_PAD), scale, db)
			});
		}
	}
}

impl<'s: 'l, 'l> Lock::HyperText<'s, 'l, '_> {
	pub fn draw(self, g: impl Into<Surf>, sc: f32, d: &HyperDB) {
		let Self { s, r, t } = self;
		s.draw(r, t, g.into(), sc, d)
	}
}

fn child_hovered(p: &Option<(usize, Vec2, Box<HyperText>)>) -> bool {
	p.as_ref().map(|(_, _, p)| p.hovered || child_hovered(&p.popup)).unwrap_or(false)
}

fn timeout(active: bool) -> bool {
	unsafe {
		static mut TIME: usize = 0;
		TIME += 1;
		if !active {
			TIME = 0
		}
		TIME > 60
	}
}

type BatchedWords = (Option<(Vec2, Astr)>, Vec2, &'static str);
