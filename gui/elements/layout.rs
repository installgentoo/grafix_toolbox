use super::*;

#[derive(Default, Debug)]
pub struct Layout {
	click: Vec2,
	pub pos: Vec2,
	pub size: Vec2,
}
impl Layout {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, content: impl FnOnce(&mut RenderLock<'l>, Geom)) {
		let TOP_PAD = 0.05;
		let CRN_PAD = 0.04;
		let id = ref_UUID(self);

		let Layout { click, pos, size } = self;
		{
			let c = r.to_clip();
			let (lb, ru) = ((-1., -1.).div(c), (1., 1.).div(c));
			*size = size.clmp((0., 0.), ru.mul(2).sub(CRN_PAD));
			*pos = pos.clmp(lb.sum((0, CRN_PAD)), ru.sub(*size).sub((CRN_PAD, 0)));
		}

		let layout = (*pos, size.sub((0, TOP_PAD)));

		let mut _pos = typed_ptr!(pos);
		r.draw_with_logic(
			Rect {
				pos: pos.sum((0, size.y() - TOP_PAD)),
				size: (size.x(), TOP_PAD),
				color: t.highlight,
			},
			move |e, focused, mouse_pos| {
				let p = _pos.get_mut();
				match e {
					OfferFocus => return Accept,
					MouseButton { state, .. } if focused && state.released() => return DropFocus,
					MouseButton { .. } if focused => *click = mouse_pos.sub(*p),
					MouseMove { at, .. } if focused => *p = at.sub(*click),
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

		r.draw_with_logic(
			Rect {
				pos: pos.sum((size.x(), -CRN_PAD)),
				size: (CRN_PAD, CRN_PAD),
				color: t.highlight,
			},
			move |e, focused, _| {
				match e {
					OfferFocus => return Accept,
					MouseButton { state, .. } if focused && state.released() => return DropFocus,
					MouseMove { at, .. } if focused => {
						*size = at.sub(pos.sum((size.x(), 0))).mul((1, -1)).sum(*size).sub(CRN_PAD * 0.5).fmax(TOP_PAD);
						pos.1 = at.y() + CRN_PAD * 0.5;
					}
					_ => (),
				}
				if focused {
					Accept
				} else {
					Reject
				}
			},
			id + 1,
		);

		let (pos, size) = layout;
		r.draw(Rect { pos, size, color: t.bg });
		let _c = r.clip(layout);
		content(r, layout);
	}
}

impl<'s: 'l, 'l> Lock::Layout<'s, 'l, '_> {
	pub fn draw(self, c: impl FnOnce(&mut RenderLock<'l>, Geom)) {
		let Self { s, r, t } = self;
		s.draw(r, t, c)
	}
}
