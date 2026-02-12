use super::{util::caret as ca, *};

#[derive(Default, Debug)]
pub struct LineEdit {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	caret: isize,
	pub editing: bool,
	pub text: CachedStr,
}
impl LineEdit {
	pub fn edited(&self, r: &RenderLock) -> vec2<bool> {
		let editing = r.focused(ref_UUID(self));
		let r @ (_edit_started, _edited) = (!self.editing && editing, !editing && self.editing);
		r
	}
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, layout @ Surf { pos, size }: Surf, filter: Option<&'l HashSet<char>>) -> Option<&'l str> {
		let CUR_PAD = 0.01;
		let (id, s, font) = (ref_UUID(self), self, &t.font);

		if s.text.changed() || s.size != size {
			((s.offset, s.scale), s.size) = (u::fit_line(&s.text, font, t.font_size, size), size);
		}
		let Self {
			offset, scale, ref mut caret, ref mut editing, ref mut text, ..
		} = *s;

		r.draw(Rect { pos, size, color: t.bg });

		let (te @ Surf { pos, .. }, focused) = (layout.xy(offset), r.focused(id));
		if focused {
			let x = ca::adv(text, font, scale, (*caret, 0), CUR_PAD);
			let Surf { pos, size } = te.x(x).size((CUR_PAD, scale));
			r.draw(Rect { pos, size, color: t.highlight });
		}

		r.draw(Text { pos, color: t.text, scale, text, font });

		let (text, edited) = (Cell::from_mut(text), !focused && *editing);
		*editing = focused;
		r.logic(
			layout,
			move |e, focused, mouse_pos| {
				let clamp = |o| ca::set(text.bind(), (*caret + o, 0), (0, 0)).0;
				let click = |p: Vec2| ca::at_pos(text.bind(), font, scale, 0, p.sub(pos)).0;
				let idx = |o| ca::idx(text.bind(), (*caret, 0), (o, 0));
				match *e {
					OfferFocus => (),
					MouseButton { m, .. } if m.pressed() => *caret = click(mouse_pos),
					Keyboard { key, m } if focused && m.pressed() => match key {
						Key::Right => *caret = u::if_ctrl(m, 10, 1).pipe(clamp),
						Key::Left => *caret = u::if_ctrl(m, -10, -1).pipe(clamp),
						Key::Delete if idx(0) < text.bind().len() => {
							let i = idx(0);
							text.mutate(|t| t.str().remove(i));
						}
						Key::Backspace if idx(0) > 0 => {
							let i = idx(-1);
							*caret = clamp(-1);
							text.mutate(|t| t.str().remove(i));
						}
						Key::Return | Key::Escape => return DropFocus,
						_ => return Pass,
					},
					Char { ch } if focused => {
						let filter = filter.map(|f| f.get(&ch).is_some()).unwrap_or(true);
						if filter {
							let i = idx(0);
							text.mutate(|t| t.str().insert(i, ch));
							*caret += 1;
						}
					}
					_ => return Pass,
				}
				Accept
			},
			id,
		);
		None.or_val(!edited, || Some(unsafe { &**text.as_ptr() }))
	}
}

impl<'s: 'l, 'l> Lock::LineEdit<'s, 'l, '_> {
	pub fn draw(self, g: impl Into<Surf>) -> Option<&'l str> {
		let Self { s, r, t } = self;
		s.draw(r, t, g.into(), None)
	}
}
