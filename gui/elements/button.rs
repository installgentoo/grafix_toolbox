use super::*;

#[derive(Default, Debug)]
pub struct Button {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	text: Str,
	easing: f32,
	pub pressed: bool,
	pub hovered: bool,
}
impl Button {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, (pos, size): Geom, text: &str) -> bool {
		let s = self;

		if *s.text != *text || s.size != size {
			let (offset, scale) = util::fit_text(text, t, size);

			*s = Button { offset, size, scale, text: text.into(), ..*s };
		}
		let Button { easing, pressed, hovered, .. } = s;

		let delta = 1. / (t.easing * (*easing - 2.).abs());
		*easing = (*easing + if *hovered { delta } else { -delta }).clamp(0., 1.);
		let color = t.fg.mix(*easing, t.fg_focus).mix(*pressed, t.highlight);

		*pressed &= *hovered;
		let p = *pressed;
		r.draw_with_logic(
			Rect { pos, size, color },
			move |e, _, _| {
				let mut pass = |s: Mod| *pressed = s.pressed();
				match *e {
					OfferFocus => return Reject,
					MouseButton { state, .. } => pass(state),
					Keyboard { key, state } if key == Key::Space => pass(state),
					_ => (),
				}
				Reject
			},
			0,
		);
		*hovered = r.hovered();
		r.draw(Text {
			pos: pos.sum(s.offset),
			color: t.text.mix(*hovered, t.text_focus).mix(p, t.text_highlight),
			scale: s.scale,
			text,
			font: &t.font,
		});
		p
	}
}

impl<'s: 'l, 'l> Lock::Button<'s, 'l, '_> {
	pub fn draw(self, g: Geom, te: &str) -> bool {
		let Self { s, r, t } = self;
		s.draw(r, t, g, te)
	}
}
