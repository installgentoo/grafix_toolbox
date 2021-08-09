use crate::uses::*;

impl Cast<Mat2> for glm::Mat2 {
	fn to(v: Mat2) -> Self {
		Self::from(unsafe { mem::transmute::<_, [[f32; 2]; 2]>(v) })
	}
}
impl Cast<Mat3> for glm::Mat3 {
	fn to(v: Mat3) -> Self {
		Self::from(unsafe { mem::transmute::<_, [[f32; 3]; 3]>(v) })
	}
}
impl Cast<Mat4> for glm::Mat4 {
	fn to(v: Mat4) -> Self {
		Self::from(unsafe { mem::transmute::<_, [[f32; 4]; 4]>(v) })
	}
}
impl Cast<Mat2x3> for glm::Mat2x3 {
	fn to(v: Mat2x3) -> Self {
		Self::from(unsafe { mem::transmute::<_, [[f32; 2]; 3]>(v) })
	}
}
impl Cast<Mat2x4> for glm::Mat2x4 {
	fn to(v: Mat2x4) -> Self {
		Self::from(unsafe { mem::transmute::<_, [[f32; 2]; 4]>(v) })
	}
}
impl Cast<Mat3x2> for glm::Mat3x2 {
	fn to(v: Mat3x2) -> Self {
		Self::from(unsafe { mem::transmute::<_, [[f32; 3]; 2]>(v) })
	}
}
impl Cast<Mat3x4> for glm::Mat3x4 {
	fn to(v: Mat3x4) -> Self {
		Self::from(unsafe { mem::transmute::<_, [[f32; 3]; 4]>(v) })
	}
}
impl Cast<Mat4x2> for glm::Mat4x2 {
	fn to(v: Mat4x2) -> Self {
		Self::from(unsafe { mem::transmute::<_, [[f32; 4]; 2]>(v) })
	}
}
impl Cast<Mat4x3> for glm::Mat4x3 {
	fn to(v: Mat4x3) -> Self {
		Self::from(unsafe { mem::transmute::<_, [[f32; 4]; 3]>(v) })
	}
}

impl Cast<glm::Mat2> for Mat2 {
	fn to(v: glm::Mat2) -> Self {
		unsafe { mem::transmute::<[[f32; 2]; 2], _>(v.into()) }
	}
}
impl Cast<glm::Mat3> for Mat3 {
	fn to(v: glm::Mat3) -> Self {
		unsafe { mem::transmute::<[[f32; 3]; 3], _>(v.into()) }
	}
}
impl Cast<glm::Mat4> for Mat4 {
	fn to(v: glm::Mat4) -> Self {
		unsafe { mem::transmute::<[[f32; 4]; 4], _>(v.into()) }
	}
}
impl Cast<glm::Mat2x3> for Mat2x3 {
	fn to(v: glm::Mat2x3) -> Self {
		unsafe { mem::transmute::<[[f32; 2]; 3], _>(v.into()) }
	}
}
impl Cast<glm::Mat2x4> for Mat2x4 {
	fn to(v: glm::Mat2x4) -> Self {
		unsafe { mem::transmute::<[[f32; 2]; 4], _>(v.into()) }
	}
}
impl Cast<glm::Mat3x2> for Mat3x2 {
	fn to(v: glm::Mat3x2) -> Self {
		unsafe { mem::transmute::<[[f32; 3]; 2], _>(v.into()) }
	}
}
impl Cast<glm::Mat3x4> for Mat3x4 {
	fn to(v: glm::Mat3x4) -> Self {
		unsafe { mem::transmute::<[[f32; 3]; 4], _>(v.into()) }
	}
}
impl Cast<glm::Mat4x2> for Mat4x2 {
	fn to(v: glm::Mat4x2) -> Self {
		unsafe { mem::transmute::<[[f32; 4]; 2], _>(v.into()) }
	}
}
impl Cast<glm::Mat4x3> for Mat4x3 {
	fn to(v: glm::Mat4x3) -> Self {
		unsafe { mem::transmute::<[[f32; 4]; 3], _>(v.into()) }
	}
}
