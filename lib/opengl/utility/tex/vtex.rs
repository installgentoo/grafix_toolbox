use crate::uses::{prefetch::*, *};
use crate::GL::{atlas, tex::*, uImage};

#[derive(Debug)]
pub struct VTex2d<S, F> {
	pub region: Vec4,
	pub tex: Rc<Tex2d<S, F>>,
}
impl<S, F> PartialEq for VTex2d<S, F> {
	fn eq(&self, r: &Self) -> bool {
		let Self { region, tex } = self;
		Rc::ptr_eq(tex, &r.tex) && *region == r.region
	}
}
impl<S, F> Eq for VTex2d<S, F> {}
impl<S, F> Clone for VTex2d<S, F> {
	fn clone(&self) -> Self {
		let VTex2d { region, tex } = self;
		let (region, tex) = (*region, tex.clone());
		VTex2d { region, tex }
	}
}

pub type AtlasTex2d<'a, S> = Prefetched<'a, u32, VTex2d<S, u8>, TexAtlas<S>>;

#[derive(Default)]
pub struct TexAtlas<S> {
	t: UnsafeCell<(Vec<FS::File::Resource>, HashMap<u32, VTex2d<S, u8>>)>,
}
impl<S: TexSize> TexAtlas<S> {
	pub fn new() -> Self {
		Self::default()
	}
	pub fn load(&self, name: &str) -> AtlasTex2d<S> {
		let (reqs, _textures) = unsafe { &mut *self.t.get() };
		ASSERT!(_textures.is_empty(), "Loading into atlas after batching");
		let k = u32::to(reqs.len());
		reqs.push(FS::Preload::File(CONCAT!("res/", name)));
		Prefetched::new(k, self)
	}
	fn initialize(&self) {
		let (reqs, textures) = unsafe { &mut *self.t.get() };
		let reqs: Vec<(u32, _)> = reqs.into_iter().enumerate().map(|(n, r)| (u32::to(n), EXPECT!(uImage::<S>::new(r.get())))).collect();
		let max_side = GL::MAX_TEXTURE_SIZE();
		let (mut atlas, mut tail) = atlas::pack_into_atlas::<_, _, S, _>(reqs, max_side, max_side);
		if tail.is_empty() {
			textures.extend(atlas.into_iter());
		} else {
			let mut last_s = tail.len();
			while !tail.is_empty() {
				textures.extend(atlas.into_iter());
				let (a, l) = atlas::pack_into_atlas::<_, _, S, _>(tail, max_side, max_side);
				atlas = a;
				tail = l;
				if tail.len() == last_s {
					ERROR!("Graphics card can't fit textures: {:?}", tail);
				}
				last_s = tail.len();
			}
		}
	}
}
impl<S: TexSize> Fetcher<u32, VTex2d<S, u8>> for TexAtlas<S> {
	fn get(&self, k: u32) -> &VTex2d<S, u8> {
		let (_, textures) = unsafe { &mut *self.t.get() };
		if textures.is_empty() {
			self.initialize();
		}
		textures.get(&k).unwrap()
	}
	fn take(&self, k: u32) -> VTex2d<S, u8> {
		let (_, textures) = unsafe { &mut *self.t.get() };
		if textures.is_empty() {
			self.initialize();
		}
		textures.remove(&k).take().unwrap()
	}
}
