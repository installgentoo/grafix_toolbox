use super::*;

#[derive(Default, Debug)]
pub struct Label {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	text: Str,
}
impl Label {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, Surf { pos, size }: Surf, text: &str) {
		let (s, font) = (self, &t.font);

		if *s.text != *text || s.size != size {
			let (offset, scale) = u::fit_line(text, font, t.font_size, size);
			*s = Self { offset, size, scale, text: text.into() };
		}
		let Self { offset, scale, .. } = *s;

		r.draw(Rect { pos, size, color: t.fg });

		r.draw(Text { pos: pos.sum(offset), color: t.text, scale, text, font });
	}
}

impl<'s: 'l, 'l> Lock::Label<'s, 'l, '_> {
	pub fn draw(self, g: impl Into<Surf>, te: impl AsRef<str>) {
		let Self { s, r, t } = self;
		s.draw(r, t, g.into(), te.as_ref())
	}
}
