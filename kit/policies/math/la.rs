pub use nalgebra as na; //TODO generalize and unify

pub type Vec2 = na::SVector<f32, 2>;
pub type Vec3 = na::SVector<f32, 3>;
pub type Vec4 = na::SVector<f32, 4>;
pub type Mat3 = na::Matrix3<f32>;
pub type Mat4 = na::Matrix4<f32>;

pub fn identity() -> Mat4 {
	Mat4::identity()
}
pub fn inverse3(m: Mat3) -> Mat3 {
	m.try_inverse().unwrap_or_else(Mat3::identity)
}
pub fn inverse4(m: Mat4) -> Mat4 {
	m.try_inverse().unwrap_or_else(Mat4::identity)
}
pub fn crop_3x3(m: &Mat4) -> Mat3 {
	m.fixed_resize(0.)
}
pub fn normal(p1: Vec3, p2: Vec3, p3: Vec3) -> Vec3 {
	(p2 - p1).cross(&(p3 - p1)).normalize()
}
pub fn translate(m: Mat4, v: Vec3) -> Mat4 {
	m.prepend_translation(&v)
}
pub fn rotate(m: Mat4, angle: f32, axis: Vec3) -> Mat4 {
	m * na::Rotation3::from_axis_angle(&na::Unit::new_normalize(axis), angle).to_homogeneous()
}
pub fn scale(m: Mat4, v: Vec3) -> Mat4 {
	m.prepend_nonuniform_scaling(&v)
}
pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
	Mat4::look_at_rh(&na::OPoint::from(eye), &na::OPoint::from(center), &up)
}
pub fn perspective(aspect: f32, fovy: f32, near: f32, far: f32) -> Mat4 {
	assert!((far - near).abs() > 0.0001, "The near-plane and far-plane must not be superimposed.");
	let mut mat = Mat4::zeros();
	let tan_half_fovy = (fovy / 2.).tan();
	mat[(0, 0)] = 1. / (aspect * tan_half_fovy);
	mat[(1, 1)] = 1. / tan_half_fovy;
	mat[(2, 2)] = -(far + near) / (far - near);
	mat[(2, 3)] = -(2. * far * near) / (far - near);
	mat[(3, 2)] = -1.;

	mat
}
