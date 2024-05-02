pub use nalgebra as na; // TODO generalize and unify

pub type V2 = na::Vector2<f32>;
pub type V3 = na::Vector3<f32>;
pub type V4 = na::Vector4<f32>;
pub type M2 = na::Matrix2<f32>;
pub type M3 = na::Matrix3<f32>;
pub type M4 = na::Matrix4<f32>;
pub type Q = na::UnitQuaternion<f32>;

pub fn identity() -> M4 {
	M4::identity()
}
pub fn inverse3(m: M3) -> M3 {
	m.try_inverse().unwrap_or_else(M3::identity)
}
pub fn inverse4(m: M4) -> M4 {
	m.try_inverse().unwrap_or_else(M4::identity)
}
pub fn crop_3x3(m: &M4) -> M3 {
	m.fixed_resize(0.)
}
pub fn normal(p1: V3, p2: V3, p3: V3) -> V3 {
	(p2 - p1).cross(&(p3 - p1)).normalize()
}
pub fn translate(m: M4, v: V3) -> M4 {
	m.prepend_translation(&v)
}
pub fn rotate(m: M4, angle: f32, axis: na::Unit<V3>) -> M4 {
	let r: M4 = Q::from_axis_angle(&axis, angle).to_rotation_matrix().into();
	m * r
}
pub fn scale(m: M4, v: V3) -> M4 {
	m.prepend_nonuniform_scaling(&v)
}
pub fn iL(pos: V3) -> M4 {
	na::Translation3::new(-pos.x, -pos.y, -pos.z).to_homogeneous()
}
pub fn perspective(aspect: f32, fovy: f32, near: f32, far: f32) -> M4 {
	assert!((far - near).abs() > 0.0001, "The near-plane and far-plane must not be superimposed.");
	let mut mat = M4::zeros();
	let tan_half_fovy = (fovy / 2.).tan();
	mat[(0, 0)] = 1. / (aspect * tan_half_fovy);
	mat[(1, 1)] = 1. / tan_half_fovy;
	mat[(2, 2)] = -(far + near) / (far - near);
	mat[(2, 3)] = -(2. * far * near) / (far - near);
	mat[(3, 2)] = -1.;

	mat
}
