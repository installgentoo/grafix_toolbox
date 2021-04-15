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

pub fn move_caret(lines: &[&str], (x, y): Caret, (ox, oy): iVec2, clamp_x: bool) -> Caret {
	let (x, ox, y, oy) = vec4::<isize>::to((x, ox, y, oy));
	let last_idx = lines.last_idx();
	let y = usize::to((y + oy).clamp(0, isize::to(last_idx)));
	let text = lines[y];
	let x = {
		let x = x + ox;
		if x + ox < 1 {
			if y == 0 {
				return (1, 0);
			};
			let skip = isize::to(lines[y - 1].utf8_len() + 1);
			return move_caret(lines, (0, y - 1), iVec2::to((x + skip, 0)), true);
		}
		if clamp_x && x > isize::to(text.utf8_len() + 1) {
			if y == last_idx {
				return (lines[last_idx].utf8_len() + 1, last_idx);
			};
			return move_caret(lines, vec2::<usize>::to((x - isize::to(text.utf8_len() + 1), y + 1)), (0, 0), true);
		}
		x
	};
	vec2::<usize>::to((x, y))
}

pub fn caret_to_cursor(lines: &[&str], (start, len): Vec2, t: &Theme, scale: f32, pos: Vec2, (x, y): Vec2) -> Caret {
	let (b, e) = vec2::<isize>::to((start, start + len));
	let y = isize::to(start + (pos.y() - y) / scale).clamp(b - 1, e);
	let text = line(lines, (0, y));
	let ((caret_x, _), (str, _)) = Text::substr(text, &t.font, scale, x - pos.x());
	let past_end = x > pos.x() + caret_x;
	let c = (str.utf8_len(), usize::to(y));
	let o = (1.or_def(past_end), 0);
	move_caret(lines, c, o, true)
}

pub fn clamp(lines: &[&str], c: Caret) -> Caret {
	c.clmp((1, 0), (line(lines, c).utf8_len() + 1, lines.last_idx()))
}

pub fn line<'a, X, Y>(lines: &'a [&str], caret: (X, Y)) -> &'a str
where
	vec2<isize>: Cast<(X, Y)>,
{
	let (_, y) = vec2::<isize>::to(caret);
	let y = usize::to(y.clamp(0, isize::to(lines.last_idx())));
	if lines.len() > 0 {
		unsafe { lines.get_unchecked(y) }
	} else {
		""
	}
}

pub type Caret = vec2<usize>;
