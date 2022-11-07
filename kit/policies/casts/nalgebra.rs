use crate::uses::*;

macro_rules! impl_vec {
	($t: ty) => {
		impl Cast<glm::TVec2<$t>> for vec2<$t> {
			fn to(v: glm::TVec2<$t>) -> Self {
				unsafe { mem::transmute::<[_; 2], _>(v.into()) }
			}
		}
		impl Cast<glm::TVec3<$t>> for vec3<$t> {
			fn to(v: glm::TVec3<$t>) -> Self {
				unsafe { mem::transmute::<[_; 3], _>(v.into()) }
			}
		}
		impl Cast<glm::TVec4<$t>> for vec4<$t> {
			fn to(v: glm::TVec4<$t>) -> Self {
				unsafe { mem::transmute::<[_; 4], _>(v.into()) }
			}
		}

		impl Cast<vec2<$t>> for glm::TVec2<$t> {
			fn to(v: vec2<$t>) -> Self {
				Self::from(<[_; 2]>::to(v))
			}
		}
		impl Cast<vec3<$t>> for glm::TVec3<$t> {
			fn to(v: vec3<$t>) -> Self {
				Self::from(<[_; 3]>::to(v))
			}
		}
		impl Cast<vec4<$t>> for glm::TVec4<$t> {
			fn to(v: vec4<$t>) -> Self {
				Self::from(<[_; 4]>::to(v))
			}
		}

		impl Cast<na::Point2<$t>> for vec2<$t> {
			fn to(v: na::Point2<$t>) -> Self {
				unsafe { mem::transmute::<[_; 2], _>(v.into()) }
			}
		}
		impl Cast<na::Point3<$t>> for vec3<$t> {
			fn to(v: na::Point3<$t>) -> Self {
				unsafe { mem::transmute::<[_; 3], _>(v.into()) }
			}
		}
		impl Cast<na::Point4<$t>> for vec4<$t> {
			fn to(v: na::Point4<$t>) -> Self {
				unsafe { mem::transmute::<[_; 4], _>(v.into()) }
			}
		}

		impl Cast<vec2<$t>> for na::Point2<$t> {
			fn to(v: vec2<$t>) -> Self {
				Self::from(<[_; 2]>::to(v))
			}
		}
		impl Cast<vec3<$t>> for na::Point3<$t> {
			fn to(v: vec3<$t>) -> Self {
				Self::from(<[_; 3]>::to(v))
			}
		}
		impl Cast<vec4<$t>> for na::Point4<$t> {
			fn to(v: vec4<$t>) -> Self {
				Self::from(<[_; 4]>::to(v))
			}
		}
	};
}
impl_vec!(u8);
impl_vec!(i8);
impl_vec!(u16);
impl_vec!(i16);
impl_vec!(u32);
impl_vec!(i32);
impl_vec!(f16);
impl_vec!(f32);

macro_rules! impl_mat {
	($t: ty) => {
		impl Cast<mat2<$t>> for na::Matrix2<$t> {
			fn to(v: mat2<$t>) -> Self {
				Self::from(unsafe { mem::transmute::<_, [[$t; 2]; 2]>(v) })
			}
		}
		impl Cast<mat3<$t>> for na::Matrix3<$t> {
			fn to(v: mat3<$t>) -> Self {
				Self::from(unsafe { mem::transmute::<_, [[$t; 3]; 3]>(v) })
			}
		}
		impl Cast<mat4<$t>> for na::Matrix4<$t> {
			fn to(v: mat4<$t>) -> Self {
				Self::from(unsafe { mem::transmute::<_, [[$t; 4]; 4]>(v) })
			}
		}
		impl Cast<mat2x3<$t>> for na::Matrix2x3<$t> {
			fn to(v: mat2x3<$t>) -> Self {
				Self::from(unsafe { mem::transmute::<_, [[$t; 2]; 3]>(v) })
			}
		}
		impl Cast<mat2x4<$t>> for na::Matrix2x4<$t> {
			fn to(v: mat2x4<$t>) -> Self {
				Self::from(unsafe { mem::transmute::<_, [[$t; 2]; 4]>(v) })
			}
		}
		impl Cast<mat3x2<$t>> for na::Matrix3x2<$t> {
			fn to(v: mat3x2<$t>) -> Self {
				Self::from(unsafe { mem::transmute::<_, [[$t; 3]; 2]>(v) })
			}
		}
		impl Cast<mat3x4<$t>> for na::Matrix3x4<$t> {
			fn to(v: mat3x4<$t>) -> Self {
				Self::from(unsafe { mem::transmute::<_, [[$t; 3]; 4]>(v) })
			}
		}
		impl Cast<mat4x2<$t>> for na::Matrix4x2<$t> {
			fn to(v: mat4x2<$t>) -> Self {
				Self::from(unsafe { mem::transmute::<_, [[$t; 4]; 2]>(v) })
			}
		}
		impl Cast<mat4x3<$t>> for na::Matrix4x3<$t> {
			fn to(v: mat4x3<$t>) -> Self {
				Self::from(unsafe { mem::transmute::<_, [[$t; 4]; 3]>(v) })
			}
		}

		impl Cast<na::Matrix2<$t>> for mat2<$t> {
			fn to(v: na::Matrix2<$t>) -> Self {
				unsafe { mem::transmute::<[[$t; 2]; 2], _>(v.into()) }
			}
		}
		impl Cast<na::Matrix3<$t>> for mat3<$t> {
			fn to(v: na::Matrix3<$t>) -> Self {
				unsafe { mem::transmute::<[[$t; 3]; 3], _>(v.into()) }
			}
		}
		impl Cast<na::Matrix4<$t>> for mat4<$t> {
			fn to(v: na::Matrix4<$t>) -> Self {
				unsafe { mem::transmute::<[[$t; 4]; 4], _>(v.into()) }
			}
		}
		impl Cast<na::Matrix2x3<$t>> for mat2x3<$t> {
			fn to(v: na::Matrix2x3<$t>) -> Self {
				unsafe { mem::transmute::<[[$t; 2]; 3], _>(v.into()) }
			}
		}
		impl Cast<na::Matrix2x4<$t>> for mat2x4<$t> {
			fn to(v: na::Matrix2x4<$t>) -> Self {
				unsafe { mem::transmute::<[[$t; 2]; 4], _>(v.into()) }
			}
		}
		impl Cast<na::Matrix3x2<$t>> for mat3x2<$t> {
			fn to(v: na::Matrix3x2<$t>) -> Self {
				unsafe { mem::transmute::<[[$t; 3]; 2], _>(v.into()) }
			}
		}
		impl Cast<na::Matrix3x4<$t>> for mat3x4<$t> {
			fn to(v: na::Matrix3x4<$t>) -> Self {
				unsafe { mem::transmute::<[[$t; 3]; 4], _>(v.into()) }
			}
		}
		impl Cast<na::Matrix4x2<$t>> for mat4x2<$t> {
			fn to(v: na::Matrix4x2<$t>) -> Self {
				unsafe { mem::transmute::<[[$t; 4]; 2], _>(v.into()) }
			}
		}
		impl Cast<na::Matrix4x3<$t>> for mat4x3<$t> {
			fn to(v: na::Matrix4x3<$t>) -> Self {
				unsafe { mem::transmute::<[[$t; 4]; 3], _>(v.into()) }
			}
		}
	};
}
impl_mat!(u8);
impl_mat!(i8);
impl_mat!(u16);
impl_mat!(i16);
impl_mat!(u32);
impl_mat!(i32);
impl_mat!(f16);
impl_mat!(f32);
