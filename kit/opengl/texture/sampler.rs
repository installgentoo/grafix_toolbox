use super::*;

#[macro_export]
macro_rules! Sampler {
	($(($n: expr, $v: expr)),+) => {{
		use $crate::GL::{macro_uses::sampler_use::id, gl::*};
		let id = const { id(&[$($n, $v),+]) };
		Sampler::pooled(id, &[$(($n, $v)),+])
	}};
}

pub struct Sampler(pub(super) Rc<Obj<SamplerT>>);
impl<S: Borrow<[(GLenum, GLenum)]>> From<S> for Sampler {
	fn from(args: S) -> Self {
		Obj::new()
			.pipe(Rc::new)
			.pipe(Self)
			.tap(|s| args.borrow().iter().for_each(|&(p, v)| s.Parameter(p, v)))
	}
}
impl Sampler {
	pub fn Parameter(&mut self, name: GLenum, args: impl SamplerArg) {
		args.apply(self.0.obj, name);
	}
	pub fn pooled(id: u32, args: &[(GLenum, GLenum)]) -> Self {
		let p = LocalStatic!(HashMap<u32, rc::Weak<Obj<SamplerT>>>);

		if let Some(w) = p.get(&id)
			&& let Some(s) = w.upgrade()
		{
			ASSERT!(
				LocalStatic!(HashMap<u32, Vec<(u32, u32)>>)
					.entry(id)
					.or_insert(args.to_vec())
					.iter()
					.eq(args.iter()),
				"Sampler collision"
			);
			return Self(s);
		}

		Self::from(args).tap(|s| p.insert(id, Rc::downgrade(&s.0)).sink())
	}
	pub fn linear() -> Self {
		Sampler!(
			(TEXTURE_MIN_FILTER, LINEAR),
			(TEXTURE_MAG_FILTER, LINEAR),
			(TEXTURE_WRAP_R, CLAMP_TO_EDGE),
			(TEXTURE_WRAP_S, CLAMP_TO_EDGE),
			(TEXTURE_WRAP_T, CLAMP_TO_EDGE)
		)
	}
}

pub trait SamplerArg {
	fn apply(&self, _: u32, _: GLenum);
}
impl SamplerArg for GLenum {
	fn apply(&self, obj: u32, name: GLenum) {
		GL!(gl::SamplerParameteri(obj, name, i32(self)));
	}
}
impl SamplerArg for f32 {
	fn apply(&self, obj: u32, name: GLenum) {
		GL!(gl::SamplerParameterf(obj, name, *self));
	}
}
impl SamplerArg for Vec4 {
	fn apply(&self, obj: u32, name: GLenum) {
		let s = [*self];
		GL!(gl::SamplerParameterfv(obj, name, s.as_ptr() as *const f32));
	}
}

pub mod sampler_use {
	pub use super::chksum::const_fnv1_u32 as id;
}
