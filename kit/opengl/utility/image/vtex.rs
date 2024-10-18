use super::*;
use crate::{lazy::*, prefetch::*};

#[derive(Debug)]
pub struct VTex2d<S, F> {
	pub region: Vec4,
	pub atlas: Rc<Tex2d<S, F>>,
}
impl<S, F> VTex2d<S, F> {
	pub fn eq_atlas(&self, r: &Self) -> bool {
		Rc::ptr_eq(&self.atlas, &r.atlas)
	}
}
impl<S, F> PartialEq for VTex2d<S, F> {
	fn eq(&self, r: &Self) -> bool {
		self.eq_atlas(r) && self.region == r.region
	}
}
impl<S, F> Eq for VTex2d<S, F> {}
impl<S, F> Clone for VTex2d<S, F> {
	fn clone(&self) -> Self {
		let &Self { region, ref atlas } = self;
		let (region, atlas) = (region, atlas.clone());
		Self { region, atlas }
	}
}

pub type VTex2dEntry<'a, S> = Prefetched<'a, u32, VTex2d<S, u8>, TexAtlas<S>>;

#[derive(Default)]
pub struct TexAtlas<S>(Cell<State<S>>);
impl<S: TexSize> TexAtlas<S> {
	pub fn new() -> Self {
		Def()
	}
	pub fn load(&self, name: &str) -> VTex2dEntry<S> {
		match unsafe { &mut *self.0.as_ptr() } {
			Baked(_) => ERROR!("Trying to load into finalized atals"),
			Fresh(reqs) => {
				let k = u32(reqs.len());
				let name: Astr = format!("res/{name}").into();
				reqs.push((name.clone(), Lazy::new(FS::Lazy::File(name))));
				Prefetched::new(k, self)
			}
		}
	}
	fn finalise(state: &mut State<S>) -> &mut VTexMap<S> {
		let t = match mem::take(state) {
			Baked(_) => unreachable!(),
			Fresh(mut reqs) => {
				let reqs = reqs
					.iter_mut()
					.enumerate()
					.map(|(n, (name, r))| (u32(n), uImage::<S>::load(&r.get()[..]).explain_err(|| format!("Cannot atlas image {name:?}")).warn()))
					.collect_vec();
				let max_side = GL::MAX_TEXTURE_SIZE();
				let (atlas, mut tail) = atlas::pack_into_atlas(reqs, max_side, max_side);
				let mut textures: VTexMap<S> = atlas.into_iter().collect();
				while !tail.is_empty() {
					let last_l = tail.len();
					let (a, t) = atlas::pack_into_atlas(tail, max_side, max_side);
					if last_l == t.len() {
						ERROR!("Graphics card can't fit textures: {t:?}");
					}
					textures.extend(a.into_iter());
					tail = t;
				}
				textures
			}
		};

		*state = Baked(t);
		match state {
			Fresh(_) => unreachable!(),
			Baked(t) => t,
		}
	}
}

impl<S: TexSize> Fetcher<u32, VTex2d<S, u8>> for TexAtlas<S> {
	fn get(&self, k: u32) -> &VTex2d<S, u8> {
		let s = unsafe { &mut *self.0.as_ptr() };
		let textures = match s {
			Fresh(_) => Self::finalise(s),
			Baked(t) => t,
		};
		textures.get(&k).valid()
	}
	fn take(&self, k: u32) -> VTex2d<S, u8> {
		let s = unsafe { &mut *self.0.as_ptr() };
		let textures = match s {
			Fresh(_) => Self::finalise(s),
			Baked(t) => t,
		};
		textures.remove(&k).take().valid()
	}
}

enum State<S> {
	Fresh(Vec<(Astr, Lazy<Vec<u8>>)>),
	Baked(VTexMap<S>),
}
impl<S> Default for State<S> {
	fn default() -> Self {
		Fresh(Def())
	}
}
use State::*;
type VTexMap<S> = HashMap<u32, VTex2d<S, u8>>;
