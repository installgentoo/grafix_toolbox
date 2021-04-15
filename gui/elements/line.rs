use super::*;

#[derive(Default)]
pub struct LineEdit {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	caret: usize,
	pub text: CachedStr,
}
impl LineEdit {
	pub fn draw<'s>(&'s mut self, r: &mut RenderLock<'s>, t: &'s Theme, pos: Vec2, size: Vec2) {
		const CUR_PAD: f32 = 0.01;

		if self.text.changed() || self.size != size {
			let (offset, scale) = util::fit_text(&self.text, t, size);

			self.offset = offset;
			self.size = size;
			self.scale = scale;
		}

		r.draw(Rect { pos, size, color: t.bg });

		let id = LUID(self);
		let Self { offset, scale, caret, text, .. } = self;

		if r.focused(id) {
			let x = util::caret_x(text, t, *scale, *caret, CUR_PAD);
			r.draw(Rect {
				pos: offset.sum(pos).sum((x, 0.)),
				size: (CUR_PAD, *scale),
				color: t.highlight,
			});
		}

		r.draw(Text {
			pos: offset.sum(pos),
			color: t.text,
			scale: *scale,
			text,
			font: &t.font,
		});
		r.logic(
			(pos, pos.sum(size)),
			move |e, focused, mouse_pos| {
				let _text = text as *mut CachedStr;
				let clamp = |c, o| util::move_caret(&[(text as &str)], (c, 0), (o, 0), true).0;
				let click = || util::caret_to_cursor(&[(text as &str)], (0., 0.), t, *scale, (pos.x() + offset.x(), 0.), mouse_pos).0;
				let idx = |o| {
					let (pos, o) = vec2::<isize>::to((*caret, o));
					(text as &str).len_at_char(usize::to((pos + o).max(0)))
				};
				let text = unsafe { &mut *_text };

				match e {
					OfferFocus => return Accept,
					MouseButton { state, .. } if state.pressed() => *caret = click(),
					Keyboard { key, state } if focused && state.pressed() => match key {
						Key::Enter | Key::Escape if focused => return CancelFocus,
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
						text.str().insert(i, *ch);
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
