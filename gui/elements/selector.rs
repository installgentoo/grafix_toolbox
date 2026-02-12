use super::*;

#[derive(Default, Debug)]
pub struct Selector {
	button: Button,
	edit: LineEdit,
	choices: Vec<Button>,
	open: bool,
	pub choice: usize,
}
impl Selector {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, layout: Surf, options: &'s mut [String]) -> &'s str {
		if options.is_empty() {
			return "";
		}

		let (len, Self { button, edit, choices, open, choice }) = (options.len(), self);

		*choice = (*choice).min(len - 1);
		choices.resize_with(len, Def);

		if !*open {
			let text = &options[*choice];

			if button.draw(r, t, layout, text) {
				(*open, (*edit, *choices)) = (true, Def());
				edit.text = text.into();
			}

			return text;
		}

		for (n, c) in choices.iter_mut().enumerate() {
			if c.draw(r, t, layout.y_self(n + 1), &options[n]) {
				(*choice, (*open, *button)) = (n, Def());
				return &options[n];
			}
		}

		let text = options.at_mut(*choice);

		if let Some(edit) = edit.draw(r, t, layout, None) {
			(*text, *open) = (edit.into(), false);
		}

		*open &= r.hovers_in(layout.h_scale(len + 1));

		options.at(*choice)
	}
}

impl<'s: 'l, 'l> Lock::Selector<'s, 'l, '_> {
	pub fn draw(self, g: impl Into<Surf>, o: &'s mut [String]) -> &'s str {
		let Self { s, r, t } = self;
		s.draw(r, t, g.into(), o)
	}
}
