use super::*;

#[derive(Default, Debug)]
pub struct Selector {
	button: Button,
	line_edit: LineEdit,
	choices: Vec<Button>,
	open: bool,
	editing: bool,
	pub choice: usize,
}
impl Selector {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, layout @ (pos, size): Geom, options: &'s mut [String]) -> usize {
		let Selector { button, line_edit, choices, open, editing, choice } = self;
		let text = options.at(*choice);

		if !*open {
			choices.clear();
			let mut pressed = typed_ptr!(&mut button.pressed);
			if button.lock(r).draw(layout, text) {
				*pressed.get_mut() = false;
				*open = true;
				line_edit.text = text.into();
			}
		} else {
			choices.resize_with(options.len(), Def);
			for (n, c) in choices.iter_mut().enumerate() {
				if c.lock(r).draw((pos.sum(size.mul((0, n + 1))), size), &options[n]) {
					*open = false;
					*editing = false;
					*choice = n;
					return *choice;
				}
			}

			let text = options.at_mut(*choice);

			if *editing {
				if !r.focused(LUID(line_edit)) {
					*text = line_edit.text.to_string();
					*open = false;
					*editing = false;
				}
			} else {
				*open &= r.hovers_in((pos, size.mul((1, options.len() + 1))))
			}

			*editing |= r.focused(LUID(line_edit));
			line_edit.lock(r).draw(layout);
		}
		*choice
	}
}

impl<'s: 'l, 'l> Lock::Selector<'s, 'l, '_> {
	pub fn draw(self, g: Geom, o: &'s mut [String]) -> usize {
		let Self { s, r, .. } = self;
		s.draw(r, g, o)
	}
}
