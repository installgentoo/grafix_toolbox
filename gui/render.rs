use super::*;

DRAWABLE!(Rect, Rect);
DRAWABLE!(Sprite<'r, RGB>, ImgRGB);
DRAWABLE!(Sprite<'r, RGBA>, ImgRGBA);
DRAWABLE!(Sprite9<'r, RGB>, Img9RGB);
DRAWABLE!(Sprite9<'r, RGBA>, Img9RGBA);
DRAWABLE!(Frame9<'r>, Frame9);
DRAWABLE!(Text<'r, '_>, Text);

pub struct RenderLock<'s> {
	pub(super) r: Renderer,
	n: u32,
	clip: Vec<Geom>,
	logics: Vec<LogicStorage<'s>>,
}
impl<'s> RenderLock<'s> {
	pub fn theme(&self) -> &'s Theme {
		unsafe { mem::transmute(&self.r.theme) }
	}
	pub fn clip(&mut self, (pos, size): Geom) -> ClipLock<'s> {
		let Self { clip, .. } = self;
		let is_neg = size.ls((0, 0));
		clip.push((pos.sum(size.mul(is_neg)), pos.sum(size.abs())));
		ClipLock::new(self)
	}
	pub fn draw<'r: 's>(&mut self, prim: impl DrawablePrimitive<'r>) {
		let Self { r, n, ref clip, .. } = self;
		prim.draw(*n, clip.last().valid(), r);
		*n += 1;
	}
	pub fn logic(&mut self, b_box: Geom, func: impl 's + EventReaction, id: LogicId) {
		let (id, bound, func) = (id, LogicBound::Crop(b_box), Box(func));
		self.logics.push(LogicStorage { id, bound, func });
	}
	pub fn draw_with_logic<'r: 's>(&mut self, prim: impl DrawablePrimitive<'r>, func: impl 's + EventReaction, id: LogicId) {
		let Self { r, n, ref clip, logics, .. } = self;
		prim.draw(*n, clip.last().valid(), r);
		let (id, bound, func) = (id, LogicBound::Obj(*n), Box(func));
		logics.push(LogicStorage { id, bound, func });
		*n += 1;
	}
	pub fn hovers_in(&mut self, (pos, size): Geom) -> bool {
		contains((pos, pos.sum(size)), self.r.mouse_pos)
	}
	pub fn hovered(&self) -> bool {
		let &Self { ref r, n, .. } = self;
		if n < 1 {
			return false;
		}
		let b_box = r.cache.get(n - 1).o.obj().base().bound_box();
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
		if n < u32(r.cache.objs.len()) {
			r.cache.shrink(n);
			r.flush |= State::BATCH_RESIZED;
		}
		r.consume_events(logics, events);
		r.render(w);
		r
	}
	pub fn unlock_skip_render(self, _: &mut impl Frame, events: &mut Vec<Event>) -> Renderer {
		let Self { mut r, logics, n, .. } = self;
		if n < u32(r.cache.objs.len()) {
			r.cache.shrink(n);
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
	cache: RenderCache,
	flush: State,
	status: State,
	to_clip: Vec2,
	focus: LogicId,
	mouse_pos: Vec2,
	theme: Theme,
	pub(super) storage: Cell<ElementStorage>,
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
		}
	}
	fn consume_events(&mut self, logics: Vec<LogicStorage>, events: &mut Vec<Event>) {
		let Self { ref cache, focus, mouse_pos, .. } = self;

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
						Pass => FAIL!("Passthrough elements must not grab focus"),
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
						let b = cache.get(at).o.obj().base().bound_box();
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
						Pass => (),
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
		let Self { vao, idxs, xyzw, uv, rgba, cache, flush, status, to_clip, .. } = self;

		if !flush.is_empty() {
			cache.batch();
			let flush = cache.flush(frame.to_clip(), &mut idxs.buff, &mut xyzw.buff, &mut rgba.buff, &mut uv.buff);

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
			cache.draw_opaque_batches(&v);

			GLEnable!(BLEND);
			cache.draw_translucent_batches(&v);
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
	obj: spec::ArrObject<T, D>,
	buff: Vec<D>,
	size: usize,
}
impl<T: spec::Buffer, D: Copy> Storage<T, D> {
	fn flush(&mut self, from: Option<usize>, mut rebind: impl FnMut(&spec::ArrObject<T, D>)) {
		let Some(from) = from else { return () };

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
type IdxArrStorage = Storage<spec::Index, u16>;
type ArrStorage<T> = Storage<spec::Attribute, T>;

pub struct ClipLock<'s> {
	r: Dummy<&'s u32>,
	ptr: usize,
}
impl<'l: 's, 's> ClipLock<'s> {
	fn new(r: &RenderLock<'l>) -> Self {
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
