use super::{object::*, policy::*, types::*};
use crate::uses::*;

pub type Sampler = Object<SamplObj>;

impl Sampler {
	pub fn Parameter(&mut self, name: GLenum, args: impl SamplerArg) {
		args.apply(self.obj, name);
	}
}

pub trait SamplerArg {
	fn apply(&self, _: u32, _: GLenum);
}
impl SamplerArg for GLenum {
	fn apply(&self, obj: u32, name: GLenum) {
		GLCheck!(gl::SamplerParameteri(obj, name, i32(*self)));
	}
}
impl SamplerArg for f32 {
	fn apply(&self, obj: u32, name: GLenum) {
		GLCheck!(gl::SamplerParameterf(obj, name, *self));
	}
}
impl SamplerArg for Vec4 {
	fn apply(&self, obj: u32, name: GLenum) {
		let s = [*self];
		GLCheck!(gl::SamplerParameterfv(obj, name, s.as_ptr() as *const f32));
	}
}
