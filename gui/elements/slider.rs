use super::*;

pub struct Slider {
	pub pip_pos: f32,
}
impl Default for Slider {
	fn default() -> Self {
		Self { pip_pos: 1. }
	}
}
impl Slider {
	pub fn draw<'s>(&'s mut self, r: &mut RenderLock<'s>, t: &Theme, pos: Vec2, size: Vec2, pip_size: f32) -> f32 {
		let vert = size.y() > size.x();
		let o = move |v: Vec2| if vert { v.y() } else { v.x() };
		let set_pip = move |v: f32| ((v - o(pos)) / o(size)).clamp(0., 1.);

		let id = LUID(self);
		let Self { pip_pos } = self;

		let p = *pip_pos * (1. - pip_size);
		r.draw_with_logic(
			Rect { pos, size, color: t.fg },
			move |e, focused, mouse_pos| {
				match e {
					OfferFocus => return Accept,
					MouseButton { state, .. } if focused && state.released() => return DropFocus,
					MouseButton { state, .. } if state.pressed() => *pip_pos = set_pip(o(mouse_pos)),
					MouseMove { at, .. } if focused => *pip_pos = set_pip(o(*at)),
					Scroll { at, .. } => {
						*pip_pos = (*pip_pos + o(at.mul((-1, 1))) * pip_size).clamp(0., 1.);
						return Accept;
					}
					_ => (),
				}
				Reject
			},
			id,
		);

		r.draw(Rect {
			pos: pos.sum(Vec2::to((!vert, vert)).mul(size).mul(p)),
			size: size.mul((if !vert { pip_size } else { 1. }, if vert { pip_size } else { 1. })),
			color: t.highlight,
		});
		p
	}
}
