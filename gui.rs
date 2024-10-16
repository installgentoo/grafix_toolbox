pub use {
	elements::*,
	parts::hex_to_rgba,
	render::{RenderLock, Renderer},
};

impl<'l> RenderLock<'l> {
	pub fn clipboard() -> String {
		let (str, _) = clip_store().to_owned();
		str
	}
	pub fn set_clipboard(s: &str) {
		*clip_store() = (s.into(), true);
	}
	pub fn sync_clipboard(&self, w: &mut Window) {
		let mut lock = clip_store();
		let (str, changed) = &mut *lock;
		if *changed {
			w.set_clipboard(str);
			*changed = false;
			return;
		}

		let wstr = w.clipboard();
		if *str != wstr {
			*str = wstr
		}
	}
}

fn clip_store<'s>() -> MutexGuard<'s, (String, bool)> {
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

use crate::{lib::*, math::*, sync::*};
use GL::{atlas::VTex2d, event::*, font::Font, window::*, *};
use {batch::*, objects::*, parts::*, Event::*, EventReply::*};
