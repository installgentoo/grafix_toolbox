pub use {button::*, hypertext::*, label::*, layout::*, lineedit::*, selector::*, slider::*, textedit::*};

macro_rules! element_lock {
	($($t: ident),+) => {
		pub mod Lock {
			#![allow(dead_code)]
			$(impl super::$t {
				pub fn lock<'s, 'l, 'r>(&'s mut self, r: &'r mut super::RenderLock<'l>) -> $t<'s, 'l, 'r>
				{
					let t = r.theme();
					$t { s: self, r, t }
				}
			}
			pub struct $t<'s, 'l, 'r> {
				pub(super) s: &'s mut super::$t,
				pub(super) r: &'r mut super::RenderLock<'l>,
				pub(super) t: &'l super::Theme,
			})+
		}
	};
}
element_lock!(Button, HyperText, Label, Layout, LineEdit, Selector, Slider, TextEdit);

#[derive(Default, Debug)]
pub struct Theme {
	pub easing: f32,
	pub bg: Color,
	pub bg_focus: Color,
	pub fg: Color,
	pub fg_focus: Color,
	pub highlight: Color,
	pub text: Color,
	pub text_focus: Color,
	pub text_highlight: Color,
	pub font: Font,
	pub font_size: f32,
}

mod button;
mod hypertext;
mod label;
mod layout;
mod lineedit;
mod selector;
mod slider;
mod textedit;
mod util;

use {super::*, util::Caret};
