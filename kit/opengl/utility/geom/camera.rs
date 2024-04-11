use crate::math::la::*;
use crate::{lib::*, math::*, GL::Frame};

#[derive(Debug, Clone)]
pub struct FocusCam {
	pub target: V3,
	proj: Memoized<M4, (uVec2, Vec2)>,
	view: Memoized<M4, (Q, V3)>,
	orient: Memoized<(Q, V3), (Vec3, V3)>,
	view_proj: Cell<Option<M4>>,
}
impl FocusCam {
	pub fn pos(&self) -> Vec3 {
		Vec3(self.orient.get().1)
	}
	pub fn new(target: Vec3, pos: Vec3) -> Self {
		let (target, pos) = <(V3, V3)>::to((target, pos));
		let (orient, dist) = look_at(&target, &pos);
		let (a, e) = polar(&orient);
		let proj = Memoized::zero(proj_f);
		let view = Memoized::zero(view_f);
		let orient = Memoized::zero(orient_f);
		let view_proj = Def();

		let mut c = FocusCam { target, proj, orient, view, view_proj };
		c.set_polar((a, e, dist));
		c
	}
	pub fn track(&mut self, tgt: V3) {
		let Self { target, orient, view_proj, .. } = self;
		if &tgt != target {
			*target = tgt;
			orient.reset();
			view_proj.replace(None);
		}
	}
	pub fn set_proj(&mut self, f: &impl Frame, (fov, far): Vec2) {
		if self.proj.apply((f.size(), (fov, far))).0 {
			self.view_proj.replace(None);
		}
	}
	pub fn set_polar(&mut self, polar_zoom: Vec3) {
		let Self { target, orient, view, view_proj, .. } = self;

		let (c, (o, p)) = orient.apply((polar_zoom, *target));
		if c && view.apply((*o, *p)).0 {
			view_proj.replace(None);
		}
	}
	pub fn V(&self) -> &M4 {
		&self.view
	}
	pub fn VP(&self) -> &M4 {
		if unsafe { &*self.view_proj.as_ptr() }.as_ref().is_none() {
			self.view_proj.replace(Some(self.P() * self.V()));
		}
		unsafe { &*self.view_proj.as_ptr() }.as_ref().valid()
	}
	pub fn P(&self) -> &M4 {
		&self.proj
	}
	pub fn MV(&self, model: &M4) -> M4 {
		self.V() * model
	}
	pub fn MVP(&self, model: &M4) -> M4 {
		self.VP() * model
	}
	pub fn N(&self, model: &M4) -> M3 {
		la::inverse3(la::crop_3x3(model)).transpose()
	}
	pub fn NV(&self, model: &M4) -> M3 {
		let m = self.V() * model;
		la::inverse3(la::crop_3x3(&m).transpose())
	}
}
impl Default for FocusCam {
	fn default() -> Self {
		Self::new(Vec3((0, 0, 0)), Vec3((0, 0, 1)))
	}
}
#[cfg(feature = "adv_fs")]
mod serde {
	use {super::*, crate::ser::*};

	impl Serialize for FocusCam {
		fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
			let Self { target, proj, view, orient, .. } = self;
			(target, proj, view, orient).serialize(s)
		}
	}
	impl<'de> Deserialize<'de> for FocusCam {
		fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
			let (target, proj, view, orient) = <(V3, Memoized<M4, (uVec2, Vec2)>, Memoized<M4, (Q, V3)>, Memoized<(Q, V3), (Vec3, V3)>)>::deserialize(d)?;
			let proj = proj.finalize_deserialization(proj_f);
			let view = view.finalize_deserialization(view_f);
			let orient = orient.finalize_deserialization(orient_f);
			Ok(Self { target, proj, view, orient, view_proj: Def() })
		}
	}
}

fn proj_f(&(size, (fov, far)): &(uVec2, Vec2)) -> M4 {
	let (w, h) = size;
	let aspect = f32(w) / f32(h);
	let fov = fov.to_radians();
	let fov = if w < h { 2. * ((fov * 0.5).tan() / aspect).atan() } else { fov };
	la::perspective(aspect, fov, 0.01, far)
}
fn view_f((orient, pos): &(Q, V3)) -> M4 {
	let rot = orient.inverse().to_rotation_matrix().to_homogeneous();
	let trans = M4::new_translation(&-pos);
	rot * trans
}
fn orient_f(&((a, e, dist), target): &(Vec3, V3)) -> (Q, V3) {
	let (a, e) = (a, e).map(|c| c.to_radians());

	let mut orient = Q::identity();
	let yaw = Q::from_axis_angle(&V3::y_axis(), a);
	orient = yaw * orient;

	let pitch = Q::from_axis_angle(&V3::x_axis(), e);
	orient *= pitch;

	let dir = orient * -V3::z_axis();
	let pos = target - *dir * dist;
	(orient, pos)
}

fn look_at(target: &V3, pos: &V3) -> (Q, f32) {
	let fwd = pos - target;
	let dist = fwd.magnitude();
	let fwd = fwd.normalize();
	let up = V3::y_axis();
	let right = up.cross(&fwd).normalize();
	let up = fwd.cross(&right);
	let orient = Q::face_towards(&fwd, &up);
	(orient, dist)
}
fn polar(orient: &Q) -> Vec2 {
	let local_fwd = -V3::z_axis();
	let world_fwd = orient * local_fwd;
	let a = world_fwd.z.atan2(world_fwd.x);
	let e = world_fwd.y.atan2((world_fwd.x.powi(2) + world_fwd.z.powi(2)).sqrt());
	(a, e)
}
