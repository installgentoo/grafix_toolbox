use super::*;

#[derive(Debug)]
pub struct Slider {
	pub pip_pos: f32,
}
impl Default for Slider {
	fn default() -> Self {
		Self { pip_pos: 1. }
	}
}
impl Slider {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, Surf { pos, size }: Surf, pip_size: f32) -> f32 {
		let (id, Self { pip_pos }) = (ref_UUID(self), self);
		*pip_pos = pip_pos.clamp(0., 1.);

		let (vert, p) = (size.y() >= size.x(), *pip_pos);
		r.draw_with_logic(
			Rect { pos, size, color: t.fg },
			move |e, focused, mouse_pos| {
				let orient = |v: Vec2| v.y().or_val(vert, || v.x()).clamp(0., 1.);
				let mut set_pip = |p: Vec2| *pip_pos = p.sub(pos).sub(pip_size * 0.5).div(size.sub(pip_size)).pipe(orient);
				match *e {
					OfferFocus => (),
					MouseButton { m, .. } if m.released() => return DropFocus,
					MouseButton { m, .. } if m.pressed() => set_pip(mouse_pos),
					MouseMove { at, .. } if focused => set_pip(at),
					Scroll { at, .. } => *pip_pos = at.mul((-1, 1)).mul(pip_size).sum(*pip_pos).pipe(orient),
					_ => return Pass,
				}
				Accept
			},
			id,
		);

		r.draw(Rect {
			pos: pos.sum(size.sub(pip_size).mul((!vert, vert)).mul(p)),
			size: (size.x(), pip_size).or_val(vert, || (pip_size, size.y())),
			color: t.highlight,
		});
		p
	}
}

impl<'s: 'l, 'l> Lock::Slider<'s, 'l, '_> {
	pub fn draw(self, g: impl Into<Surf>) -> f32 {
		let (Self { s, r, t }, g) = (self, g.into());
		s.draw(r, t, g, g.size.min_comp())
	}
}
