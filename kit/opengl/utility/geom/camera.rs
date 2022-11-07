use super::args::*;
use crate::uses::*;

#[derive(Default)]
pub struct Camera {
	proj: M4,
	view: M4,
	view_proj: M4,
}
impl Camera {
	pub fn new(cargs: impl CameraArgs, pargs: impl PosArgs) -> Self {
		let p = cargs.get();
		let v = pargs.getp();
		Self {
			proj: p,
			view: v,
			view_proj: p * v,
		}
	}
	pub fn zero(cargs: impl CameraArgs) -> Self {
		Self {
			proj: cargs.get(),
			view: M4::identity(),
			view_proj: M4::identity(),
		}
	}
	pub fn setProj(&mut self, p: impl CameraArgs) {
		let p = p.get();
		self.proj = p;
		self.view_proj = p * self.view;
	}
	pub fn setView(&mut self, v: impl PosArgs) {
		let v = v.getp();
		self.view = v;
		self.view_proj = self.proj * v;
	}
	pub fn V(&self) -> Mat4 {
		Mat4(self.view)
	}
	pub fn VP(&self) -> Mat4 {
		Mat4(self.view_proj)
	}
	pub fn MV(&self, model: &M4) -> Mat4 {
		Mat4(self.view * model)
	}
	pub fn MVP(&self, model: &M4) -> Mat4 {
		Mat4(self.view_proj * model)
	}
	pub fn N(&self, model: &M4) -> Mat3 {
		Mat3(glm::inverse_transpose(model.fixed_resize(0.)))
	}
	pub fn NV(&self, model: &M4) -> Mat3 {
		Mat3(glm::inverse_transpose((self.view * model).fixed_resize(0.)))
	}
}
use glm::Mat4 as M4;
