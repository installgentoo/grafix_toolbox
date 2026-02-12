use super::*;

DRAWABLE!(Rect, Rect);
DRAWABLE!(Sprite<'r, RGB>, ImgRGB);
DRAWABLE!(Sprite<'r, RGBA>, ImgRGBA);
DRAWABLE!(Sprite9<'r, RGB>, Img9RGB);
DRAWABLE!(Sprite9<'r, RGBA>, Img9RGBA);
DRAWABLE!(Frame9<'r>, Frame9);
DRAWABLE!(Text<'r, '_>, Text);

pub struct RenderLock<'l> {
	pub(super) r: Renderer,
	n: u32,
	crop: Rc<Cell<Geom>>,
	logics: Vec<LogicStorage<'l>>,
}
impl<'l> RenderLock<'l> {
	pub fn theme(&self) -> &'l Theme {
		unsafe { mem::transmute(&self.r.theme) }
	}
	pub fn clip(&self, s: Surf) -> ClipLock {
		let Self { crop, .. } = self;
		let (lock, prev_crop @ (c1, c2)) = (crop.clone(), crop.get());
		let (p1, p2) = s.b_box();
		crop.set((p1.fmax(c1), p2.fmin(c2)));
		ClipLock { lock, prev_crop }
	}
	pub fn unclipped(&mut self, f: impl FnOnce(&mut Self)) {
		let c = self.crop.replace(uncropped());
		f(self);
		self.crop.set(c)
	}
	pub fn draw<'r: 'l>(&mut self, prim: impl DrawablePrimitive<'r>) {
		let Self { ref mut r, ref mut n, ref crop, .. } = *self;
		prim.draw(*n, (&**crop).bind(), r);
		*n += 1;
	}
	pub fn draw_with_logic<'r: 'l>(&mut self, prim: impl DrawablePrimitive<'r>, func: impl 'l + EventReaction, id: LogicId) {
		let Self { ref mut r, ref mut n, ref crop, ref mut logics } = *self;
		prim.draw(*n, (&**crop).bind(), r);
		let (id, bound, func) = (id, LogicBound::Obj(*n), Box(func));
		logics.push(LogicStorage { id, bound, func });
		*n += 1;
	}
	pub fn logic(&mut self, s: Surf, func: impl 'l + EventReaction, id: LogicId) {
		let (id, bound, func) = (id, LogicBound::Crop(s.b_box()), Box(func));
		self.logics.push(LogicStorage { id, bound, func });
	}
	pub fn hovers_in(&self, s: Surf) -> bool {
		inside(s.b_box(), self.r.mouse_pos)
	}
	pub fn hovered(&self) -> bool {
		let Self { ref r, n, .. } = *self;
		if n < 1 {
			return false;
		}
		inside(r.cache.b_box(n - 1), r.mouse_pos)
	}
	pub fn focused(&self, l: LogicId) -> bool {
		l == self.r.focus
	}
	pub fn mouse_pos(&self) -> Vec2 {
		self.r.mouse_pos
	}
	pub fn aspect(&self) -> Vec2 {
		self.r.aspect
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
	aspect: Vec2,
	focus: LogicId,
	mouse_pos: Vec2,
	theme: Theme,
	pub(super) storage: Cell<ElementStorage>,
}
impl Renderer {
	pub fn new(theme: Theme, f: &impl Frame) -> Self {
		Self { theme, aspect: f.clip_aspect(), ..Def() }
	}
	pub fn lock<'a>(self) -> RenderLock<'a> {
		RenderLock {
			r: self,
			crop: uncropped().pipe(Cell).pipe(Rc::new),
			n: 0,
			logics: vec![],
		}
	}
	fn consume_events(&mut self, mut logics: Vec<LogicStorage>, events: &mut Vec<Event>) {
		let Self { ref cache, ref mut focus, ref mut mouse_pos, .. } = *self;
		let b_box = |l: &mut LogicStorage| {
			use LogicBound::*;
			match l.bound {
				Crop(b_box) => b_box,
				Obj(at) => cache.b_box(at).tap(|b| l.bound = Crop(*b)),
			}
		};

		events.retain(|e| {
			map_variant!(&MouseMove { at, .. } = e => *mouse_pos = at);
			let (refocus, mouse) = (matches!(e, MouseButton { m, .. } if m.contains(Mod::PRESS)), *mouse_pos);

			if *focus != 0 {
				let focused = logics.iter_mut().rev().find(|l| *focus == l.id);
				if let Some(l) = focused {
					if !refocus || inside(b_box(l), mouse) {
						match (l.func)(e, true, mouse) {
							Reject => return true,
							Accept => return false,
							DropFocus => {
								*focus = 0;
								return false;
							}
							Pass => (),
						}
					} else {
						(l.func)(&Defocus, true, mouse);
						*focus = 0;
					}
				} else {
					*focus = 0;
				}
			}

			for (b, l) in logics.iter_mut().rev().map(|l| (b_box(l), l)) {
				if !inside(b, mouse) {
					continue;
				}

				if refocus
					&& *focus == 0 && let Accept = (l.func)(&OfferFocus, false, mouse)
				{
					*focus = l.id;
				}

				match (l.func)(e, *focus == l.id, mouse) {
					Reject => return true,
					Accept | DropFocus => return false,
					Pass => (),
				}
			}
			true
		});
	}
	fn render(&mut self, frame: &mut impl Frame) {
		let Self { vao, idxs, xyzw, uv, rgba, cache, flush, status, aspect, .. } = self;

		if !flush.is_empty() {
			let flush = cache.flush(frame.clip_aspect(), &mut idxs.buff, &mut xyzw.buff, &mut rgba.buff, &mut uv.buff);

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

		(*status, *flush, *aspect) = (State::XYZW.or_def(frame.clip_aspect() != *aspect), State::empty(), frame.clip_aspect());
	}
}

#[derive(Default)]
struct Storage<T: spec::Buffer, D> {
	obj: spec::ArrObj<T, D>,
	buff: Vec<D>,
	size: usize,
}
impl<T: spec::Buffer, D: Copy> Storage<T, D> {
	fn flush(&mut self, from: Option<usize>, mut rebind: impl FnMut(&spec::ArrObj<T, D>)) {
		let Some(from) = from else { return };

		let Self { ref mut obj, ref buff, ref mut size } = *self;
		let new_size = buff.len();
		if new_size <= *size && new_size * 2 > *size {
			obj.MapMut((from..new_size, gl::MAP_INVALIDATE_RANGE_BIT)).mem().copy_from_slice(&buff[from..]);
			return;
		}

		(*obj, *size) = ((buff, gl::DYNAMIC_STORAGE_BIT | gl::MAP_WRITE_BIT).pipe(spec::ArrObj::new), new_size);
		rebind(obj);
	}
}
type IdxArrStorage = Storage<spec::Index, u16>;
type ArrStorage<T> = Storage<spec::Attribute, T>;

pub struct ClipLock {
	prev_crop: Geom,
	lock: Rc<Cell<Geom>>,
}
impl Drop for ClipLock {
	fn drop(&mut self) {
		self.lock.set(self.prev_crop);
	}
}

fn inside(g: Geom, p: Vec2) -> bool {
	contains(g, (p, p))
}

fn uncropped() -> Geom {
	let L = f32::INFINITY;
	((-L, -L), (L, L))
}
