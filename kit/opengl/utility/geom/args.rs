use crate::uses::{GL::Frame, *};

pub trait CameraArgs {
	fn get(self) -> M4;
}
impl CameraArgs for M4 {
	fn get(self) -> M4 {
		self
	}
}
impl<F: Frame, L> CameraArgs for (&F, L)
where
	Vec2: Cast<L>,
{
	fn get(self) -> M4 {
		let ((x, y), (fov, far)) = (self.0.aspect(), Vec2(self.1));
		glm::perspective(y / x, fov.to_radians(), 0.01, far)
	}
}

pub trait PosArgs {
	fn getp(self) -> M4;
}
impl PosArgs for M4 {
	fn getp(self) -> M4 {
		self
	}
}
impl<AT, FROM, UP> PosArgs for (AT, FROM, UP)
where
	vec3<Vec3>: Cast<(AT, FROM, UP)>,
{
	fn getp(self) -> M4 {
		let (at, from, up) = vec3::<V3>::to(vec3::<Vec3>::to(self));
		glm::look_at(&at, &from, &up)
	}
}
impl<AT, FROM> PosArgs for (AT, FROM)
where
	Vec3: Cast<AT>,
	Vec3: Cast<FROM>,
{
	fn getp(self) -> M4 {
		(self.0, self.1, (0., 1., 0.)).getp()
	}
}
use glm::{Mat4 as M4, Vec3 as V3};
