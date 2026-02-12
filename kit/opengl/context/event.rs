use super::*;

#[derive(Debug, Clone)]
pub enum Event {
	MouseMove { at: Vec2, m: Mod },
	MouseButton { button: Click, m: Mod },
	Scroll { at: Vec2, m: Mod },
	Keyboard { key: Key, m: Mod },
	Char { ch: char },
	OfferFocus,
	Defocus,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum EventReply {
	Accept,
	Reject,
	#[default]
	Pass,
	DropFocus,
}

bitflags! {#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Mod: u16 {
	const PRESS = 0x1;
	const RELEASE = 0x2;
	const SHIFT = 0x10;
	const CTRL = 0x20;
	const ALT = 0x40;
	const WIN = 0x80;
	const LEFT = 0x100;
	const MID = 0x200;
	const RIGHT = 0x400;
}}
impl Mod {
	pub fn pressed(&self) -> bool {
		self.contains(Mod::PRESS)
	}
	pub fn released(&self) -> bool {
		self.contains(Mod::RELEASE)
	}
	pub fn ctrl(&self) -> bool {
		self.contains(Mod::CTRL)
	}
	pub fn shift(&self) -> bool {
		self.contains(Mod::SHIFT)
	}
	pub fn alt(&self) -> bool {
		self.contains(Mod::ALT)
	}
	pub fn win(&self) -> bool {
		self.contains(Mod::WIN)
	}
	pub fn lmb(&self) -> bool {
		self.contains(Mod::LEFT)
	}
	pub fn mmb(&self) -> bool {
		self.contains(Mod::MID)
	}
	pub fn rmb(&self) -> bool {
		self.contains(Mod::RIGHT)
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Click {
	Left,
	Right,
	Middle,
}

//pub use glfw::Key;
pub use sdl2::keyboard::Keycode as Key;
