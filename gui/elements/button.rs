use super::*;

#[derive(Default, Debug)]
pub struct Button {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	text: Str,
	easing: f32,
	last_pressed: bool,
	pub pressed: bool,
}
impl Button {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, layout @ Surf { pos, size }: Surf, text: &str) -> bool {
		let (s, font, hovered) = (self, &t.font, r.hovers_in(layout));

		if &*s.text != text || s.size != size {
			((s.offset, s.scale), s.size, s.text) = (u::fit_line(text, font, t.font_size, size), size, text.into());
		}
		let Self {
			offset,
			scale,
			ref mut easing,
			ref mut last_pressed,
			ref mut pressed,
			..
		} = *s;

		t.ease(easing, hovered);

		let clicked = *pressed && !*last_pressed;
		*last_pressed = *pressed;
		*pressed &= hovered;
		r.draw_with_logic(
			Rect { pos, size, color: t.fg(easing, *pressed) },
			move |e, _, _| {
				let mut press = |m: Mod| (*pressed, *last_pressed) = (m.pressed(), m.released());
				match *e {
					MouseButton { m, .. } => press(m),
					Keyboard { key: Key::Space, m } => press(m),
					_ => return Pass,
				}
				Accept
			},
			0,
		);

		r.draw(Text {
			pos: pos.sum(offset),
			color: t.text(hovered, clicked),
			scale,
			text,
			font,
		});
		clicked
	}
}

impl<'s: 'l, 'l> Lock::Button<'s, 'l, '_> {
	pub fn draw(self, g: impl Into<Surf>, te: impl AsRef<str>) -> bool {
		let Self { s, r, t } = self;
		s.draw(r, t, g.into(), te.as_ref())
	}
}
