use super::*;

#[derive(Default, Debug)]
pub struct LineEdit {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	caret: usize,
	pub text: CachedStr,
}
impl LineEdit {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, (pos, size): Geom) {
		let CUR_PAD = 0.01;
		let s = self;

		if s.text.changed() || s.size != size {
			let (offset, scale) = util::fit_text(&s.text, t, size);

			s.offset = offset;
			s.size = size;
			s.scale = scale;
		}

		r.draw(Rect { pos, size, color: t.bg });

		let id = LUID(s);
		let &mut LineEdit { offset, scale, ref mut caret, ref mut text, .. } = s;

		if r.focused(id) {
			let x = util::caret_x(text, t, scale, *caret, CUR_PAD);
			r.draw(Rect {
				pos: offset.sum(pos).sum((x, 0.)),
				size: (CUR_PAD, scale),
				color: t.highlight,
			});
		}

		r.draw(Text {
			pos: offset.sum(pos),
			color: t.text,
			scale,
			text,
			font: &t.font,
		});
		r.logic(
			(pos, pos.sum(size)),
			move |e, focused, mouse_pos| {
				let mut _text = typed_ptr!(text);
				let clamp = |c, o| util::move_caret(&[(text as &str)], (c, 0), (o, 0), true).0;
				let click = || util::caret_to_cursor(&[(text as &str)], (0., 0.), t, scale, (pos.x() + offset.x(), 0.), mouse_pos).0;
				let idx = |o| {
					let (pos, o) = ilVec2((*caret, o));
					(text as &str).len_at_char(usize((pos + o).max(0)))
				};
				let text = _text.get_mut();

				match *e {
					OfferFocus => return Accept,
					MouseButton { state, .. } if state.pressed() => *caret = click(),
					Keyboard { key, state } if focused && state.pressed() => match key {
						Key::Enter | Key::Escape if focused => return DropFocus,
						Key::Right => *caret = clamp(*caret, if state.ctrl() { 10 } else { 1 }),
						Key::Left => *caret = clamp(*caret, -if state.ctrl() { 10 } else { 1 }),
						Key::Delete if idx(-1) < text.len() => {
							let i = idx(-1);
							text.str().remove(i);
						}
						Key::Backspace if idx(-1) > 0 => {
							let i = idx(-2);
							*caret = clamp(*caret, -1);
							text.str().remove(i);
						}
						_ => (),
					},
					Char { ch } if focused => {
						let i = idx(-1);
						text.str().insert(i, ch);
						*caret = clamp(*caret, 1);
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
	}
}

impl<'s: 'l, 'l> Lock::LineEdit<'s, 'l, '_> {
	pub fn draw(self, g: Geom) {
		let Self { s, r, t } = self;
		s.draw(r, t, g)
	}
}
