use super::{batch::*, parts::*};
use crate::uses::{GL::tex::*, GL::VaoBinding, *};

#[derive(Default)]
pub struct Objects {
	pub batches: Vec<Batch>,
	pub objs: Vec<Primitive>,
	pub first_transparent: usize,
}
impl Objects {
	pub fn get(&mut self, at: u32) -> &mut Primitive {
		unsafe { self.objs.get_unchecked_mut(at as usize) }
	}
	pub fn shrink(&mut self, to: u32) {
		let Self { batches, objs, .. } = self;
		batches.retain_mut(|b| !b.shrink_and_empty(objs, to));
		objs.truncate(to as usize);
	}
	pub fn batch(&mut self) {
		let Self { batches, objs, first_transparent } = self;

		let overlaps = |o, z| {
			batches.iter().position(|b| b.contains(objs, (o, z))).map_or(false, |b| {
				let covered = || batches[..b].iter().find(|b| b.covered(objs, (o, z))).is_some();
				let covers = || batches[b + 1..].iter().find(|b| b.covers(objs, (o, z))).is_some();
				covered() || covers()
			})
		};

		if let Some(first_invalid) = objs
			.iter()
			.enumerate()
			.find(|(n, Primitive { state, o, .. })| {
				let overlapping = || state.contains(State::XYZW) && o.obj().ordered() && overlaps(o, u32::to(*n));
				state.contains(State::TYPE) || overlapping()
			})
			.map(|(n, _)| n)
		{
			batches.retain_mut(|b| !b.shrink_and_empty(objs, u32::to(first_invalid)));

			objs.iter().enumerate().skip(first_invalid).for_each(|(z, o)| {
				let (z, o) = (u32::to(z), &o.o);
				for b in batches.iter_mut().rev() {
					if b.try_add(objs, (o, z)) {
						return;
					}

					if b.interferes(objs, o) {
						break;
					}
				}

				if o.obj().ordered() {
					batches.push(Batch::new(z));
				} else {
					batches.insert(0, Batch::new(z));
				}
			});

			*first_transparent = batches.iter().position(|b| b.typ(&objs).obj().ordered()).unwrap_or(batches.len());
		}
	}
	pub fn flush(&mut self, idxs: &mut Vec<u16>, xyzw: &mut Vec<f16>, rgba: &mut Vec<u8>, uv: &mut Vec<f16>) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
		let Self { batches, objs, .. } = self;

		batches
			.iter_mut()
			.fold(((None, None, None, None), State::empty(), 0, 0), |(mut flush, mut state, idx_start, batch_start), b| {
				let (batch_size, s) = b.redraw(objs);
				state |= s;

				if state.contains(State::RESIZED) {
					flush.0.get_or_insert(idx_start as usize);
					let mut indices = b.typ(objs).obj().gen_idxs(vec2::to((batch_start, batch_size)));
					b.idx_range = (idx_start, u32::to(indices.len()));
					idxs.truncate(idx_start as usize);
					idxs.append(&mut indices);
				}

				fn update<T: Copy>(changed: bool, ordered: bool, dim: usize, v: &mut Vec<T>, at: usize, upd: &[T], flush: &mut Option<usize>) {
					if changed {
						flush.get_or_insert(at * dim);
						v.truncate(at * dim);
						if ordered {
							v.extend_from_slice(upd);
						} else {
							v.extend_from_slice(&upd.chunks(dim).rev().flatten().cloned().collect::<Vec<_>>());
						}
					}
				}

				let ordered = b.typ(objs).obj().ordered();
				update(state.contains(State::XYZW), ordered, 4, xyzw, batch_start, &b.xyzw, &mut flush.1);
				update(state.contains(State::RGBA), ordered, 4, rgba, batch_start, &b.rgba, &mut flush.2);
				update(state.contains(State::UV), ordered, 2, uv, batch_start, &b.uv, &mut flush.3);

				(flush, state, b.idx_range.0 + b.idx_range.1, batch_start + batch_size as usize)
			})
			.0
	}
	pub fn draw_opaque_batches(&self, v: &VaoBinding<u16>) {
		let Self { batches, objs, first_transparent } = self;
		batches
			.iter()
			.take(*first_transparent)
			.for_each(|b| b.typ(&objs).obj().batch_draw(&v, vec2::to(b.idx_range)));
	}
	pub fn draw_transparent_batches(&self, v: &VaoBinding<u16>) {
		let Self { batches, objs, first_transparent } = self;
		batches
			.iter()
			.skip(*first_transparent)
			.for_each(|b| b.typ(&objs).obj().batch_draw(&v, vec2::to(b.idx_range)));
	}
}

pub struct Primitive {
	pub state: State,
	pub o: ObjStore,
}
pub enum ObjStore {
	DrawRect(RectImpl),
	DrawImgRGB(SpriteImpl<RGB>),
	DrawImgRGBA(SpriteImpl<RGBA>),
	DrawImg9RGB(Sprite9Impl<RGB>),
	DrawImg9RGBA(Sprite9Impl<RGBA>),
	DrawFrame9(Frame9Impl),
	DrawText(TextImpl),
}
impl ObjStore {
	#[inline(always)]
	pub fn obj(&self) -> &dyn Object {
		match self {
			DrawRect(r) => r,
			DrawImgRGB(r) => r,
			DrawImgRGBA(r) => r,
			DrawImg9RGB(r) => r,
			DrawImg9RGBA(r) => r,
			DrawFrame9(r) => r,
			DrawText(r) => r,
		}
	}
	pub fn batchable(&self, r: &ObjStore) -> bool {
		match (self, r) {
			(DrawRect(l), DrawRect(r)) => l.batchable(r),
			(DrawImgRGB(l), DrawImgRGB(r)) => l.batchable(r),
			(DrawImg9RGB(l), DrawImg9RGB(r)) => l.batchable(r),
			(DrawImgRGBA(l), DrawImgRGBA(r)) => l.batchable(r),
			(DrawImg9RGBA(l), DrawImg9RGBA(r)) => l.batchable(r),
			(DrawFrame9(l), DrawFrame9(r)) => l.batchable(r),
			(DrawText(l), DrawText(r)) => l.batchable(r),
			_ => return false,
		}
	}
}
use ObjStore::*;
