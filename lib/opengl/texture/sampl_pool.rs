use super::{sampler::*, types::*};
use crate::uses::*;

#[macro_export]
macro_rules! Sampler {
($(($n: expr, $v: expr)),+) => {{
	const _ID: u32 = sampler_use::id(&[$($n, $v),+]);
	Sampler::pooled(_ID, &[$(($n, $v)),+])
}};
}

impl Sampler {
	pub fn pooled(id: u32, args: &[(GLenum, GLenum)]) -> Rc<Sampler> {
		let p = UnsafeOnce!(HashMap<u32, Weak<Sampler>>, { HashMap::new() });

		if let Some(w) = p.get(&id) {
			if let Some(s) = w.upgrade() {
				ASSERT!(_collision_map().entry(id).or_insert(args.to_vec()).iter().eq(args.iter()), "Sampler param collision");
				return s;
			}
		}

		let mut s = Sampler::new();
		args.iter().for_each(|&(p, v)| s.Parameter(p, v));
		let s = Rc::new(s);
		p.insert(id, Rc::downgrade(&s));
		return s;
	}
	pub fn linear() -> Rc<Sampler> {
		Sampler!(
			(gl::TEXTURE_MIN_FILTER, gl::LINEAR),
			(gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE),
			(gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE),
			(gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE)
		)
	}
}

pub mod sampler_use {
	pub use super::chksum::const_fnv1_u32 as id;
}

fn _collision_map() -> &'static mut HashMap<u32, Vec<(u32, u32)>> {
	UnsafeOnce!(HashMap<u32, Vec<(u32, u32)>>, { HashMap::new() })
}
