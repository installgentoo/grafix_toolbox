use super::{parts::*, *};
use crate::{lib::*, math::*, GL::font::Font, GL::window::*};

#[derive(Default)]
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

pub fn hex_to_rgba(c: u32) -> Color {
	Color::to(((c & 0xff000000) >> 24, (c & 0xff0000) >> 16, (c & 0xff00) >> 8, c & 0xff)).div(255)
}

impl Renderer {
	pub fn new(t: Theme) -> Self {
		let s = Self::default();
		s.set_theme(t);
		s
	}
	pub fn set_theme(&self, t: Theme) {
		ASSERT!(borrow_map().is_empty(), "Cannot change theme mid-draw");
		*theme() = t;
	}
}

pub trait GuiStorage {
	fn storage(id: u32) -> &'static mut Self;
}
macro_rules! storage {
	($t: ty) => {
		impl GuiStorage for $t {
			fn storage(id: u32) -> &'static mut Self {
				LazyStatic!(HashMap<u32, $t>).entry(id).or_default()
			}
		}
	};
}
storage!(Button);
storage!(HyperText);
storage!(Label);
storage!(Layout);
storage!(LineEdit);
storage!(Selector);
storage!(Slider);
storage!(TextEdit);

impl<'l> RenderLock<'l> {
	pub fn Button(&mut self, id: u32, pos: Vec2, size: Vec2, text: &str) -> bool {
		check_borrow(id);
		Button::storage(id).draw(self, t(), pos, size, text) // TODO store in the renderlock
	}
	pub fn Label(&mut self, id: u32, pos: Vec2, size: Vec2, text: &str) {
		check_borrow(id);
		Label::storage(id).draw(self, t(), pos, size, text)
	}
	pub fn HyperText(&mut self, id: u32, pos: Vec2, size: Vec2, scale: f32, db: &HyperDB) {
		check_borrow(id);
		HyperText::storage(id).draw(self, t(), pos, size, scale, db)
	}
	pub fn Layout(&mut self, id: u32, content: impl FnOnce(&mut RenderLock<'l>, Crop)) {
		check_borrow(id);
		Layout::storage(id).draw(self, t(), content)
	}
	pub fn LineEdit(&mut self, id: u32, pos: Vec2, size: Vec2) {
		check_borrow(id);
		LineEdit::storage(id).draw(self, t(), pos, size)
	}
	pub fn Selector(&mut self, id: u32, pos: Vec2, size: Vec2, options: &'l mut [String]) -> usize {
		check_borrow(id);
		Selector::storage(id).draw(self, t(), pos, size, options)
	}
	pub fn Slider(&mut self, id: u32, pos: Vec2, size: Vec2, pip_size: f32) -> f32 {
		check_borrow(id);
		Slider::storage(id).draw(self, t(), pos, size, pip_size)
	}
	pub fn TextEdit(&mut self, id: u32, pos: Vec2, size: Vec2, scale: f32) {
		check_borrow(id);
		TextEdit::storage(id).draw(self, t(), pos, size, scale, false)
	}
	pub fn TextList(&mut self, id: u32, pos: Vec2, size: Vec2, scale: f32) {
		check_borrow(id);
		TextEdit::storage(id).draw(self, t(), pos, size, scale, true)
	}
	pub fn clipboard() -> &'l str {
		let (str, _) = clip_store();
		str
	}
	pub fn set_clipboard(s: &str) {
		*clip_store() = (s.into(), true);
	}
	pub fn sync_clipboard(&self, w: &mut Window) {
		let (str, changed) = clip_store();
		if *changed {
			w.set_clipboard(str);
			*changed = false;
		}
	}
}

pub fn borrow_map() -> &'static mut HashSet<u32> {
	LazyStatic!(HashSet<u32>)
}
fn check_borrow(id: u32) {
	ASSERT!(
		borrow_map().get(&id).is_none(),
		"An element {:?} cannot be drawn more than once per frame",
		chksum::collision_map().get(&id).unwrap()
	);
	borrow_map().insert(id);
}

fn clip_store() -> &'static mut (String, bool) {
	LazyStatic!((String, bool))
}

fn theme() -> &'static mut Theme {
	LazyStatic!(Theme)
}
fn t() -> &'static Theme {
	let t = theme();
	ASSERT!(!t.font.glyphs.is_empty(), "No theme set for gui");
	t
}
