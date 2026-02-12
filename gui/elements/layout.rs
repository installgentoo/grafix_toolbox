use super::*;

#[derive(Default, Debug)]
pub struct Layout {
	click: Vec2,
	pub layout: Surf,
}
impl Layout {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, content: impl FnOnce(&mut RenderLock<'l>, Surf)) {
		let (id, s, TOP_PAD, PIP_PAD) = (ref_UUID(self), self, 0.05, 0.04);

		s.layout.clamp_to_screen(r);
		let layout = s.layout.h_sub(TOP_PAD).y(PIP_PAD).size_sub(Vec2(PIP_PAD));
		let Self { click, layout: Surf { pos, size: s_size } } = s;

		let s_pos = Cell::from_mut(pos);
		let Surf { pos, size } = layout.y_self(1).h(TOP_PAD);
		r.draw_with_logic(
			Rect { pos, size, color: t.highlight },
			move |e, focused, mouse_pos| {
				match e {
					OfferFocus => (),
					MouseButton { m, .. } if m.released() => return DropFocus,
					MouseButton { .. } if focused => *click = mouse_pos.sub(s_pos.bind()),
					MouseMove { at, .. } if focused => s_pos.mutate(|p| *p = at.sub(*click)),
					_ => return Reject,
				}
				Accept
			},
			id,
		);

		let Surf { pos, size } = layout.x_self(1).y(-PIP_PAD).size(Vec2(PIP_PAD));
		r.draw_with_logic(
			Rect { pos, size, color: t.highlight },
			move |e, focused, _| {
				match e {
					OfferFocus => (),
					MouseButton { m, .. } if m.released() => return DropFocus,
					MouseMove { at, .. } if focused => {
						*s_size = at.sub(s_pos.bind().sum((s_size.x(), 0))).mul((1, -1)).sum(*s_size).sub(PIP_PAD * 0.5).fmax(TOP_PAD);
						s_pos.mutate(|(_, y)| *y = at.y() + PIP_PAD * 0.5);
					}
					_ => return Reject,
				}
				Accept
			},
			id + 1,
		);

		let Surf { pos, size } = layout;
		r.draw(Rect { pos, size, color: t.bg });

		let _c = r.clip(layout);
		content(r, layout);
	}
}

impl<'s: 'l, 'l> Lock::Layout<'s, 'l, '_> {
	pub fn draw(self, c: impl FnOnce(&mut RenderLock<'l>, Surf)) {
		let Self { s, r, t } = self;
		s.draw(r, t, c)
	}
}
