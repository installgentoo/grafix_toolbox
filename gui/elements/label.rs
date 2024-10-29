use super::*;

#[derive(Default, Debug)]
pub struct Label {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	text: Str,
}
impl Label {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, Surface { pos, size }: Surface, text: &str) {
		let s = self;

		if *s.text != *text || s.size != size {
			let (offset, scale) = util::fit_text(text, t, size);

			*s = Label { offset, size, scale, text: text.into() };
		}

		r.draw(Rect { pos, size, color: t.fg });
		r.draw(Text {
			pos: pos.sum(s.offset),
			color: t.text,
			scale: s.scale,
			text,
			font: &t.font,
		});
	}
}

impl<'s: 'l, 'l> Lock::Label<'s, 'l, '_> {
	pub fn draw(self, g: impl Into<Surface>, te: impl AsRef<str>) {
		let Self { s, r, t } = self;
		s.draw(r, t, g.into(), te.as_ref())
	}
}
