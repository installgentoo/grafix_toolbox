use super::*;

#[macro_export]
macro_rules! Sampler {
($(($n: expr, $v: expr)),+) => {{
	use $crate::GL::{macro_uses::sampler_use::id, gl::*};
	const _ID: u32 = id(&[$($n, $v),+]);
	Sampler::pooled(_ID, &[$(($n, $v)),+])
}};
}

impl<S: AsRef<[(GLenum, GLenum)]>> From<S> for Sampler {
	fn from(args: S) -> Self {
		let mut s = Self::new();
		args.as_ref().iter().for_each(|&(p, v)| s.Parameter(p, v));
		s
	}
}
impl Sampler {
	pub fn pooled(id: u32, args: &[(GLenum, GLenum)]) -> Rc<Self> {
		let p = LocalStatic!(HashMap<u32, Weak<Sampler>>);

		if let Some(w) = p.get(&id) {
			if let Some(s) = w.upgrade() {
				let _collision_map = LocalStatic!(HashMap<u32, Vec<(u32, u32)>>);
				ASSERT!(_collision_map.entry(id).or_insert(args.to_vec()).iter().eq(args.iter()), "Sampler param collision");
				return s;
			}
		}

		let s = Rc::new(args.into());
		p.insert(id, Rc::downgrade(&s));
		s
	}
	pub fn linear() -> Rc<Self> {
		Sampler!(
			(TEXTURE_MIN_FILTER, LINEAR),
			(TEXTURE_WRAP_R, CLAMP_TO_EDGE),
			(TEXTURE_WRAP_S, CLAMP_TO_EDGE),
			(TEXTURE_WRAP_T, CLAMP_TO_EDGE)
		)
	}
}

pub mod sampler_use {
	pub use super::chksum::const_fnv1_u32 as id;
}
