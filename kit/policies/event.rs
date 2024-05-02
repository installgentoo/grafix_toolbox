use {super::math::*, bitflags::bitflags};

#[derive(Debug, Clone)]
pub enum Event {
	MouseMove { at: Vec2, state: Mod },
	MouseButton { button: Click, state: Mod },
	Scroll { at: Vec2, state: Mod },
	Keyboard { key: Key, state: Mod },
	Char { ch: char },
	OfferFocus,
	Defocus,
}

#[derive(Debug, Clone)]
pub enum EventReply {
	Accept,
	Reject,
	Decline,
	DropFocus,
}

bitflags! {
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mod: u32 {
	const PRESS = 0x1;
	const REPEAT = 0x2;
	const RELEASE = 0x4;
	const SHIFT = 0x10;
	const CTRL = 0x20;
	const ALT = 0x40;
	const WIN = 0x80;
	const LEFT = 0x100;
	const MID = 0x200;
	const RIGHT = 0x400;
}
}
impl Mod {
	pub fn pressed(&self) -> bool {
		self.intersects(Mod::PRESS | Mod::REPEAT)
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

#[derive(Debug, Clone)]
pub enum Click {
	Left,
	Right,
	Middle,
}

pub use glfw::Key;
