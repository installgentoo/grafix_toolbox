pub use {
	elements::*,
	primitive::prim,
	render::{RenderLock, Renderer},
	surface::*,
};

pub fn hex_to_rgba(c: u32) -> Color {
	vec4(((c & 0xff000000) >> 24, (c & 0xff0000) >> 16, (c & 0xff00) >> 8, c & 0xff)).div(255)
}

pub fn pix_to_size(pixels: f32, window: &impl Frame) -> f32 {
	pixels * window.pixel() * 2.
}

impl RenderLock<'_> {
	pub fn clipboard() -> String {
		let (str, _) = clip_store().to_owned();
		str
	}
	pub fn set_clipboard(s: impl ToString) {
		*clip_store() = (s.to_string(), true);
	}
	pub fn sync_clipboard(&self, w: &mut impl Window) {
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

macro_rules! storage {
	($($t: ident),+) => {
		impl Renderer {
			$(pub fn $t(&mut self, id: u32) -> &mut $t {
				self.storage.get_mut().$t(id)
			})+
		}
		impl<'l> RenderLock<'l> {
			$(pub fn $t<'r: 'l>(&mut self, id: u32) -> Lock::$t<'r, 'l, '_> {
				let s = unsafe { &mut *self.r.storage.as_ptr() };
				s.$t(id).lock(self)
			})+
		}
		#[derive(Default, Debug)]
		struct ElementStorage {
			$($t: HashMap<u32, $t>,)+
		}
		impl ElementStorage {
			$(fn $t(&mut self, id: u32) -> &mut $t {
				self.$t.entry(id).or_insert_with(|| Def())
			})+
		}
	}
}
storage!(Button, HyperText, Label, Layout, LineEdit, Selector, Slider, SliderNum, TextEdit);

#[macro_use]
mod cache;

mod batch;
mod elements;
mod primitive;
mod render;
mod surface;

type Geom = (Vec2, Vec2);
type Color = Vec4;
type TexCoord = Vec4;
type LogicId = usize;

struct LogicStorage<'s> {
	id: LogicId,
	bound: LogicBound,
	func: Box<dyn 's + EventReaction>,
}
trait_alias!(pub EventReaction, FnMut(&Event, bool, Vec2) -> EventReply); // TODO trait alias all

enum LogicBound {
	Crop(Geom),
	Obj(u32),
}

use crate::{lazy::*, lib::*, math::*};
use GL::{atlas::VTex2d, event::*, font::Font, window::*, *};
use {Event::*, EventReply::*, batch::*, cache::*, primitive::*};
