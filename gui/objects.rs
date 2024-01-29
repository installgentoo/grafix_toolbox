use super::{batch::*, parts::*};
use crate::{lib::*, math::*, GL::tex::*, GL::VaoBinding};

#[derive(Default)]
pub struct Objects {
	pub batches: Vec<Batch>,
	pub objs: Vec<Primitive>,
	pub first_transparent: usize,
}
impl Objects {
	pub fn get(&self, at: u32) -> &Primitive {
		self.objs.at(at)
	}
	pub fn get_mut(&mut self, at: u32) -> &mut Primitive {
		self.objs.at_mut(at)
	}
	pub fn shrink(&mut self, to: u32) {
		let Self { batches, objs, .. } = self;
		batches.retain_mut(|b| !b.shrink_and_empty(objs, to));
		objs.truncate(usize(to));
	}
	pub fn batch(&mut self) {
		let Self { batches, objs, first_transparent } = self;

		let overlaps = |o, z| {
			batches.iter().position(|b| b.contains(objs, (o, z))).map_or(false, |b| {
				let covered = || batches[..b].iter().any(|b| b.covered(objs, (o, z)));
				let covers = || batches[b + 1..].iter().any(|b| b.covers(objs, (o, z)));
				covered() || covers()
			})
		};

		if let Some(first_invalid) = objs
			.iter()
			.enumerate()
			.find(|(n, Primitive { state, o, .. })| {
				let overlapping = || state.contains(State::XYZW) && o.obj().ordered() && overlaps(o, u32(*n));
				state.contains(State::MISMATCH) || overlapping()
			})
			.map(|(n, _)| n)
		{
			batches.retain_mut(|b| !b.shrink_and_empty(objs, u32(first_invalid)));

			objs.iter().enumerate().skip(first_invalid).for_each(|(z, o)| {
				let (z, o) = (u32(z), &o.o);
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

			*first_transparent = batches.iter().position(|b| b.typ(objs).obj().ordered()).unwrap_or(batches.len());
		}
	}
	pub fn flush(
		&mut self, aspect: Vec2, idxs: &mut Vec<u16>, xyzw: &mut Vec<f16>, rgba: &mut Vec<u8>, uv: &mut Vec<f16>,
	) -> (Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
		let Self { batches, objs, .. } = self;
		const MAXIDX: usize = u16::MAX as usize;

		batches
			.iter_mut()
			.fold(((None, None, None, None), State::empty(), 0, 0), |(mut flush, mut state, idx_start, batch_start), b| {
				let (batch_size, s) = b.redraw(aspect, objs);
				state |= s;

				if state.contains(State::BATCH_RESIZED) {
					flush.0.get_or_insert(idx_start);
					let mut indices = b.typ(objs).obj().gen_idxs(usVec2((batch_start, batch_size)));
					if idx_start + indices.len() > MAXIDX {
						WARN!("GUI batch too saturated with polygons, dropping some");
						let mut i = indices.into_vec();
						i.truncate(MAXIDX - idx_start);
						indices = i.into();
					}
					b.idx_range = usVec2((idx_start, indices.len()));
					idxs.truncate(usize(idx_start));
					idxs.extend_from_slice(&indices);
				}

				fn update<T: Copy>(changed: bool, ordered: bool, dim: usize, v: &mut Vec<T>, at: usize, upd: &[T], flush: &mut Option<usize>) {
					if changed {
						flush.get_or_insert(at * dim);
						v.truncate(at * dim);
						if ordered {
							v.extend_from_slice(upd);
						} else {
							v.extend(upd.chunks(dim).rev().flatten());
						}
					}
				}

				let ordered = b.typ(objs).obj().ordered();
				update(state.contains(State::XYZW), ordered, 4, xyzw, batch_start, &b.xyzw, &mut flush.1);
				update(state.contains(State::RGBA), ordered, 4, rgba, batch_start, &b.rgba, &mut flush.2);
				update(state.contains(State::UV), ordered, 2, uv, batch_start, &b.uv, &mut flush.3);

				(flush, state, ulVec2(b.idx_range).fold(|l, r| l + r).min(MAXIDX), batch_start + usize(batch_size))
			})
			.0
	}
	pub fn draw_opaque_batches(&self, v: &VaoBinding<u16>) {
		let Self { batches, objs, first_transparent } = self;
		batches.iter().take(*first_transparent).for_each(|b| b.typ(objs).obj().batch_draw(v, b.idx_range));
	}
	pub fn draw_transparent_batches(&self, v: &VaoBinding<u16>) {
		let Self { batches, objs, first_transparent } = self;
		batches.iter().skip(*first_transparent).for_each(|b| b.typ(objs).obj().batch_draw(v, b.idx_range));
	}
}

pub struct Primitive {
	pub state: State,
	pub o: ObjStore,
}
pub enum ObjStore {
	Rect(RectImpl),
	ImgRGB(SpriteImpl<RGB>),
	ImgRGBA(SpriteImpl<RGBA>),
	Img9RGB(Sprite9Impl<RGB>),
	Img9RGBA(Sprite9Impl<RGBA>),
	Frame9(Frame9Impl),
	Text(TextImpl),
}
impl ObjStore {
	#[inline(always)]
	pub fn obj(&self) -> &dyn Object {
		match self {
			Rect(r) => r,
			ImgRGB(r) => r,
			ImgRGBA(r) => r,
			Img9RGB(r) => r,
			Img9RGBA(r) => r,
			Frame9(r) => r,
			Text(r) => r,
		}
	}
	pub fn batchable(&self, r: &Self) -> bool {
		match (self, r) {
			(Rect(l), Rect(r)) => l.batchable(r),
			(ImgRGB(l), ImgRGB(r)) => l.batchable(r),
			(Img9RGB(l), Img9RGB(r)) => l.batchable(r),
			(ImgRGBA(l), ImgRGBA(r)) => l.batchable(r),
			(Img9RGBA(l), Img9RGBA(r)) => l.batchable(r),
			(Frame9(l), Frame9(r)) => l.batchable(r),
			(Text(l), Text(r)) => l.batchable(r),
			_ => false,
		}
	}
}
use ObjStore::*;
