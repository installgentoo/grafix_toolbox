use crate::lib::*;

macro_rules! arr_cast {
	($from: ty, $to: ty, $dim: literal) => {
		impl Cast<$from> for $to {
			#[inline(always)]
			fn to(v: $from) -> Self {
				<[_; $dim]>::from(v).into()
			}
		}
		impl Cast<$to> for $from {
			#[inline(always)]
			fn to(v: $to) -> Self {
				<[_; $dim]>::from(v).into()
			}
		}
	};
}
macro_rules! impl_vec {
	($($t: ty),+) => {
		$(
			arr_cast!(na::Vector2<$t>, vec2<$t>, 2);
			arr_cast!(na::Vector3<$t>, vec3<$t>, 3);
			arr_cast!(na::Vector4<$t>, vec4<$t>, 4);
			arr_cast!(na::Point2<$t>, vec2<$t>, 2);
			arr_cast!(na::Point3<$t>, vec3<$t>, 3);
			arr_cast!(na::Point4<$t>, vec4<$t>, 4);
		)+
	};
}
impl_vec!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64);

macro_rules! mat_cast {
	($from: ty, $to: ty, $l: literal, $c: literal, $r: literal) => {
		impl Cast<$to> for $from {
			#[inline(always)]
			fn to(v: $to) -> Self {
				let m: [_; $c] = v.into();
				Self::from_column_slice(&m.map(Into::<[_; $r]>::into).concat())
			}
		}
		impl Cast<$from> for $to {
			#[inline(always)]
			fn to(v: $from) -> Self {
				let m: [_; $l] = v.as_slice().try_into().valid();
				let m: [[_; $r]; $c] = unsafe { mem::transmute(m) };
				m.map(Into::into).into()
			}
		}
	};
}
macro_rules! impl_mat {
	($($t: ty),+) => {
		$(
			mat_cast!(na::Matrix2<$t>, mat2<$t>, 4, 2, 2);
			mat_cast!(na::Matrix3<$t>, mat3<$t>, 9, 3, 3);
			mat_cast!(na::Matrix4<$t>, mat4<$t>, 16, 4, 4);
			mat_cast!(na::Matrix2x3<$t>, mat3x2<$t>, 6, 3, 2);
			mat_cast!(na::Matrix2x4<$t>, mat4x2<$t>, 8, 4, 2);
			mat_cast!(na::Matrix3x2<$t>, mat2x3<$t>, 6, 2, 3);
			mat_cast!(na::Matrix3x4<$t>, mat4x3<$t>, 12, 4, 3);
			mat_cast!(na::Matrix4x2<$t>, mat2x4<$t>, 8, 2, 4);
			mat_cast!(na::Matrix4x3<$t>, mat3x4<$t>, 12, 3, 4);
		)+
	};
}
impl_mat!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64);

use nalgebra as na;
