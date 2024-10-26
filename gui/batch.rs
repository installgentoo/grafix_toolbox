use super::*;

struct Obj {
	idx: u32,
	size: u32,
}
#[derive(Default)]
pub struct Batch {
	pub idx_range: usVec2,
	pub xyzw: Vec<f16>,
	pub uv: Vec<f16>,
	pub rgba: Vec<u8>,
	objs: Vec<Obj>,
}
impl Batch {
	pub fn new(z: u32) -> Self {
		Self { objs: vec![Obj { idx: z, size: 0 }], ..Def() }
	}
	pub fn typ<'a>(&self, cache: &'a [PrimCache]) -> &'a PrimImpl {
		get(cache, self.objs.first().valid())
	}
	pub fn contains(&self, cache: &[PrimCache], (o, z): (&PrimImpl, u32)) -> bool {
		let (t, objs) = (self.typ(cache), &self.objs);
		t.batchable(o) && objs.binary_search_by(|o| o.idx.cmp(&z)).is_ok()
	}
	pub fn covers(&self, cache: &[PrimCache], (o, z): (&PrimImpl, u32)) -> bool {
		let (t, objs, obj) = (self.typ(cache), self.objs.iter(), o.obj());

		!(t.batchable(o)
			|| !t.obj().ordered()
			|| !objs.take_while(|i| i.idx <= z).any(|i| {
				let l = get(cache, i).obj().base();
				l.intersects(obj.base())
			}))
	}
	pub fn covered(&self, cache: &[PrimCache], (o, z): (&PrimImpl, u32)) -> bool {
		let (t, objs, obj) = (self.typ(cache), self.objs.iter(), o.obj());
		!(t.batchable(o)
			|| !t.obj().ordered()
			|| !objs.rev().take_while(|i| i.idx >= z).any(|i| {
				let l = get(cache, i).obj().base();
				l.intersects(obj.base())
			}))
	}
	pub fn shrink_and_empty(&mut self, cache: &mut [PrimCache], z: u32) -> bool {
		let objs = &mut self.objs;
		let l = objs.iter().rposition(|i| i.idx < z).map(|i| i + 1).unwrap_or(0);
		if !objs[l..].is_empty() {
			objs.drain(l..).for_each(|o| cache.at_mut(o.idx).state = State::MISMATCH);
			objs.first().map(|o| cache.at_mut(o.idx).state |= State::BATCH_RESIZED);
		}
		objs.is_empty()
	}
	pub fn try_add(&mut self, cache: &[PrimCache], (o, z): (&PrimImpl, u32)) -> bool {
		if !self.typ(cache).batchable(o) {
			return false;
		}

		self.objs.push(Obj { idx: z, size: 0 });
		true
	}
	pub fn interferes(&self, cache: &[PrimCache], o: &PrimImpl) -> bool {
		let (t, mut objs, obj) = (self.typ(cache).obj(), self.objs.iter(), o.obj());
		t.ordered()
			&& obj.ordered()
			&& objs.any(|i| {
				let l = get(cache, i).obj().base();
				l.intersects(obj.base())
			})
	}
	pub fn redraw(&mut self, aspect: Vec2, cache: &[PrimCache]) -> (u32, State) {
		let Self { xyzw, rgba, uv, objs, .. } = self;

		let (len, mut state) = objs.iter_mut().fold((0, State::empty()), |(start, flush), Obj { idx, size }| {
			let &PrimCache { state, ref o } = cache.at(*idx);
			let (obj, idx) = (o.obj(), *idx);

			let (new_size, state) = (|| {
				if (state ^ State::BATCH_RESIZED).is_empty() {
					return (*size, state);
				}

				let new_size = obj.vert_count();

				let state = (|| {
					if state.contains(State::MISMATCH) {
						let to = usize(start + new_size);
						xyzw.resize(to * 4, f16(0));
						rgba.resize(to * 4, 0);
						uv.resize(to * 2, f16(0));
						return State::MISMATCH;
					}

					if new_size > *size {
						let O = f16::ZERO;
						let (at, s) = ulVec2((start, new_size - *size));
						xyzw.splice(at * 4..at * 4, vec![O; s * 4]);
						rgba.splice(at * 4..at * 4, vec![0; s * 4]);
						uv.splice(at * 2..at * 2, vec![O; s * 2]);
						return State::FULL | State::BATCH_RESIZED;
					}

					if new_size < *size {
						let (from, to) = ulVec2((start + new_size, start + *size));
						xyzw.drain(from * 4..to * 4);
						rgba.drain(from * 4..to * 4);
						uv.drain(from * 2..to * 2);
						return State::FULL | State::BATCH_RESIZED;
					}

					state
				})();

				let (z, s) = <(f32, usize)>::to((idx, start));
				obj.write_mesh(
					aspect,
					BatchedObj {
						z: f16(1. - z / 1000.),
						state,
						xyzw: &mut xyzw[s * 4..],
						rgba: &mut rgba[s * 4..],
						uv: &mut uv[s * 2..],
					},
				);

				(new_size, state)
			})();

			*size = new_size;

			(start + new_size, flush | state)
		});

		if state.contains(State::BATCH_RESIZED) {
			let to = usize(len);
			xyzw.truncate(to * 4);
			rgba.truncate(to * 4);
			uv.truncate(to * 2);
			state |= State::FULL;
		}
		(len, state)
	}
}
fn get<'a>(cache: &'a [PrimCache], o: &Obj) -> &'a PrimImpl {
	&cache.at(o.idx).o
}
