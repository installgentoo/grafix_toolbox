pub use {button::*, hypertext::*, label::*, layout::*, lineedit::*, selector::*, slider::*, textedit::*};

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
	pub font: Rc<Font>,
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

macro_rules! element_lock {
	($($t: ident),+) => {
		pub(super) mod Lock {
			#![allow(dead_code)]
			$(impl super::$t {
				pub fn lock<'s, 'l: 'a, 'a>(&'s mut self, r: &'a mut RenderLock<'l>) -> $t<'s, 'l, 'a>
				{
					let t = r.theme();
					$t { s: self, r, t }
				}
			}
			pub struct $t<'s, 'l: 'a, 'a> {
				pub(super) s: &'s mut super::$t,
				pub(super) r: &'a mut RenderLock<'l>,
				pub(super) t: &'l Theme,
			})+
			use super::*;
		}
	};
}
element_lock!(Button, HyperText, Label, Layout, LineEdit, Selector, Slider, TextEdit);

use {super::*, util::Caret};
