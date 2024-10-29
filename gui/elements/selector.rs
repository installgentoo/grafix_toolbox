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
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, layout @ Surface { pos, size }: Surface, options: &'s mut [String]) -> usize {
		let Selector { button, line_edit, choices, open, editing, choice } = self;
		let text = options.at(*choice);

		if !*open {
			if button.draw(r, t, layout, text) {
				*open = true;
				line_edit.text = text.into();
			}
		} else {
			choices.resize_with(options.len(), Def);
			for (n, c) in choices.iter_mut().enumerate() {
				if c.draw(r, t, layout.y(size.y() * f32(n + 1)), &options[n]) {
					*open = false;
					*editing = false;
					*choice = n;
					return *choice;
				}
			}

			let text = options.at_mut(*choice);
			let line_id = ref_UUID(line_edit);

			if *editing {
				if !r.focused(line_id) {
					*text = line_edit.text.to_string();
					*open = false;
					*editing = false;
				}
			} else {
				*open &= r.hovers_in((pos, size.mul((1, options.len() + 1))))
			}

			*editing |= r.focused(line_id);
			line_edit.draw(r, t, layout, None);
		}
		*choice
	}
}

impl<'s: 'l, 'l> Lock::Selector<'s, 'l, '_> {
	pub fn draw(self, g: impl Into<Surface>, o: &'s mut [String]) -> usize {
		let Self { s, r, t } = self;
		s.draw(r, t, g.into(), o)
	}
}
