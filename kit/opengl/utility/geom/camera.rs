use crate::math::la::*;
use crate::{lib::*, math::*, GL::Frame};

#[derive(Debug, Clone)]
pub struct FocusCam {
	pub target: V3,
	proj: Memoized<M4, (uVec2, Vec2)>,
	view: Memoized<M4, (Q, V3)>,
	orient: Memoized<(Q, V3), (Vec3, V3)>,
	view_proj: Cell<Option<M4>>,
	inv_view: Cell<Option<M4>>,
}
impl FocusCam {
	pub fn pos(&self) -> Vec3 {
		Vec3(self.orient.get().1)
	}
	pub fn fov(&self) -> f32 {
		self.proj.get_args().1 .0
	}
	pub fn new(target: V3, polar_zoom: Vec3) -> Self {
		let proj = Memoized::zero(proj_f);
		let view = Memoized::zero(view_f);
		let orient = Memoized::zero(orient_f);
		let (view_proj, inv_view) = Def();

		let mut c = FocusCam { target, proj, orient, view, view_proj, inv_view };
		c.set_polar(polar_zoom);
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
		let Self { target, orient, view, view_proj, inv_view, .. } = self;

		let (c, (o, p)) = orient.apply((&polar_zoom, &*target));
		if c && view.apply((o, p)).0 {
			view_proj.replace(None);
			inv_view.replace(None);
		}
	}
	pub fn V(&self) -> &M4 {
		&self.view
	}
	pub fn iV(&self) -> &M4 {
		let Self { inv_view, .. } = self;
		if inv_view.inspect(|s| s.is_none()) {
			inv_view.replace(Some(inverse4(*self.view)));
		}

		unsafe { &*inv_view.as_ptr() }.as_ref().valid()
	}
	pub fn VP(&self) -> &M4 {
		let Self { view_proj, .. } = self;
		if view_proj.inspect(|s| s.is_none()) {
			view_proj.replace(Some(self.P() * self.V()));
		}

		unsafe { &*view_proj.as_ptr() }.as_ref().valid()
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
	pub fn iL(&self) -> Vec3 {
		Vec3(-self.orient.1)
	}
	pub fn N(&self, model: &M4) -> M3 {
		inverse3(crop_3x3(model)).transpose()
	}
	pub fn NV(&self, model: &M4) -> M3 {
		let m = self.V() * model;
		inverse3(crop_3x3(&m).transpose())
	}
}
impl Default for FocusCam {
	fn default() -> Self {
		Self::new(V3::new(0., 0., 0.), Vec3((0, -90, 1)))
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
			Ok(Self { target, proj, view, orient, ..Def() })
		}
	}
}

fn proj_f(&(size, (fov, far)): &(uVec2, Vec2)) -> M4 {
	let (w, h) = size;
	let aspect = f32(w) / f32(h);
	let fov = fov.to_radians();
	let fov = if w < h { 2. * ((fov * 0.5).tan() / aspect).atan() } else { fov };
	perspective(aspect, fov, 0.01, far)
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
