use super::*;

#[derive(Default, Debug)]
pub struct Label {
	offset: Vec2,
	size: Vec2,
	scale: f32,
	text: Str,
}
impl Label {
	pub fn draw<'s>(&'s mut self, r: &mut RenderLock<'s>, t: &'s Theme, pos: Vec2, size: Vec2, text: &str) {
		if *self.text != *text || self.size != size {
			let (offset, scale) = util::fit_text(text, t, size);

			*self = Self { offset, size, scale, text: text.into() };
		}

		r.draw(Rect { pos, size, color: t.fg });
		r.draw(Text {
			pos: pos.sum(self.offset),
			color: t.text,
			scale: self.scale,
			text,
			font: &t.font,
		});
	}
}
