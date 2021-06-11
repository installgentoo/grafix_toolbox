use super::{objects::*, parts::*};
use crate::events::*;
use crate::uses::{math::*, *};
use crate::GL::{opengl, spec, window::*, Vao, RGB, RGBA};

macro_rules! Draw {
	($t: ty, $draw_spec: tt) => {
		impl<'l, 'a> DrawablePrimitive<'l, 'a> for $t {
			fn draw(self, obj_n: u32, clip: &Crop, r: &mut Renderer) {
				use ObjStore::$draw_spec as object;
				if obj_n < u32::to(r.objs.objs.len()) {
					let Primitive { state, o, .. } = r.objs.get(obj_n);
					if let object(l) = o {
						*state = self.compare(clip, l) | r.status;
						if !state.contains(State::TYPE) {
							if !state.is_empty() {
								r.flush |= *state;
								*o = object(self.obj(*clip));
							}
							return;
						}
					}

					r.objs.shrink(obj_n);
				}

				r.flush |= State::FULL;
				r.objs.objs.push(Primitive {
					state: State::MISMATCH,
					o: object(self.obj(*clip)),
				});
			}
		}
	};
}
pub trait DrawablePrimitive<'l, 'a> {
	fn draw(self, _: u32, _: &Crop, _: &mut Renderer);
}
Draw!(Rect, DrawRect);
Draw!(Sprite<'l, RGB>, DrawImgRGB);
Draw!(Sprite<'l, RGBA>, DrawImgRGBA);
Draw!(Sprite9<'l, RGB>, DrawImg9RGB);
Draw!(Sprite9<'l, RGBA>, DrawImg9RGBA);
Draw!(Frame9<'l>, DrawFrame9);
Draw!(Text<'l, 'a>, DrawText);

#[derive(Default)]
pub struct RenderLock<'l> {
	r: Renderer,
	n: u32,
	clip: Vec<Crop>,
	logics: Vec<LogicStorage<'l>>,
	l: Dummy<&'l Primitive>,
}
impl<'l> RenderLock<'l> {
	pub fn clip(&mut self, pos: Vec2, size: Vec2) {
		let Self { clip, .. } = self;
		let is_neg = size.ls((0, 0));
		clip.push((pos.sum(size.mul(is_neg)), pos.sum(size.abs())));
	}
	pub fn unclip(&mut self) {
		let Self { clip, .. } = self;
		if clip.len() > 1 {
			clip.pop();
		}
	}
	pub fn draw<'a, T: DrawablePrimitive<'l, 'a>>(&mut self, prim: T) {
		let Self { r, n, clip, .. } = self;
		prim.draw(*n, EXPECT!(clip.iter().last()), r);
		*n += 1;
	}
	pub fn logic<F: 'l + FnMut(&Event, bool, Vec2) -> EventReply>(&mut self, b_box: Crop, func: F, id: LogicId) {
		let (id, bound, func) = (id, LogicBound::Crop(b_box), Box::new(func));
		self.logics.push(LogicStorage { id, bound, func });
	}
	pub fn draw_with_logic<'a, T: DrawablePrimitive<'l, 'a>, F: 'l + FnMut(&Event, bool, Vec2) -> EventReply>(&mut self, prim: T, func: F, id: LogicId) {
		let Self { r, n, clip, logics, .. } = self;
		prim.draw(*n, EXPECT!(clip.iter().last()), r);
		let (id, bound, func) = (id, LogicBound::Obj(*n), Box::new(func));
		logics.push(LogicStorage { id, bound, func });
		*n += 1;
	}
	pub fn hovers_in(&mut self, b_box: Crop) -> bool {
		contains(b_box, self.r.mouse_pos)
	}
	pub fn hovered(&mut self) -> bool {
		let Self { r, n, .. } = self;
		if *n < 1 {
			return false;
		}
		let b_box = r.objs.get(*n - 1).o.obj().base().bound_box();
		self.hovers_in(b_box)
	}
	pub fn focused(&self, l: LogicId) -> bool {
		l == self.r.focus
	}
	pub fn mouse_pos(&self) -> Vec2 {
		self.r.mouse_pos
	}
	pub fn unlock(self, events: &mut Vec<Event>) -> Renderer {
		debug_assert!({
			super::sugar::borrow_map().clear();
			true
		});
		let Self { mut r, logics, n, .. } = self;
		if n < u32::to(r.objs.objs.len()) {
			r.objs.shrink(n);
		}
		r.consume_events(logics, events);
		r.render();
		r
	}
}

#[derive(Default)]
pub struct Renderer {
	vao: Vao<u16>,
	idxs: IdxArrStorage,
	xyzw: ArrStorage<f16>,
	uv: ArrStorage<f16>,
	rgba: ArrStorage<u8>,
	objs: Objects,
	flush: State,
	status: State,
	aspect: Vec2,
	focus: LogicId,
	mouse_pos: Vec2,
}
impl Renderer {
	pub fn lock<'l>(self) -> RenderLock<'l> {
		const L: f32 = 10_000_000_000.;
		RenderLock {
			r: self,
			clip: vec![((-L, -L), (L, L))],
			..Default::default()
		}
	}
	pub fn default() -> Self {
		Self {
			aspect: Window::aspect(),
			..Default::default()
		}
	}
	fn consume_events(&mut self, logics: Vec<LogicStorage>, events: &mut Vec<Event>) {
		let Self { objs, focus, mouse_pos, .. } = self;

		let logics = UnsafeCell::new(logics);
		let logics = || unsafe { &mut *logics.get() }.iter_mut().rev();
		events.retain(|e| {
			use {Event::*, EventReply::*};

			if let MouseMove { at, .. } = e {
				*mouse_pos = *at
			};

			let refocus = if let MouseButton { state, .. } = e { state.contains(Mod::PRESS) } else { false };

			if !refocus && *focus != 0 {
				if let Some(l) = logics().find(|l| *focus == l.id) {
					match (l.func)(&e, true, *mouse_pos) {
						Accept => return false,
						Reject => return true,
						DropFocus => *focus = 0,
						CancelFocus => {
							*focus = 0;
							return false;
						}
						Decline => (),
					}
				} else {
					*focus = 0;
				}
			}

			let id_match = |f: &LogicId, l: &LogicStorage| *f == l.id && l.id != 0;
			let defocus = |f: &LogicId| logics().find(|l| id_match(f, l)).map(|l| (l.func)(&Defocus, false, *mouse_pos));

			let mut grabbed_focus = false;
			for l in logics() {
				use LogicBound::*;
				let b_box = match l.bound {
					Crop(b_box) => b_box,
					Obj(at) => {
						let b = objs.get(at).o.obj().base().bound_box();
						l.bound = Crop(b);
						b
					}
				};
				if contains(b_box, *mouse_pos) {
					let (this_id, focused) = (l.id, id_match(focus, l) || grabbed_focus);
					grabbed_focus |= focused;

					if refocus && !focused {
						match (l.func)(&OfferFocus, false, *mouse_pos) {
							Accept => {
								defocus(focus);
								*focus = this_id;
							}
							DropFocus => {
								defocus(focus);
								*focus = 0;
							}
							_ => (),
						}
					}
					let focused = id_match(focus, l);
					match (l.func)(&e, focused, *mouse_pos) {
						Accept => return false,
						Reject => return true,
						DropFocus => *focus = 0,
						CancelFocus => {
							*focus = 0;
							return false;
						}
						Decline => (),
					}
				}
			}

			if refocus && *focus != 0 {
				defocus(focus);
				*focus = 0;
			}

			true
		});
	}
	fn render(&mut self) {
		let Self {
			vao,
			idxs,
			xyzw,
			uv,
			rgba,
			objs,
			flush,
			status,
			aspect,
			..
		} = self;
		if !flush.is_empty() {
			objs.batch();
			let flush = objs.flush(&mut idxs.buff, &mut xyzw.buff, &mut rgba.buff, &mut uv.buff);

			idxs.flush(flush.0, |o| vao.BindIdxs(o));
			xyzw.flush(flush.1, |o| vao.AttribFmt(o, (0, 4)));
			rgba.flush(flush.2, |o| vao.AttribFmt(o, (1, 4, true)));
			uv.flush(flush.3, |o| vao.AttribFmt(o, (2, 2)));
		}

		GL::ClearDepth(1.);

		GL::BlendFunc::Save();
		GL::DepthFunc::Save();
		GLSave!(CULL_FACE, DEPTH_TEST, DEPTH_WRITEMASK, BLEND);

		GLDisable!(CULL_FACE);
		GLEnable!(DEPTH_TEST, DEPTH_WRITEMASK);
		GL::BlendFunc::Set((gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
		GL::DepthFunc::Set(gl::LEQUAL);

		{
			let v = vao.Bind();
			GLDisable!(BLEND);
			objs.draw_opaque_batches(&v);

			GLEnable!(BLEND);
			objs.draw_transparent_batches(&v);
		}

		GLRestore!(CULL_FACE, DEPTH_TEST, DEPTH_WRITEMASK, BLEND);
		GL::BlendFunc::Restore();
		GL::DepthFunc::Restore();

		*flush = State::empty();
		*status = State::XYZW.or_def(Window::aspect() != *aspect);
		*aspect = Window::aspect();
	}
}

#[derive(Default)]
struct Storage<T: spec::State, D> {
	pub obj: spec::ArrObject<T, D>,
	pub buff: Vec<D>,
	size: usize,
}
impl<T: spec::State + spec::Buffer, D: Copy> Storage<T, D> {
	fn flush(&mut self, from: Option<usize>, mut rebind: impl FnMut(&spec::ArrObject<T, D>)) {
		if let Some(from) = from {
			let Self { obj, buff, size } = self;
			let new_size = buff.len();
			if new_size <= *size && new_size * 2 > *size {
				obj.MapRangeMut((from, new_size - from, gl::MAP_INVALIDATE_RANGE_BIT)).mem().copy_from_slice(&buff[from..]);
				return;
			}

			*size = new_size;
			*obj = spec::ArrObject::new((&buff[..], gl::DYNAMIC_STORAGE_BIT | gl::MAP_WRITE_BIT));
			rebind(obj);
		}
	}
}
type IdxArrStorage = Storage<spec::Index, u16>;
type ArrStorage<T> = Storage<spec::Attribute, T>;

struct LogicStorage<'l> {
	id: LogicId,
	bound: LogicBound,
	func: Box<dyn 'l + FnMut(&Event, bool, Vec2) -> EventReply>,
}
enum LogicBound {
	Crop(Crop),
	Obj(u32),
}
pub fn LUID<T>(v: &T) -> usize {
	v as *const T as usize
}
type LogicId = usize;

fn contains((b1, b2): Crop, (x, y): Vec2) -> bool {
	!(x < b1.x() || x > b2.x() || y < b1.y() || y > b2.y())
}
