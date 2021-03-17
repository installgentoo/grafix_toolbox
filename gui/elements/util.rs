use super::*;

pub fn fit_text(text: &str, t: &Theme, size: Vec2) -> (Vec2, f32) {
	let scale = t.font_size * size.y();
	let text_size = Text::size(text, &t.font, scale);
	if text_size.x() > 0. {
		let r = (size.x() / text_size.x()).min(1.);
		(size.sub(text_size.mul(r)).mul(0.5), scale * r)
	} else {
		((size.x(), size.y() - scale).mul(0.5), scale)
	}
}

pub fn caret_x(text: &str, t: &Theme, scale: f32, at: usize, pad: f32) -> f32 {
	let font = &t.font;
	let past_end = at > text.utf8_len();
	let last_ch = Text::char_at(text, font, scale, at);
	let adv = Text::adv_at(text, font, scale, at);
	let last_w = (last_ch.adv - pad).or_def(past_end);
	let empty_x = pad.or_def(text.is_empty());
	adv + last_ch.coord.x() + last_w + empty_x
}

pub fn move_caret(lines: &[&str], (x, y): Caret, (ox, oy): iVec2) -> Caret {
	let (x, ox, y, oy) = vec4::<isize>::to((x, ox, y, oy));
	let last_line = lines.last_idx();
	let y = usize::to((y + oy).min(isize::to(last_line)).max(0));
	let text = lines[y];
	let x = usize::to(if ox != 0 {
		let x = x.min(isize::to(text.utf8_len()) + 1) + ox;
		if x < 1 {
			if y == 0 {
				return (1, 0);
			};
			return move_caret(lines, vec2::<usize>::to((lines[y - 1].utf8_len() + 1, y - 1)), (i32::to(x), 0));
		}
		if x > isize::to(text.utf8_len() + 1) {
			if y == last_line {
				return (lines[last_line].utf8_len() + 1, last_line);
			};
			return move_caret(lines, vec2::<usize>::to((1, y + 1)), (i32::to(x - isize::to(text.utf8_len()) - 2), 0));
		}
		x
	} else {
		x.max(1)
	});
	(x, y)
}

pub fn caret_to_cursor(lines: &[&str], (start, len): Vec2, t: &Theme, scale: f32, pos: Vec2, (x, y): Vec2) -> Caret {
	let whole_text_h = scale * f32::to(lines.len());
	let (b, e) = vec2::<isize>::to((start, start + len));
	let y = usize::to(
		isize::to(start + (pos.y() - y) / whole_text_h * f32::to(lines.len()))
			.clamp(b, e)
			.min(isize::to(lines.last_idx())),
	);
	let text = lines[y];
	let ((caret_x, _), (str, _)) = Text::substr(text, &t.font, scale, x - pos.x());
	let past_end = x > pos.x() + caret_x;
	let c = (str.utf8_len(), y);
	let o = (1.or_def(past_end), 0);
	move_caret(lines, c, o)
}

pub type Caret = vec2<usize>;
