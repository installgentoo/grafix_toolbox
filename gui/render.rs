use super::*;

macro_rules! storage {
	($($t: ident),+) => {
		#[derive(Default, Debug)]
		pub struct ElementStorage {
			$($t: HashMap<u32, $t>,)+
		}
		impl ElementStorage {
			$(pub fn $t(&mut self, id: u32) -> &mut $t {
				self.$t.entry(id).or_insert_with(|| Def())
			})+
		}
		impl Renderer {
			$(pub fn $t(&mut self, id: u32) -> &mut $t {
				self.storage.$t(id)
			})+
		}
		impl<'l> RenderLock<'l> {
			$(pub fn $t(&mut self, id: u32) -> Lock::$t<'static, 'l, '_> {
				let s = unsafe { mem::transmute::<_, &mut ElementStorage>(&mut self.r.storage) };
				s.$t(id).lock(self)
			})+
		}
	}
}
storage!(Button, HyperText, Label, Layout, LineEdit, Selector, Slider, TextEdit);

macro_rules! DRAWABLE {
	($t: ty, $draw_spec: ident) => {
		impl<'l> DrawablePrimitive<'l> for $t {
			fn draw(self, obj_n: u32, clip: &Crop, r: &mut Renderer) {
				use ObjStore::$draw_spec as object;
				if obj_n < u32(r.objs.objs.len()) {
					let Primitive { state, o, .. } = r.objs.get_mut(obj_n);
					if let object(l) = o {
						*state = self.compare(clip, l) | r.status;
						if !state.contains(State::MISMATCH) {
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
				let p = Primitive { state: State::MISMATCH, o: object(self.obj(*clip)) };
				r.objs.objs.push(p);
			}
		}
	};
}
pub trait DrawablePrimitive<'l> {
	fn draw(self, _: u32, _: &Crop, _: &mut Renderer);
}
DRAWABLE!(Rect, Rect);
DRAWABLE!(Sprite<'l, RGB>, ImgRGB);
DRAWABLE!(Sprite<'l, RGBA>, ImgRGBA);
DRAWABLE!(Sprite9<'l, RGB>, Img9RGB);
DRAWABLE!(Sprite9<'l, RGBA>, Img9RGBA);
DRAWABLE!(Frame9<'l>, Frame9);
DRAWABLE!(Text<'l, '_>, Text);

pub struct RenderLock<'l> {
	pub(super) r: Renderer,
	n: u32,
	clip: Vec<Crop>,
	logics: Vec<LogicStorage<'l>>,
	l: Dummy<&'l Primitive>,
}
impl<'l> RenderLock<'l> {
	pub fn theme(&self) -> &'l Theme {
		unsafe { mem::transmute(&self.r.theme) }
	}
	pub fn clip(&mut self, pos: Vec2, size: Vec2) -> ClipLock<'l> {
		let Self { clip, .. } = self;
		let is_neg = size.ls((0, 0));
		clip.push((pos.sum(size.mul(is_neg)), pos.sum(size.abs())));
		ClipLock::new(self)
	}
	pub fn draw(&mut self, prim: impl DrawablePrimitive<'l>) {
		let Self { r, n, ref clip, .. } = self;
		prim.draw(*n, clip.last().valid(), r);
		*n += 1;
	}
	pub fn logic(&mut self, b_box: Crop, func: impl 'l + EventReaction, id: LogicId) {
		let (id, bound, func) = (id, LogicBound::Crop(b_box), Box(func));
		self.logics.push(LogicStorage { id, bound, func });
	}
	pub fn draw_with_logic(&mut self, prim: impl DrawablePrimitive<'l>, func: impl 'l + EventReaction, id: LogicId) {
		let Self { r, n, ref clip, logics, .. } = self;
		prim.draw(*n, clip.last().valid(), r);
		let (id, bound, func) = (id, LogicBound::Obj(*n), Box(func));
		logics.push(LogicStorage { id, bound, func });
		*n += 1;
	}
	pub fn hovers_in(&mut self, pos: Vec2, size: Vec2) -> bool {
		contains((pos, pos.sum(size)), self.r.mouse_pos)
	}
	pub fn hovered(&self) -> bool {
		let &Self { ref r, n, .. } = self;
		if n < 1 {
			return false;
		}
		let b_box = r.objs.get(n - 1).o.obj().base().bound_box();
		contains(b_box, self.r.mouse_pos)
	}
	pub fn focused(&self, l: LogicId) -> bool {
		l == self.r.focus
	}
	pub fn mouse_pos(&self) -> Vec2 {
		self.r.mouse_pos
	}
	pub fn to_clip(&self) -> Vec2 {
		self.r.to_clip
	}
	pub fn unlock(self, w: &mut impl Frame, events: &mut Vec<Event>) -> Renderer {
		let Self { mut r, logics, n, .. } = self;
		if n < u32(r.objs.objs.len()) {
			r.objs.shrink(n);
			r.flush |= State::BATCH_RESIZED;
		}
		r.consume_events(logics, events);
		r.render(w);
		r
	}
	pub fn unlock_skip_render(self, _: &mut impl Frame, events: &mut Vec<Event>) -> Renderer {
		let Self { mut r, logics, n, .. } = self;
		if n < u32(r.objs.objs.len()) {
			r.objs.shrink(n);
			r.flush |= State::BATCH_RESIZED;
		}
		r.consume_events(logics, events);
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
	to_clip: Vec2,
	focus: LogicId,
	mouse_pos: Vec2,
	theme: Theme,
	storage: ElementStorage,
}
impl Renderer {
	pub fn new(theme: Theme) -> Self {
		Self { theme, ..Def() }
	}
	pub fn lock<'l>(self) -> RenderLock<'l> {
		let L = 10_000_000_000.;
		RenderLock {
			r: self,
			clip: vec![((-L, -L), (L, L))],
			n: 0,
			logics: vec![],
			l: Dummy,
		}
	}
	fn consume_events(&mut self, logics: Vec<LogicStorage>, events: &mut Vec<Event>) {
		let Self { ref objs, focus, mouse_pos, .. } = self;

		let logics = Cell(logics);
		let logics = || unsafe { &mut *logics.as_ptr() }.iter_mut().rev();
		events.retain(|e| {
			map_variant!(&MouseMove { at, .. } = e => *mouse_pos = at);
			let refocus = if let MouseButton { state, .. } = e { state.contains(Mod::PRESS) } else { false };

			if !refocus && *focus != 0 {
				if let Some(l) = logics().find(|l| *focus == l.id) {
					match (l.func)(e, true, *mouse_pos) {
						Accept => return false,
						Reject => return true,
						DropFocus => {
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
						if let Accept = (l.func)(&OfferFocus, false, *mouse_pos) {
							defocus(focus);
							*focus = this_id;
						}
					}
					let focused = id_match(focus, l);
					match (l.func)(e, focused, *mouse_pos) {
						Accept => return false,
						Reject => return true,
						DropFocus => {
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
	fn render(&mut self, frame: &mut impl Frame) {
		let Self { vao, idxs, xyzw, uv, rgba, objs, flush, status, to_clip, .. } = self;

		if !flush.is_empty() {
			objs.batch();
			let flush = objs.flush(frame.to_clip(), &mut idxs.buff, &mut xyzw.buff, &mut rgba.buff, &mut uv.buff);

			idxs.flush(flush.0, |o| vao.BindIdxs(o));
			xyzw.flush(flush.1, |o| vao.AttribFmt(o, (0, 4)));
			rgba.flush(flush.2, |o| vao.AttribFmt(o, (1, 4, true)));
			uv.flush(flush.3, |o| vao.AttribFmt(o, (2, 2)));
		}

		frame.bind();
		frame.ClearDepth(1);

		GL::BlendFunc::Save();
		GL::DepthFunc::Save();
		GLSave!(CULL_FACE, DEPTH_TEST, DEPTH_WRITABLE, BLEND);

		GLDisable!(CULL_FACE);
		GLEnable!(DEPTH_TEST, DEPTH_WRITABLE);
		GL::BlendFunc::Set((gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
		GL::DepthFunc::Set(gl::LEQUAL);

		{
			let v = vao.Bind();
			GLDisable!(BLEND);
			objs.draw_opaque_batches(&v);

			GLEnable!(BLEND);
			objs.draw_translucent_batches(&v);
		}

		GLRestore!(CULL_FACE, DEPTH_TEST, DEPTH_WRITABLE, BLEND);
		GL::BlendFunc::Restore();
		GL::DepthFunc::Restore();

		*flush = State::empty();
		*status = State::XYZW.or_def(frame.to_clip() != *to_clip);
		*to_clip = frame.to_clip();
	}
}

#[derive(Default)]
struct Storage<T: spec::Buffer, D> {
	pub obj: spec::ArrObject<T, D>,
	pub buff: Vec<D>,
	size: usize,
}
impl<T: spec::Buffer, D: Copy> Storage<T, D> {
	fn flush(&mut self, from: Option<usize>, mut rebind: impl FnMut(&spec::ArrObject<T, D>)) {
		if let Some(from) = from {
			let Self { obj, ref buff, size } = self;
			let new_size = buff.len();
			if new_size <= *size && new_size * 2 > *size {
				obj.MapMut((from..new_size, gl::MAP_INVALIDATE_RANGE_BIT)).mem().copy_from_slice(&buff[from..]);
				return;
			}

			*size = new_size;
			*obj = spec::ArrObject::new((buff, gl::DYNAMIC_STORAGE_BIT | gl::MAP_WRITE_BIT));
			rebind(obj);
		}
	}
}
type IdxArrStorage = Storage<spec::Index, u16>;
type ArrStorage<T> = Storage<spec::Attribute, T>;

trait_alias!(EventReaction, FnMut(&Event, bool, Vec2) -> EventReply); // TODO trait alias all
struct LogicStorage<'l> {
	id: LogicId,
	bound: LogicBound,
	func: Box<dyn 'l + EventReaction>,
}
enum LogicBound {
	Crop(Crop),
	Obj(u32),
}

fn contains((b1, b2): Crop, p: Vec2) -> bool {
	!(p.ls(b1).any() || p.gt(b2).any())
}

pub struct ClipLock<'l> {
	r: Dummy<&'l u32>,
	ptr: usize,
}
impl<'l> ClipLock<'l> {
	pub fn new(r: &RenderLock<'l>) -> Self {
		let ptr = r as *const RenderLock as usize;
		Self { r: Dummy, ptr }
	}
}
impl Drop for ClipLock<'_> {
	fn drop(&mut self) {
		let clip = &mut unsafe { &mut *(self.ptr as *mut RenderLock) }.clip;
		if clip.len() > 1 {
			clip.pop();
		}
	}
}
