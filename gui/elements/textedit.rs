use super::{lazy::*, *};
use u::{Caret, DynamicStr, caret as ca, if_ctrl};

#[derive(Default, Debug)]
pub struct TextEdit {
	size: Vec2,
	scale: f32,
	a: mAffected,
	parser: LazyCell<ParseResult<mAffected>>,
	scrollbar: Slider,
	pub text: CachedStr,
}
#[derive(Default, Debug)]
struct mAffected {
	caret: Caret,
	select: Caret,
	history: History,
}
impl TextEdit {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, layout @ Surf { size, .. }: Surf, scale: f32, readonly: bool) {
		let SCR_PAD = 0.02;
		let CUR_PAD = 0.01;
		let (id, s, font) = (ref_UUID(self), self, &t.font);

		let lazy_parse = async move |text: Astr, font: Arc<Font>| {
			let lnum_w = {
				let lnum = f32(1 + stream::iter(text.lines()).count().await);
				let max_lnum = lnum.log10().max(1.).ceil();
				let w = font.glyph('0').adv * scale * max_lnum;
				w.or_def(w * 20. < size.x())
			};
			let (lsb, lines, lnums) = u::parse_text(&text, &font, scale, size.x() - lnum_w - SCR_PAD, char::is_whitespace).await;
			let lines: Vec<_> = stream::iter(lines.into_iter().zip(lnums.into_iter()).map(DynamicStr::new)).collect().await;
			((lsb, lnum_w), lines.into())
		};

		if s.text.changed() || scale != s.scale || size != s.size {
			let (text, font) = (s.text.clone(), font.clone());

			(s.size, s.scale, (s.a.caret, s.a.select), s.parser) = (
				size,
				scale,
				Def(),
				LazyCell::with(((Def(), vec![P(text.clone())].into()), Def()), async move |_| (lazy_parse(text, font).await, Def())),
			);
		}

		let Self { a, parser, scrollbar, .. } = s;
		let mut parser = parser.lock();
		let (((lsb, lnum_w), ref lines), ref reconcile) = *parser;
		reconcile.apply(a);
		let mAffected { caret, select, history } = a;

		let (scrollable, p, (start, len)) = {
			let (start, len) = (1. - scrollbar.pip_pos, lines.len());
			let (_, l) = u::visible_range(layout, scale, 0., len);
			let skip = f32(len - l) * start;
			let p = move |lines, n| u::line_pos(lines, font, scale, layout, skip, n, lsb);
			(len > l, p, u::visible_range(layout, scale, skip, len))
		};
		let ((beg_y, end_y, len_y), (pip_size, adv)) = (vec3((start, start + len, len)), u::visible_norm(layout, len, lines.len()));

		let Surf { pos, size } = layout.w(lnum_w);
		r.draw(Rect { pos, size, color: t.fg });

		let _c = r.clip(layout);

		let layout @ Surf { pos, size } = layout.x(lnum_w).w_sub(f32(scrollable) * SCR_PAD + lnum_w);
		r.draw(Rect { pos, size, color: t.bg });

		if !r.focused(id) {
		} else if caret != select {
			let (caret @ (_, b), select @ (_, e)) = ca::sort(*caret, *select);

			for i in b.max(beg_y)..=e.min(end_y) {
				let x = 0.0.or_val(i != b, || ca::adv(lines, font, scale, caret, 0.));
				let w = size.x().or_val(i != e, || ca::adv(lines, font, scale, select, 0.));
				let pos = pos.sum(p(lines, usize(i))).sum((x, 0));
				r.draw(Rect { pos, size: (w - x, scale), color: t.highlight });
			}
		} else {
			let x = ca::adv(lines, font, scale, *caret, CUR_PAD);
			let Surf { pos, size } = layout.xy(p(lines, usize(caret.y()))).x(x).size((CUR_PAD, scale));
			r.draw(Rect { pos, size, color: t.highlight });
		}

		let words = lines
			.iter()
			.enumerate()
			.skip(start)
			.take(len + 1)
			.map(|(n, line)| {
				let p = p(lines, n);
				if let R(_) = line {
					let Surf { pos, size } = layout.x_self(1).x(-CUR_PAD).size((CUR_PAD, CUR_PAD));
					r.draw(Rect { pos: pos.sum(p), size, color: t.highlight });
				}

				(p, line)
			})
			.collect_vec();

		words.into_iter().for_each(|(p, line)| {
			let (p, text) = (pos.sum(p), line.as_clipped_str(font, scale, size.x()));
			if !text.is_empty() {
				r.draw(Text { pos: p, scale, color: t.text, text, font });
			}

			if 0. < lnum_w
				&& let Some(text) = line.lnum()
			{
				let (pos, text) = ((pos.x() - lnum_w, p.y()), &text);
				r.draw(Text { pos, scale, color: t.highlight, text, font });
			}
		});

		let (pos, sc) = (pos.sum(p(lines, start)), Cell::from_mut(scrollbar));
		r.logic(
			layout,
			move |e, focused, mouse_pos| {
				let parser = Cell::from_mut(&mut parser);
				let (cs @ (c, s), rw, lines) = ((*caret, *select), !readonly, || (unsafe { &*parser.as_ptr() }).pipe(|((_, l), _)| l));

				let click = |c: Vec2| ca::at_pos(lines(), font, scale, start, c.sub(pos));
				let max_caret = || ca::set(lines(), vec2(isize::MAX), (0, 0));

				let move_pip = |o: f32| sc.mutate(|s| s.pip_pos = (s.pip_pos + o * adv).clamp(0., 1.).or_val(scrollable, || 1.));

				let center_pip = |sel @ (c, _): (Caret, _)| {
					f32(beg_y + len_y / 2 - c.y()).pipe(move_pip);
					sel
				};

				let clamp_pip = |sel @ (c, _): (Caret, _)| {
					if c.y() >= end_y {
						f32(beg_y + len_y - c.y()).pipe(move_pip)
					} else if c.y() <= beg_y {
						f32(beg_y + 1 - c.y()).pipe(move_pip)
					}
					sel
				};

				let move_caret = |c, o| ca::set(lines(), c, o);

				let set_caret = |m: Mod, c| (c, s.or_val(m.shift(), || c));

				let collect = |(c, s)| u::collect_range(lines().iter(), c, s).pipe(task::block_on);

				let edit = |pre, post, ins: &str, copy_out: fn(&str)| {
					let (c, s) = ca::sort(c, s);
					let s = s.or_val(c != s, || move_caret(c, (pre, 0)));
					let (c, s) = ca::sort(c, s);

					if c == s && ins.is_empty() {
						return (c, c);
					}

					if c != s {
						let del = &collect(cs);
						copy_out(del);
					}

					parser.mutate(|p| p.set(|((_, l), _)| u::replace_range(l, ins, c, s)));

					let c = move_caret(c, (post, 0));

					parser.mutate(|p| {
						let (caret_was, font) = (c, font.clone());
						p.update(async move |((_, lines), _)| {
							let caret = ca::serialise(lines.iter(), caret_was).await;

							let text = String::new()
								.tap_async(async |t| stream::iter(lines.iter()).for_each(|l| l.write_self(t)).await)
								.await;

							let (lines, parsed) = lazy_parse(text.into(), font).await.pipe(|(h, l)| (l.clone(), (h, l))); // TODO use vervec and compact() for more efficient reparses

							let caret = ca::set_async(&lines, (0, 0), (caret, 0)).await;

							let effect = move |mAffected { caret: c, select: s, history }: &mut _| {
								if *c == caret_was && c == s {
									(*c, *s) = (caret, caret);
								}
								history.add((caret, lines))
							};

							(parsed, effect.into())
						})
					});

					(c, c)
				};

				let caret = |cs| (*caret, *select) = cs;

				match *e {
					OfferFocus => return Accept,
					Defocus => move_pip(0.),
					Scroll { at, m } => return move_pip(at.y() * if_ctrl(m, 10., 1.)).pipe(|_| Accept),
					MouseButton { m, .. } if m.pressed() => set_caret(m, click(mouse_pos)).pipe(caret),
					MouseMove { at, m } if focused && m.lmb() => set_caret(m, click(at)).pipe(clamp_pip).pipe(caret),
					Keyboard { key, m } if focused && m.pressed() => {
						let x = |o| set_caret(m, move_caret(c, (o, 0)));
						let y = |o| set_caret(m, move_caret(c, (0, o)));

						match key {
							Key::Escape => return DropFocus,
							Key::Right => x(if_ctrl(m, 10, 1)).pipe(clamp_pip).pipe(caret),
							Key::Left => x(-if_ctrl(m, 10, 1)).pipe(clamp_pip).pipe(caret),
							Key::Up => y(-1).pipe(clamp_pip).pipe(caret),
							Key::Down => y(1).pipe(clamp_pip).pipe(caret),
							Key::PageUp => y(-len_y).pipe(center_pip).pipe(caret),
							Key::PageDown => y(len_y).pipe(center_pip).pipe(caret),
							Key::A if m.ctrl() => (max_caret(), (0, 0)).pipe(center_pip).pipe(caret),
							Key::C if m.ctrl() && c != s => collect(cs).pipe(RenderLock::set_clipboard),
							Key::X if rw && m.ctrl() => edit(0, 0, "", |s| RenderLock::set_clipboard(s)).pipe(clamp_pip).pipe(caret),
							Key::V if rw && m.ctrl() => edit(0, 0, &RenderLock::clipboard(), noop).pipe(clamp_pip).pipe(caret),
							Key::Delete if rw => edit(1, 0, "", noop).pipe(clamp_pip).pipe(caret),
							Key::Backspace if rw => edit(-1, 0, "", noop).pipe(clamp_pip).pipe(caret),
							Key::Return if rw => edit(0, 1, "\n", noop).pipe(clamp_pip).pipe(caret),
							Key::Z if rw && m.ctrl() => {
								let h = 's: {
									if !m.shift()
										&& let h @ Some(_) = history.undo()
									{
										break 's h;
									}
									if m.shift()
										&& let h @ Some(_) = history.redo()
									{
										break 's h;
									}
									None
								};

								if let Some((c, t)) = h {
									parser.mutate(|p| p.set(move |((_, l), _)| *l = t));
									(c, c).pipe(center_pip).pipe(caret)
								}
							}
							_ => (),
						}
					}
					Char { ch } if rw && focused => edit(0, 1, ch.as_str(), noop).pipe(clamp_pip).pipe(caret),
					_ => (),
				}
				Accept.or_def(focused)
			},
			id,
		);

		if scrollable {
			sc.mutate(|s| s.draw(r, t, layout.x_self(1).w(SCR_PAD), pip_size));
		}
	}
}

impl<'s: 'l, 'l> Lock::TextEdit<'s, 'l, '_> {
	pub fn draw(self, g: impl Into<Surf>, sc: f32) {
		let Self { s, r, t } = self;
		s.draw(r, t, g.into(), sc, false)
	}
}

#[derive(Default, Debug)]
struct History {
	states: Vec<TextState>,
	at: usize,
}
impl History {
	fn add(&mut self, (c, v): TextState) {
		let HIST_SIZE = 100;

		let Self { states, at } = self;

		if *at + 1 < states.len() {
			states.truncate(*at + 1);
		}
		states.push((c, v));

		let len = states.len();
		if len < HIST_SIZE {
			*at += 1.or_def(len > 1);
		} else {
			states.remove(0);
		}
	}
	fn undo(&mut self) -> Option<TextState> {
		let Self { states, at } = self;

		if *at < 1 {
			None?
		}
		*at -= 1;

		states.at(*at).clone().pipe(Some)
	}
	fn redo(&mut self) -> Option<TextState> {
		let Self { states, at } = self;

		if *at + 1 >= states.len() {
			None?
		}
		*at += 1;

		states.at(*at).clone().pipe(Some)
	}
}

fn noop(_: &str) {}
type Lines = VerVec<DynamicStr>;
type ParseResult<T> = ((Vec2, Lines), Effect<T>);
type TextState = (Caret, VerVec<DynamicStr>);
use DynamicStr::*;
