use super::*;

const SPLIT: fn(char) -> bool = char::is_whitespace;

#[derive(Default, Debug)]
struct Popup {
	size: Vec2,
	text: Str,
}
#[derive(Default, Debug)]
struct HyperKey {
	val: Option<Popup>,
	trie: HashMap<Str, Self>,
}
#[derive(Default, Debug)]
pub struct HyperDB {
	keys: HyperKey,
	max_breaks: usize,
}
impl HyperDB {
	pub fn new(f: &Font, scale: f32, pairs: impl iter::IntoIterator<Item = (impl ToString, impl ToString)>) -> Self {
		let MAX_RATIO = 10.;
		let PADDING = 0.1;

		let mut max_breaks = 0;
		let mut keys: HyperKey = Def();
		pairs.into_iter().for_each(|(k, v)| {
			let (k, v) = (k.to_string(), v.to_string());
			max_breaks = max_breaks.max(k.chars().filter(|c| SPLIT(*c)).count());
			let (w, h) = v.lines().fold((0., 0.), |(x, y), l| {
				let (w, h) = Text::size(l, f, scale);
				(w.max(x), h + y)
			});

			let s = if w < MAX_RATIO * h {
				(w, h)
			} else {
				let s = (w * h).sqrt();
				(s * 1.2, s)
			};

			let val = &mut k.split(SPLIT).fold(&mut keys, |t, k| t.trie.entry(k.to_lowercase().into()).or_default()).val;
			*val = Some(Popup { size: s.sum(PADDING * scale).fmin((2, 1.5)), text: v.into() });
		});
		Self { keys, max_breaks }
	}
	fn get<'a>(&self, keys: impl iter::Iterator<Item = &'a str>) -> Option<(usize, Vec2, &str)> {
		let mut trie = &self.keys.trie;
		for (n, k) in keys.enumerate() {
			if let Some(k) = trie.get(&*k.to_lowercase()) {
				if let Some(Popup { size, text }) = k.val.as_ref() {
					return Some((n, *size, text));
				}
				trie = &k.trie;
			} else {
				return None;
			}
		}
		None
	}
}

#[derive(Default, Debug)]
pub struct HyperText {
	size: Vec2,
	scale: f32,
	lines: Box<[STR]>,
	hovered: bool,
	scrollbar: Slider,
	popup: Option<(usize, Vec2, Box<Self>)>,
	pub text: CachedStr,
}
impl<'s: 'l, 'l> Lock::HyperText<'s, 'l, '_> {
	pub fn draw(self, pos: Vec2, size: Vec2, scale: f32, db: &HyperDB) {
		let SCR_PAD = 0.01;
		let Self { s, r, t } = self;

		let font = &t.font;
		if s.text.changed() || scale != s.scale || size != s.size {
			let (lines, _) = util::parse_text_by(&s.text, font, scale, size.x(), SPLIT);
			s.size = size;
			s.scale = scale;
			s.lines = lines;
		}
		let id = LUID(s);
		let HyperText { lines, hovered, scrollbar, popup, .. } = s;

		let whole_text_h = scale * f32(lines.len());
		let (p, vis_range) = {
			let start_at = scrollbar.pip_pos;
			let win_h = size.y();

			let len = lines.len();
			let start = (1. - start_at) * (whole_text_h - win_h);
			let vis_range = (start, win_h).mul(len).div(whole_text_h).fmax(0).sum((0, 1)).fmin(len);
			let line_pos = move |n| win_h + start - scale * f32(n + 1);
			let p = move |x, n| pos.sum((x, line_pos(n)));
			(p, vis_range)
		};
		let (start, len) = ulVec2(vis_range);

		let _c = r.clip(pos, size);

		r.draw(Rect { pos, size, color: t.bg });
		*hovered = r.hovered();

		let window = db.max_breaks + 2;

		let lines = lines
			.iter()
			.enumerate()
			.skip(start)
			.take(len)
			.filter(|(_, t)| !t.is_empty())
			.flat_map(|(n, &line)| {
				let pos = p(0., n);
				line.split_inclusive(SPLIT).scan(1, move |at, text| {
					let adv = Text::adv_at(line, font, scale, *at);
					*at += text.chars().count();
					Some((pos.sum((adv, 0)), text))
				})
			})
			.chain(vec![Def(); window])
			.collect_vec();

		let (mut skip, mut hover, mut hover_at) = (0, false, None);
		lines.windows(window).enumerate().for_each(|(id, window)| {
			let mut draw_text = |hover_at: Option<(Vec2, &str)>, keyword: bool| {
				let mut draw_text = |color| {
					let (pos, text) = window[0];
					r.draw(Text { pos, scale, color, text, font })
				};

				if !keyword {
					draw_text(t.text);
					return;
				}

				draw_text(t.highlight);
				let h = r.hovered();
				hover |= h;

				if let Some((size, text)) = hover_at {
					if !h || popup.as_ref().map(|(i, _, _)| *i == id).unwrap_or(false) || child_hovered(popup) {
						return;
					}

					let at = r.mouse_pos();
					let (side, nat) = (at.sum(size).ls(at.sub(size).abs()), at.sub(size));
					let at = (if side.x() { at.x() } else { nat.x() }, if side.y() { at.y() } else { nat.y() });
					*popup = Some((id, at, Box(HyperText { size, text: text.into(), ..Def() })));
				}
			};

			if skip > 0 {
				skip -= 1;
				draw_text(hover_at, true);
				return;
			}

			let keys = window
				.iter()
				.map(|(_, l)| l.trim_matches(|c: char| c.is_whitespace() || c.is_ascii_punctuation()))
				.filter(|l| {
					let r = l.is_empty();
					skip += usize(r);
					!r
				});

			if let Some((n, s, p)) = db.get(keys) {
				skip += n;
				hover_at = Some((s, p));
				draw_text(hover_at, true);
			} else {
				skip = 0;
				draw_text(None, false);
			}
		});

		if whole_text_h > size.y() {
			let mut pip_pos = typed_ptr!(&mut scrollbar.pip_pos);
			r.logic(
				(pos, pos.sum(size)),
				move |e, _, _| {
					let pip = pip_pos.get_mut();
					let move_pip = |v: f32| (*pip + v).clamp(0., 1.);

					match e {
						Scroll { at, state } => {
							let turbo = if state.ctrl() { 10. } else { 1. };
							*pip = move_pip(at.y() * scale / whole_text_h * turbo);
							return Accept;
						}
						_ => (),
					}
					Reject
				},
				id,
			);
			let visible_h = size.y() / whole_text_h;
			scrollbar.lock(r).draw(pos.sum((size.x() - SCR_PAD, 0.)), (SCR_PAD, size.y()), visible_h);
		}

		if !hover && !child_hovered(popup) && timeout(true) {
			timeout(false);
			*popup = None;
		}

		if let Some((_, pos, p)) = popup {
			let size = p.size;
			p.lock(r).draw(*pos, size, scale, db);
		}
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
		TIME > 30
	}
}
