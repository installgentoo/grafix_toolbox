pub use {
	elements::*,
	parts::hex_to_rgba,
	render::{RenderLock, Renderer},
};

impl<'l> RenderLock<'l> {
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

fn clip_store() -> &'static mut (String, bool) {
	LazyStatic!((String, bool))
}

pub mod prim {
	pub use super::parts::{Frame9, Rect, Sprite, Sprite9, Text};
}

mod batch;
mod elements;
mod objects;
mod parts;
mod render;

fn LUID<T>(v: &T) -> LogicId {
	v as *const T as usize
}
type LogicId = usize;

use crate::{event::*, lib::*, math::*, GL};
use GL::{atlas::VTex2d, font::Font, *};
use {batch::*, objects::*, parts::*, window::*, Event::*, EventReply::*};
