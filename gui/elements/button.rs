use super::*;

#[derive(Default)]
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
	pub fn draw<'s>(&'s mut self, r: &mut RenderLock<'s>, t: &'s Theme, pos: Vec2, size: Vec2, text: &str) -> bool {
		if *self.text != *text || self.size != size {
			let (offset, scale) = util::fit_text(text, t, size);

			*self = Self { offset, size, scale, text: text.into(), ..*self };
		}
		let Self { easing, pressed, hovered, .. } = self;

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
					OfferFocus => return DropFocus,
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
			pos: pos.sum(self.offset),
			color: t.text.mix(*hovered, t.text_focus).mix(p, t.text_highlight),
			scale: self.scale,
			text,
			font: &t.font,
		});
		p
	}
}
