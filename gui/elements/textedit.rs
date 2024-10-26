use super::*;

#[derive(Default, Debug)]
pub struct TextEdit {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	lines: Box<[STR]>,
	wraps: Box<[u32]>,
	select: Caret,
	caret: Caret,
	changes: Option<(Vec<STR>, Vec<Str>)>,
	history: History,
	scrollbar: Slider,
	pub text: CachedStr,
}
impl TextEdit {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, layout @ (pos, size): Geom, scale: f32, readonly: bool) {
		let SCR_PAD = 0.02;
		let CUR_PAD = 0.01;
		let s = self;

		let font = &t.font;
		if s.text.changed() || scale != s.scale || size != s.size {
			let linenum_bar_w = |l| (font.glyph('0').map(|g| g.adv).unwrap_or_default() * scale * (f32(l).max(1.).log10() + 1.)).min(size.x());
			let offset = (linenum_bar_w(s.text.lines().count()), 0.);
			let (lines, wraps) = util::parse_text_by(&s.text, font, scale, size.sub(offset).x() - SCR_PAD, char::is_whitespace);
			s.select = util::move_caret(&lines, s.select, (0, 0), true);
			s.caret = util::move_caret(&lines, s.caret, (0, 0), true);

			s.offset = offset;
			s.size = size;
			s.scale = scale;
			s.lines = lines;
			s.wraps = wraps;
		}
		let id = LUID(s);
		let TextEdit {
			ref offset,
			ref lines,
			ref wraps,
			select,
			caret,
			changes,
			history,
			scrollbar,
			text,
			..
		} = s;
		*changes = None;

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

		let _c = r.clip(layout);

		r.draw(Rect {
			pos: offset.sum(pos),
			size: size.sub((offset.x() + SCR_PAD, 0.)).fmax(0.),
			color: t.bg,
		});

		if caret != select {
			let (beg, end) = caret_range(lines, *caret, *select);

			let x_beg = util::caret_x(util::line(lines, beg), t, scale, beg.x(), CUR_PAD);
			let x_end = util::caret_x(util::line(lines, end), t, scale, end.x(), CUR_PAD);
			for i in beg.y().max(start)..=end.y().min(start + len) {
				let x = if i != beg.y() { 0. } else { x_beg };
				let w = if i != end.y() { size.x() - offset.x() - SCR_PAD } else { x_end };
				r.draw(Rect {
					pos: p(offset.x() + x, i),
					size: (w - x, scale),
					color: t.highlight,
				});
			}
		} else if r.focused(id) && caret.y() >= start && caret.y() <= start + len {
			let x = util::caret_x(util::line(lines, *caret), t, scale, caret.x(), CUR_PAD);
			r.draw(Rect {
				pos: p(offset.x() + x, caret.y()),
				size: (CUR_PAD, scale),
				color: t.highlight,
			});
		}

		r.draw(Rect { pos, size: (offset.x(), size.y()), color: t.fg });
		wraps.iter().skip(start).take(len).enumerate().filter(|(_, &w)| w == 0).for_each(|(n, _)| {
			r.draw(Rect {
				pos: p(size.x() - SCR_PAD - CUR_PAD * 1.5, n + start),
				size: (CUR_PAD, CUR_PAD),
				color: t.highlight,
			});
		});
		lines.iter().skip(start).take(len).enumerate().for_each(|(n, text)| {
			let p = p(0., n + start);
			if !text.is_empty() {
				r.draw(Text { pos: offset.sum(p), scale, color: t.text, text, font });
			}
			let w = wraps[n + start];

			if w != 0 {
				r.draw(Text {
					pos: p,
					scale,
					color: t.highlight,
					text: &w.to_string(),
					font,
				});
			}
		});

		let (mut pip_pos, mut changes) = (typed_ptr!(&mut scrollbar.pip_pos), typed_ptr!(changes));
		r.logic(
			(pos, pos.sum(size)),
			move |e, focused, mouse_pos| {
				let changes = changes.get_mut();
				if changes.is_none() {
					*changes = Some((lines.clone().into_vec(), vec![]));
				}
				let (lines, line_cache) = changes.as_mut().valid();
				let mut _lines = typed_ptr!(lines);
				let pip = pip_pos.get_mut();
				let clampx = |c| util::clamp(lines, c);
				let setx = |c, o| util::move_caret(lines, c, (o, 0), true);
				let sety = |c, o| util::move_caret(lines, c, (0, o), false);
				let click = |p| util::caret_to_cursor(lines, vis_range, t, scale, pos.sum((offset.x(), size.y())), p);
				let move_pip = |v: f32| (*pip + v).clamp(0., 1.);
				let set_screen = |c: &Caret, at: f32| 1. - (f32(c.y()) / f32(lines.len() - len) - at).or_def(whole_text_h > size.y()).clamp(0., 1.);
				let center_pip = |c: &Caret| set_screen(c, f32(len) * scale / whole_text_h * 0.5);
				let adj_edge = |c: &Caret| {
					if c.y() <= start {
						set_screen(c, 0.)
					} else if c.y() + 1 >= start + len {
						set_screen(c, f32(len) * scale / whole_text_h)
					} else {
						*pip
					}
				};
				let range = |beg: Caret, end: Caret, text: &str| {
					let lw = lines[..beg.y()].iter().zip(&wraps[..beg.y()]);
					let start_l = util::line(lines, beg);
					let start_c = start_l.len_at_char(beg.x() - 1);
					let start = lw.fold(0, |s, (l, w)| s + l.len() + usize(*w != 0));
					let lw = lines[beg.y()..end.y()].iter().zip(&wraps[beg.y()..end.y()]);
					let end_l = util::line(lines, end);
					let wrap = wraps[end.y().max(1) - 1] == 0 && end.x() == 1;
					let end_c = end_l.len_at_char(end.x().max(1) - 1.or_def(!wrap));
					let len = if end.y() > beg.y() {
						lw.fold(0, |s, (l, w)| s + l.len() + usize(*w != 0)) + end_c
					} else {
						end_c
					};
					(start + start_c, start + len).fmin(text.len())
				};
				let lines = _lines.get_mut();

				match *e {
					OfferFocus => return Accept,
					MouseButton { state, .. } if state.pressed() => {
						*caret = click(mouse_pos);
						*select = select.or_val(state.shift(), *caret);
					}
					MouseMove { at, state, .. } if focused && state.lmb() => {
						*caret = click(at);
						*select = select.or_val(state.shift(), *caret);
						*pip = adj_edge(caret);
					}
					Scroll { at, state } => {
						let turbo = if state.ctrl() { 10. } else { 1. };
						*pip = move_pip(at.y() * scale / whole_text_h * turbo);
						return Accept;
					}
					Keyboard { key, state } if focused && state.pressed() => match key {
						Key::Escape => return DropFocus,
						Key::Right => {
							*caret = setx(clampx(*caret), if state.ctrl() { 10 } else { 1 });
							*select = select.or_val(state.shift(), *caret);
							*pip = adj_edge(caret);
						}
						Key::Left => {
							*caret = setx(clampx(*caret), -if state.ctrl() { 10 } else { 1 });
							*select = select.or_val(state.shift(), *caret);
							*pip = adj_edge(caret);
						}
						Key::Up => {
							*caret = sety(*caret, -1);
							*select = select.or_val(state.shift(), *caret);
							*pip = adj_edge(caret);
						}
						Key::Down => {
							*caret = sety(*caret, 1);
							*select = select.or_val(state.shift(), *caret);
							*pip = adj_edge(caret);
						}
						Key::PageUp => {
							*caret = sety(*caret, -i32(len));
							*select = select.or_val(state.shift(), *caret);
							*pip = center_pip(caret);
						}
						Key::PageDown => {
							*caret = sety(*caret, i32(len));
							*select = select.or_val(state.shift(), *caret);
							*pip = center_pip(caret);
						}
						Key::A if state.ctrl() => {
							*select = (1, 0);
							*caret = (util::line(lines, (0, lines.len())).utf8_len() + 1, lines.last_idx())
						}
						Key::C if state.ctrl() => {
							if *caret != *select {
								let (beg, end) = caret_range(lines, *caret, *select);
								let (b, e) = range(beg, end, text);
								RenderLock::set_clipboard(&text[b..e]);
								*pip = adj_edge(caret);
							}
						}
						Key::X if !readonly && state.ctrl() => {
							if *caret != *select {
								let (beg, end) = caret_range(lines, *caret, *select);
								let (b, e) = range(beg, end, text);
								let drained = text.str().drain(b..e);
								let drained: String = drained.collect();
								RenderLock::set_clipboard(&drained);
								history.push(Delete(drained.into(), b, beg));
								*caret = beg;
								*select = *caret;
								*pip = adj_edge(caret);
							}
						}
						Key::V if !readonly && state.ctrl() => {
							let (beg, end) = caret_range(lines, *caret, *select);
							let (b, e) = range(beg, end, text);
							let ins = RenderLock::clipboard();
							if beg != end {
								history.push(Delete(text[b..e].into(), b, beg));
								text.str().replace_range(b..e, &ins);
							} else {
								text.str().insert_str(b, &ins);
							}
							history.push(Insert(ins.into(), b, beg));
							*caret = beg;
							*select = *caret;
							*pip = adj_edge(caret);
						}
						Key::Delete if !readonly => {
							let (beg, end) = caret_range(lines, *caret, *select);
							let end = end.or_val(beg != end, setx(end, 1));
							let (b, e) = range(beg, end, text);
							let drained = text.str().drain(b..e);
							history.push(Delete(drained.collect::<String>().into(), b, beg));
							*caret = beg;
							*select = *caret;
							*pip = adj_edge(caret);
						}
						Key::Backspace if !readonly => {
							let (beg, end) = caret_range(lines, *caret, *select);
							let beg = beg.or_val(beg != end, setx(beg, -1));
							let (b, e) = range(beg, end, text);
							let drained = text.str().drain(b..e);
							history.push(Delete(drained.collect::<String>().into(), b, end));
							*caret = beg;
							*select = *caret;
							*pip = adj_edge(caret);
						}
						Key::Enter if !readonly => {
							let (beg, end) = caret_range(lines, *caret, *select);
							let (b, e) = range(beg, end, text);

							lines.insert(beg.y(), "\n");
							if beg != end {
								history.push(Delete(text[b..e].into(), b, beg));
								text.str().replace_range(b..e, "\n");
							} else {
								text.str().insert(b, '\n');
							}
							history.push(Insert("\n".into(), b, beg));
							*caret = (1, beg.y() + 1);
							*select = *caret;
							*pip = adj_edge(caret);
						}
						Key::Z if !readonly && state.ctrl() => {
							if let Some(change) = if !state.shift() { history.undo() } else { history.redo() } {
								match change {
									Insert(str, pos, at) => {
										text.str().insert_str(pos, &str);
										*caret = at;
										*select = at;
									}
									Delete(str, pos, at) => {
										text.str().drain(pos..pos + str.len());
										*caret = at;
										*select = at;
									}
								}
								*pip = adj_edge(caret);
							}
						}
						_ => (),
					},
					Char { ch } if !readonly && focused => {
						let (beg, end) = caret_range(lines, *caret, *select);
						let (b, e) = range(beg, end, text);
						let ins = ch.to_string();
						if beg != end {
							history.push(Delete(text[b..e].into(), b, beg));
							text.str().replace_range(b..e, &ins);
						} else {
							text.str().insert(b, ch);
						}
						line_cache.push([lines[beg.y()], &ins].concat().into());
						lines[beg.y()] = line_cache.last().valid();

						history.push(Insert(ins.into(), b, beg));
						*caret = beg.sum((1, 0));
						*select = *caret;
						*pip = adj_edge(caret);
					}
					_ => (),
				}
				if focused {
					Accept
				} else {
					Reject
				}
			},
			id,
		);

		if whole_text_h > size.y() {
			let visible_h = size.y() / whole_text_h;
			scrollbar.lock(r).draw((pos.sum((size.x() - SCR_PAD, 0.)), (SCR_PAD, size.y())), visible_h);
		}
	}
}

impl<'s: 'l, 'l> Lock::TextEdit<'s, 'l, '_> {
	pub fn draw(self, g: Geom, sc: f32, re: bool) {
		let Self { s, r, t } = self;
		s.draw(r, t, g, sc, re)
	}
}

#[derive(Default, Debug, Clone)]
struct History {
	changes: Vec<Change>,
	at: usize,
}
#[derive(Debug, Clone)]
enum Change {
	Insert(Str, usize, Caret),
	Delete(Str, usize, Caret),
}
impl Change {
	fn invert(self) -> Self {
		match self {
			Insert(string, usize, at) => Delete(string, usize, at),
			Delete(string, usize, at) => Insert(string, usize, at),
		}
	}
}
impl History {
	fn push(&mut self, c: impl HistoryPushArgs) {
		let c = c.get();
		let Self { changes, at, .. } = self;
		if *at < changes.len() {
			changes.drain(*at..);
		}
		if changes.len() > Self::MAXLENGTH * 2 {
			changes.drain(..changes.len() - Self::MAXLENGTH);
		}
		changes.extend_from_slice(&c);
		*at = changes.len();
	}
	fn undo(&mut self) -> Option<Change> {
		let Self { changes, at, .. } = self;
		if *at == 0 {
			return None;
		}
		*at -= 1;
		Some(changes[*at].clone().invert())
	}
	fn redo(&mut self) -> Option<Change> {
		let Self { changes, at, .. } = self;
		if *at >= changes.len() {
			return None;
		}
		*at += 1;
		Some(changes[*at - 1].clone())
	}
	const MAXLENGTH: usize = 1000;
}
use Change::*;

fn caret_range(lines: &[&str], caret: Caret, select: Caret) -> (Caret, Caret) {
	let (caret, select) = (util::clamp(lines, caret), util::clamp(lines, select));
	let seq = if caret.y() != select.y() { caret.y() > select.y() } else { caret.x() > select.x() };
	let (beg, end) = if seq { (select, caret) } else { (caret, select) };
	(beg, end)
}

trait HistoryPushArgs {
	fn get(self) -> Box<[Change]>;
}
impl<const L: usize> HistoryPushArgs for [Change; L] {
	fn get(self) -> Box<[Change]> {
		self.to_vec().into()
	}
}
impl HistoryPushArgs for Change {
	fn get(self) -> Box<[Self]> {
		[self].into()
	}
}
