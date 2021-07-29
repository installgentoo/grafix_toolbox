use super::*;

#[derive(Default)]
pub struct Selector {
	button: Button,
	line_edit: LineEdit,
	choices: Vec<Button>,
	open: bool,
	editing: bool,
	pub choice: usize,
}
impl Selector {
	pub fn draw<'s>(&'s mut self, r: &mut RenderLock<'s>, t: &'s Theme, pos: Vec2, size: Vec2, options: &'s mut [String]) -> usize {
		let Self {
			button,
			line_edit,
			choices,
			open,
			editing,
			choice,
		} = self;
		let text = &options[*choice];

		if !*open {
			choices.clear();
			let mut pressed = StaticPtr!(&button.pressed);
			if button.draw(r, t, pos, size, text) {
				*pressed.get_mut() = false;
				*open = true;
				line_edit.text = text.into();
			}
		} else {
			choices.resize_def(options.len());
			for (n, c) in choices.iter_mut().enumerate() {
				if c.draw(r, t, pos.sum(size.mul((0, n + 1))), size, &options[n]) {
					*open = false;
					*editing = false;
					*choice = n;
					return *choice;
				}
			}

			let text = &mut options[usize(*choice)];

			if *editing {
				if !r.focused(LUID(line_edit)) {
					*text = line_edit.text.to_string();
					*open = false;
					*editing = false;
				}
			} else {
				*open &= r.hovers_in((pos, pos.sum(size.mul((1, options.len() + 1)))))
			}

			*editing |= r.focused(LUID(line_edit));
			line_edit.draw(r, t, pos, size);
		}
		*choice
	}
}
