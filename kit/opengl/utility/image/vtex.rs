use super::*;
use crate::lazy::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VTex2d<S, F> {
	pub region: Vec4,
	pub atlas: Rc<Tex2d<S, F>>,
}
impl<S, F> VTex2d<S, F> {
	pub fn eq_atlas(&self, r: &Self) -> bool {
		Rc::ptr_eq(&self.atlas, &r.atlas)
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
		let reqs = unsafe { &mut *self.0.as_ptr() };
		let Fresh(reqs) = reqs else { ERROR!("Trying to load into finalized atals") };

		let k = u32(reqs.len());
		let name = format!("res/{name}").pipe(Astr::from);
		reqs.push((name.clone(), FS::Lazy::File(name).pipe(Feed::new)));
		Prefetched::new(k, self)
	}
	fn finalise(state: &mut State<S>) -> &mut VTexMap<S> {
		let Fresh(reqs) = mem::take(state) else { unreachable!() };
		let mut tail = reqs
			.into_iter()
			.enumerate()
			.map(|(n, (name, r))| (u32(n), r.take().pipe_as(uImage::<S>::load).explain_err(|| format!("Cannot atlas image {name:?}")).warn()))
			.collect_vec();

		let (max_side, mut textures) = (GL::MAX_TEXTURE_SIZE(), VTexMap::new());

		while !tail.is_empty() {
			let last_l = tail.len();
			let (a, t) = atlas::pack_into_atlas(tail, max_side, max_side);
			if last_l == t.len() {
				ERROR!("GPU cannot fit texture {t:?}");
			}
			textures.extend(a.into_iter());
			tail = t;
		}

		*state = Baked(textures);
		let Baked(t) = state else { unreachable!() };
		t
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
		textures.remove(&k).valid()
	}
}

enum State<S> {
	Fresh(Vec<(Astr, Feed<Vec<u8>>)>),
	Baked(VTexMap<S>),
}
impl<S> Default for State<S> {
	fn default() -> Self {
		Fresh(Def())
	}
}
use State::*;
type VTexMap<S> = HashMap<u32, VTex2d<S, u8>>;
