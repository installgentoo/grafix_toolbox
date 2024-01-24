use super::*;

#[derive(Default)]
pub struct Layout {
	click: Vec2,
	pub pos: Vec2,
	pub size: Vec2,
}
impl Layout {
	pub fn draw<'s>(&'s mut self, r: &mut RenderLock<'s>, t: &Theme, content: impl FnOnce(&mut RenderLock<'s>, Crop)) {
		const TOP_PAD: f32 = 0.05;
		const CRN_PAD: f32 = 0.04;

		let id = LUID(self);
		let Self { click, pos, size } = self;
		{
			let a = r.aspect();
			let (lb, ru) = ((-1., -1.).div(a), (1., 1.).div(a));
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
				Reject
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
				Reject
			},
			id + 1,
		);

		let (pos, size) = layout;
		let _c = r.clip(pos, size);
		r.draw(Rect { pos, size, color: t.bg });
		content(r, layout);
	}
}
