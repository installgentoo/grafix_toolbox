use crate::{lib::*, GL::Frame};

pub trait ProjArgs {
	fn getp(self) -> M4;
}
impl ProjArgs for M4 {
	fn getp(self) -> M4 {
		self
	}
}
impl<F: Frame, L> ProjArgs for (&F, L)
where
	Vec2: Cast<L>,
{
	fn getp(self) -> M4 {
		let (w, h) = self.0.size();
		let aspect = f32(w) / f32(h);
		let (fov, far) = Vec2(self.1);
		let fov = fov.to_radians();
		let fov = if w < h { 2. * ((fov * 0.5).tan() / aspect).atan() } else { fov };
		la::perspective(aspect, fov, 0.01, far)
	}
}

pub trait CamArgs {
	fn getc(self) -> M4;
}
impl CamArgs for M4 {
	fn getc(self) -> M4 {
		self
	}
}
impl<AT, FROM, UP> CamArgs for (AT, FROM, UP)
where
	vec3<Vec3>: Cast<(AT, FROM, UP)>,
{
	fn getc(self) -> M4 {
		let (at, from, up) = vec3::<V3>::to(vec3::<Vec3>::to(self));
		la::look_at(from, at, up)
	}
}
impl<AT, FROM> CamArgs for (AT, FROM)
where
	Vec3: Cast<AT>,
	Vec3: Cast<FROM>,
{
	fn getc(self) -> M4 {
		(self.0, self.1, (0., 1., 0.)).getc()
	}
}
use la::{Mat4 as M4, Vec3 as V3};
