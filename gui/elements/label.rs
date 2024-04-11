use super::*;

#[derive(Default, Debug)]
pub struct Label {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	text: Str,
}
impl<'s: 'l, 'l> Lock::Label<'s, 'l, '_> {
	pub fn draw(self, pos: Vec2, size: Vec2, text: &str) {
		let Self { s, r, t } = self;

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
