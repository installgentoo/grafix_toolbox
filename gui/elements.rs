pub use {button::*, hypertext::*, label::*, layout::*, lineedit::*, selector::*, slider::*, slider_num::*, textedit::*};

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

pub fn NumericOnly() -> &'static HashSet<char> {
	LocalStatic!(HashSet<char>, { HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '+', '.', 'e']) })
}

derive_common_VAL! {
pub struct Surface {
	pub pos: Vec2,
	pub size: Vec2,
}}
impl Surface {
	pub fn new(pos: Vec2, size: Vec2) -> Self {
		Self { pos, size }
	}
	pub fn pos(self, pos: Vec2) -> Self {
		Self { pos, ..self }
	}
	pub fn size(self, size: Vec2) -> Self {
		Self { size, ..self }
	}
	pub fn xy(self, offset: Vec2) -> Self {
		Self { pos: self.pos.sum(offset), ..self }
	}
	pub fn x(self, offset: f32) -> Self {
		Self { pos: self.pos.sum((offset, 0.)), ..self }
	}
	pub fn y(self, offset: f32) -> Self {
		Self { pos: self.pos.sum((0., offset)), ..self }
	}
}
impl From<Geom> for Surface {
	fn from((pos, size): Geom) -> Self {
		Self { pos, size }
	}
}

mod button;
mod hypertext;
mod label;
mod layout;
mod lineedit;
mod selector;
mod slider;
mod slider_num;
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
element_lock!(Button, HyperText, Label, Layout, LineEdit, Selector, Slider, SliderNum, TextEdit);

use {super::*, util::Caret};
