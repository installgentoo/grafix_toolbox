use super::super::{super::ext::UnwrapValid, *};

macro_rules! array_recast {
	($from: ty, $to: ty, $dim: literal) => {
		impl Cast<$from> for $to {
			fn to(v: $from) -> Self {
				<[_; $dim]>::from(v).into()
			}
		}
		impl Cast<$to> for $from {
			fn to(v: $to) -> Self {
				<[_; $dim]>::from(v).into()
			}
		}
	};
}

macro_rules! impl_vec {
	($t: ty) => {
		array_recast!(na::Vector2<$t>, vec2<$t>, 2);
		array_recast!(na::Vector3<$t>, vec3<$t>, 3);
		array_recast!(na::Vector4<$t>, vec4<$t>, 4);

		array_recast!(na::Point2<$t>, vec2<$t>, 2);
		array_recast!(na::Point3<$t>, vec3<$t>, 3);
		array_recast!(na::Point4<$t>, vec4<$t>, 4);
	};
}
impl_vec!(u8);
impl_vec!(i8);
impl_vec!(u16);
impl_vec!(i16);
impl_vec!(u32);
impl_vec!(i32);
impl_vec!(u64);
impl_vec!(i64);
impl_vec!(u128);
impl_vec!(i128);
impl_vec!(f16);
impl_vec!(f32);
impl_vec!(f64);

macro_rules! mat_recast {
	($from: ty, $to: ty, $l: literal, $c: literal, $r: literal) => {
		impl Cast<$to> for $from {
			fn to(v: $to) -> Self {
				let m: [_; $c] = v.into();
				Self::from_column_slice(&m.map(Into::<[_; $r]>::into).concat())
			}
		}
		impl Cast<$from> for $to {
			fn to(v: $from) -> Self {
				let m: [_; $l] = v.as_slice().try_into().valid();
				let m: [[_; $r]; $c] = unsafe { std::mem::transmute(m) };
				m.map(Into::into).into()
			}
		}
	};
}

macro_rules! impl_mat {
	($t: ty) => {
		mat_recast!(na::Matrix2<$t>, mat2<$t>, 4, 2, 2);
		mat_recast!(na::Matrix3<$t>, mat3<$t>, 9, 3, 3);
		mat_recast!(na::Matrix4<$t>, mat4<$t>, 16, 4, 4);
		mat_recast!(na::Matrix2x3<$t>, mat3x2<$t>, 6, 3, 2);
		mat_recast!(na::Matrix2x4<$t>, mat4x2<$t>, 8, 4, 2);
		mat_recast!(na::Matrix3x2<$t>, mat2x3<$t>, 6, 2, 3);
		mat_recast!(na::Matrix3x4<$t>, mat4x3<$t>, 12, 4, 3);
		mat_recast!(na::Matrix4x2<$t>, mat2x4<$t>, 8, 2, 4);
		mat_recast!(na::Matrix4x3<$t>, mat3x4<$t>, 12, 3, 4);
	};
}
impl_mat!(u8);
impl_mat!(i8);
impl_mat!(u16);
impl_mat!(i16);
impl_mat!(u32);
impl_mat!(i32);
impl_mat!(u64);
impl_mat!(i64);
impl_mat!(u128);
impl_mat!(i128);
impl_mat!(f16);
impl_mat!(f32);
impl_mat!(f64);

use nalgebra as na;
