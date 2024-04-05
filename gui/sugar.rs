use super::{parts::*, *};
use crate::{lib::*, math::*, GL::font::Font, GL::window::*};

pub fn gui() -> &'static mut GUI {
	LazyStatic!(GUI)
}

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
		gui().theme = t;
	}
}

impl<'l> RenderLock<'l> {
	pub fn Button(&mut self, id: u32, pos: Vec2, size: Vec2, text: &str) -> bool {
		gui().Button(id).draw(self, t(), pos, size, text)
	}
	pub fn Label(&mut self, id: u32, pos: Vec2, size: Vec2, text: &str) {
		gui().Label(id).draw(self, t(), pos, size, text)
	}
	pub fn HyperText(&mut self, id: u32, pos: Vec2, size: Vec2, scale: f32, db: &HyperDB) {
		gui().HyperText(id).draw(self, t(), pos, size, scale, db)
	}
	pub fn Layout(&mut self, id: u32, content: impl FnOnce(&mut RenderLock<'l>, Crop)) {
		gui().Layout(id).draw(self, t(), content)
	}
	pub fn LineEdit(&mut self, id: u32, pos: Vec2, size: Vec2) {
		gui().LineEdit(id).draw(self, t(), pos, size)
	}
	pub fn Selector(&mut self, id: u32, pos: Vec2, size: Vec2, options: &'l mut [String]) -> usize {
		gui().Selector(id).draw(self, t(), pos, size, options)
	}
	pub fn Slider(&mut self, id: u32, pos: Vec2, size: Vec2, pip_size: f32) -> f32 {
		gui().Slider(id).draw(self, t(), pos, size, pip_size)
	}
	pub fn TextEdit(&mut self, id: u32, pos: Vec2, size: Vec2, scale: f32) {
		gui().TextEdit(id).draw(self, t(), pos, size, scale, false)
	}
	pub fn TextList(&mut self, id: u32, pos: Vec2, size: Vec2, scale: f32) {
		gui().TextEdit(id).draw(self, t(), pos, size, scale, true)
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
fn t() -> &'static Theme {
	let t = &gui().theme;
	ASSERT!(!t.font.glyphs.is_empty(), "No theme set for gui");
	t
}

fn clip_store() -> &'static mut (String, bool) {
	LazyStatic!((String, bool))
}

macro_rules! storage {
	($($t: ident),+) => {
		#[derive(Default, Debug)]
		pub struct GUI {
			theme: Theme,
			$($t: HashMap<u32, $t>,)+
		}
		impl GUI {
			$(pub fn $t(&mut self, id: u32) -> &mut $t {
				self.$t.entry(id).or_insert_with(|| Def())
			})+
		}
	}
}
storage!(Button, HyperText, Label, Layout, LineEdit, Selector, Slider, TextEdit);
