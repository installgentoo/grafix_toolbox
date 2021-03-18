use super::*;

fn parse_text(text: &str, font: &Font, scale: f32, max_w: f32) -> (Vec<Str>, Vec<u32>) {
	let (mut lnum, mut lines, mut wraps) = (1, vec![], vec![]);
	for mut l in text.lines() {
		if l.is_empty() {
			lines.push("");
			wraps.push(lnum);
			lnum += 1;
		}
		while !l.is_empty() {
			let last_len = l.len();
			let (head, tail) = {
				let (_, (head, tail)) = Text::substr(l, font, scale, max_w);
				if tail.len() != last_len {
					(head, tail)
				} else {
					let (first_char, _) = l.char_indices().skip(1).next().unwrap_or_else(|| (l.len(), ' '));
					l.split_at(first_char)
				}
			};
			let e = tail.is_empty();
			lines.push(unsafe { mem::transmute(head) });
			wraps.push(lnum.or_def(e));
			lnum += e as u32;
			l = tail;
		}
	}
	(lines, wraps)
}

fn caret_range(lines: &[&str], mut caret: Caret, mut select: Caret) -> (Caret, Caret) {
	caret.0 = caret.x().min(get_line(lines, caret).utf8_len() + 1);
	select.0 = select.x().min(get_line(lines, select).utf8_len() + 1);
	let seq = if caret.y() != select.y() { caret.y() > select.y() } else { caret.x() > select.x() };
	let (beg, end) = if seq { (select, caret) } else { (caret, select) };
	(beg, end)
}

fn get_line<'a>(lines: &'a [&str], caret: Caret) -> &'a str {
	if lines.len() > 0 {
		lines[caret.y().min(lines.last_idx())]
	} else {
		""
	}
}

#[derive(Default)]
pub struct TextEdit {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	lines: Vec<Str>,
	wraps: Vec<u32>,
	select: Caret,
	caret: Caret,
	history: (),
	scrollbar: Slider,
	pub text: CachedStr,
}
impl TextEdit {
	pub fn draw<'s>(&'s mut self, r: &mut RenderLock<'s>, t: &'s Theme, pos: Vec2, size: Vec2, scale: f32, readonly: bool) {
		const SCR_PAD: f32 = 0.02;
		const CUR_PAD: f32 = 0.01;

		let font = &t.font;
		if self.text.changed() || scale != self.scale || size != self.size {
			let linenum_bar_w = |l| (font.char('0').adv * scale * (f32::to(l).max(1.).log10() + 1.)).min(size.x());
			let offset = (linenum_bar_w(self.text.lines().count()), 0.);
			let (lines, wraps) = parse_text(&self.text, font, scale, size.sub(offset).x() - SCR_PAD);

			self.offset = offset;
			self.size = size;
			self.scale = scale;
			self.lines = lines;
			self.wraps = wraps;
		}
		let id = LUID(self);
		let Self {
			offset,
			lines,
			wraps,
			select,
			caret,
			scrollbar,
			text,
			..
		} = self;

		r.clip(pos.sub(0), size.sum(0)); //TODO

		r.draw(Rect {
			pos: offset.sum(pos),
			size: size.sub((offset.x() + SCR_PAD, 0.)).fmax(0.),
			color: t.bg,
		});

		let len = isize::to(lines.len());
		let whole_text_h = scale * f32::to(len);
		let start = (1. - scrollbar.pip_pos) * (whole_text_h - size.y());
		let line_pos = |n| start + size.y() - scale * f32::to(n + 1);
		let vis_range = move || (start, size.y()).mul(len).div(whole_text_h).fmax(0).sum((0, 1)).fmin(len);
		let (start, len) = vec2::<usize>::to(vis_range());
		//println!("{:?}", (start, len));
		let p = |x, n| pos.sum((x, line_pos(n)));

		if caret != select {
			let (beg, end) = caret_range(lines, *caret, *select);

			let x_beg = util::caret_x(lines[beg.y()], t, scale, beg.x(), CUR_PAD);
			let x_end = util::caret_x(lines[end.y()], t, scale, end.x(), CUR_PAD);
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
			let x = util::caret_x(get_line(lines, *caret), t, scale, caret.x(), CUR_PAD);
			r.draw(Rect {
				pos: p(offset.x() + x, caret.y()),
				size: (CUR_PAD, scale),
				color: t.highlight,
			});
		}

		r.draw(Rect {
			pos,
			size: (offset.x(), size.y()),
			color: t.fg,
		});
		wraps.iter().skip(start).take(len).enumerate().filter(|(_, w)| **w == 0).for_each(|(n, _)| {
			r.draw(Rect {
				pos: p(size.x() - SCR_PAD - CUR_PAD * 1.5, n + start),
				size: (CUR_PAD, CUR_PAD),
				color: t.highlight,
			});
		});
		lines.iter().skip(start).take(len).enumerate().for_each(|(n, text)| {
			let p = p(0., n + start);
			if !text.is_empty() {
				r.draw(Text {
					pos: offset.sum(p),
					scale,
					color: t.text,
					text,
					font,
				});
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

		let p = &mut scrollbar.pip_pos as *mut f32;
		r.logic(
			(pos, pos.sum(size)),
			move |e, focused, mouse_pos| {
				let pip = unsafe { &mut *p };
				let setx = |c, o| util::move_caret(lines, c, (o, 0));
				let sety = |c, o| util::move_caret(lines, c, (0, o));
				let click = |p| util::caret_to_cursor(lines, vis_range(), t, scale, pos.sum((offset.x(), size.y())), p);
				let move_pip = |v: f32| (*pip + v).clamp(0., 1.);
				let set_screen = |c: &Caret, at: f32| 1. - (f32::to(c.y()) / f32::to(lines.len() - len) - at).clamp(0., 1.);
				let center_pip = |c: &Caret| set_screen(c, f32::to(len) * scale / whole_text_h * 0.5).or_val(whole_text_h > size.y(), 1.);
				let range = |beg: Caret, end: Caret, text: &str| {
					let lw = lines[..beg.y()].iter().zip(wraps[..beg.y()].iter());
					let start_l = get_line(lines, beg);
					let start_c = start_l.len_at_char(beg.x() - 1);
					let start = lw.fold(0, |s, (l, w)| s + l.len() + (*w != 0) as usize);
					let lw = lines[beg.y()..end.y()].iter().zip(wraps[beg.y()..end.y()].iter());
					let end_l = get_line(lines, end);
					let wrap = wraps[end.y().max(1) - 1] == 0 && end.x() == 1;
					let end_c = end_l.len_at_char(end.x().max(1) - 1.or_def(!wrap));
					let len = if end.y() > beg.y() {
						lw.fold(0, |s, (l, w)| s + l.len() + (*w != 0) as usize) + end_c
					} else {
						end_c
					};
					//println!("{:?}", (start_l, beg, end, start, start_c, len)); //TODO
					(start + start_c, start + len).fmin(text.len())
				};

				match e {
					OfferFocus => return Accept,
					MouseButton { state, .. } if state.pressed() => {
						*caret = click(mouse_pos);
						*select = select.or_val(state.shift(), *caret);
					}
					MouseMove { at, state, .. } if focused && state.lmb() => {
						*caret = click(*at);
						*select = select.or_val(state.shift(), *caret);
						*pip = center_pip(caret);
					}
					Scroll { at, state } => {
						let turbo = if state.ctrl() { 10. } else { 1. };
						*pip = move_pip(at.y() * scale / whole_text_h * turbo);
						return Accept;
					}
					Keyboard { key, state } if focused && state.pressed() => match key {
						Key::Escape => return CancelFocus,
						Key::Right => {
							*caret = setx(*caret, if state.ctrl() { 10 } else { 1 });
							*select = select.or_val(state.shift(), *caret);
							*pip = center_pip(caret);
						}
						Key::Left => {
							*caret = setx(*caret, -if state.ctrl() { 10 } else { 1 });
							*select = select.or_val(state.shift(), *caret);
							*pip = center_pip(caret);
						}
						Key::Up => {
							*caret = sety(*caret, -1);
							*select = select.or_val(state.shift(), *caret);
							*pip = center_pip(caret);
						}
						Key::Down => {
							*caret = sety(*caret, 1);
							*select = select.or_val(state.shift(), *caret);
							*pip = center_pip(caret);
						}
						Key::PageUp => {
							*caret = sety(*caret, -i32::to(len));
							*select = select.or_val(state.shift(), *caret);
							*pip = center_pip(caret);
						}
						Key::PageDown => {
							*caret = sety(*caret, i32::to(len));
							*select = select.or_val(state.shift(), *caret);
							*pip = center_pip(caret);
						}
						Key::A if state.ctrl() => {
							*select = (1, 0);
							*caret = (lines[lines.last_idx()].len().max(1), lines.last_idx())
						}
						Key::C if state.ctrl() => {}
						Key::X if !readonly && state.ctrl() => {}
						Key::V if !readonly && state.ctrl() => {}
						Key::Delete if !readonly => {
							let (beg, end) = caret_range(lines, *caret, *select);
							let end = end.or_val(beg != end, setx(end, 1));
							let (b, e) = range(beg, end, text);
							text.str().drain(b..e);
							*caret = beg;
							*select = *caret;
							*pip = center_pip(caret);
						}
						Key::Backspace if !readonly => {
							let (beg, end) = caret_range(lines, *caret, *select);
							let beg = beg.or_val(beg != end, setx(beg, -1));
							let (b, e) = range(beg, end, text);
							text.str().drain(b..e);
							*caret = beg;
							*select = *caret;
							*pip = center_pip(caret);
						}
						Key::Enter if !readonly => {
							let (beg, end) = caret_range(lines, *caret, *select);
							let (b, e) = range(beg, end, text);
							if beg != end {
								text.str().replace_range(b..e, "\n");
							} else {
								text.str().insert(b, '\n');
								//println!("{:?}", (b..e, &text[b..]));
							}
							*caret = (1, beg.y() + 1);
							*select = *caret;
							*pip = center_pip(caret);
						}
						Key::Z if state.ctrl() => {}
						_ => (),
					},
					Char { ch } if !readonly && focused => {
						let (beg, end) = caret_range(lines, *caret, *select);
						let (b, e) = range(beg, end, text);
						if beg != end {
							text.str().replace_range(b..e, &ch.to_string());
						} else {
							text.str().insert(b, *ch);
						}
						*caret = setx(beg, 1);
						*select = *caret;
						*pip = center_pip(caret);
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
			scrollbar.draw(r, t, pos.sum((size.x() - SCR_PAD, 0.)), (SCR_PAD, size.y()), visible_h);
		}
		r.unclip();
	}
}
