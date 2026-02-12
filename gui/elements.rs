pub use {button::*, hypertext::*, label::*, layout::*, lineedit::*, selector::*, slider::*, slidernum::*, textedit::*};

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
	pub font: Arc<Font>,
	pub font_size: f32,
}
impl Theme {
	pub fn ease(&self, easing: &mut f32, increase: bool) {
		let delta = 1. / (self.easing * (*easing - 2.).abs());
		*easing = (*easing + delta.or_val(increase, || -delta)).clamp(0., 1.)
	}
	pub fn fg<A, B>(&self, focus: A, highlight: B) -> Color
	where
		f32: Cast<A>,
		f32: Cast<B>,
	{
		let (f, h) = Vec2((focus, highlight));
		self.fg.mix(self.fg_focus, f).mix(self.highlight, h)
	}
	pub fn text<A, B>(&self, focus: A, highlight: B) -> Color
	where
		f32: Cast<A>,
		f32: Cast<B>,
	{
		let (f, h) = Vec2((focus, highlight));
		self.text.mix(self.text_focus, f).mix(self.text_highlight, h)
	}
}

pub fn NumericOnly() -> &'static HashSet<char> {
	LocalStatic!(HashSet<char>, { HashSet::from(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '+', '.', 'e']) })
}

mod button;
mod hypertext;
mod label;
mod layout;
mod lineedit;
mod selector;
mod slider;
mod slidernum;
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

use {super::*, util as u};
