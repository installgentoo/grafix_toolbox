use super::args::*;
use crate::lib::*;

#[derive(Default, Debug, Clone)]
pub struct Camera {
	proj: M4,
	view: M4,
	view_proj: M4,
}
impl Camera {
	pub fn new(pargs: impl ProjArgs, cargs: impl CamArgs) -> Self {
		let proj = pargs.getp();
		let view = cargs.getc();
		Self { proj, view, view_proj: proj * view }
	}
	pub fn zero(pargs: impl ProjArgs) -> Self {
		Self {
			proj: pargs.getp(),
			view: M4::identity(),
			view_proj: M4::identity(),
		}
	}
	pub fn set_proj(&mut self, p: impl ProjArgs) {
		let p = p.getp();
		self.proj = p;
		self.view_proj = p * self.view;
	}
	pub fn set_view(&mut self, v: impl CamArgs) {
		let v = v.getc();
		self.view = v;
		self.view_proj = self.proj * v;
	}
	pub fn V(&self) -> &M4 {
		&self.view
	}
	pub fn P(&self) -> &M4 {
		&self.proj
	}
	pub fn VP(&self) -> &M4 {
		&self.view_proj
	}
	pub fn MV(&self, model: &M4) -> M4 {
		self.view * model
	}
	pub fn MVP(&self, model: &M4) -> M4 {
		self.view_proj * model
	}
	pub fn N(&self, model: &M4) -> M3 {
		la::inverse3(la::crop_3x3(model)).transpose()
	}
	pub fn NV(&self, model: &M4) -> M3 {
		let m = self.view * model;
		la::inverse3(la::crop_3x3(&m).transpose())
	}
	pub fn view(args: impl CamArgs) -> M4 {
		args.getc()
	}
}
use {la::Mat3 as M3, la::Mat4 as M4};
