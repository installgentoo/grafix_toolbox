use super::super::pre::*;

impl<T: Copy> Cast<mat4<T>> for mat3<T> {
	fn to(((v11, v12, v13, _), (v21, v22, v23, _), (v31, v32, v33, _), _): mat4<T>) -> Self {
		((v11, v12, v13), (v21, v22, v23), (v31, v32, v33))
	}
}
impl<T: Copy> Cast<mat4<T>> for mat2<T> {
	fn to(((v11, v12, _, _), (v21, v22, _, _), _, _): mat4<T>) -> Self {
		((v11, v12), (v21, v22))
	}
}
impl<T: Copy> Cast<mat3<T>> for mat2<T> {
	fn to(((v11, v12, _), (v21, v22, _), _): mat3<T>) -> Self {
		((v11, v12), (v21, v22))
	}
}
impl<T: Copy + Cast<u32>> Cast<mat2<T>> for mat3<T> {
	fn to(((v11, v12), (v21, v22)): mat2<T>) -> Self {
		let (z, o) = <(T, T)>::to((0, 1));
		((v11, v12, z), (v21, v22, z), (z, z, o))
	}
}
impl<T: Copy + Cast<u32>> Cast<mat2<T>> for mat4<T> {
	fn to(((v11, v12), (v21, v22)): mat2<T>) -> Self {
		let (z, o) = <(T, T)>::to((0, 1));
		((v11, v12, z, z), (v21, v22, z, z), (z, z, o, z), (z, z, z, o))
	}
}
impl<T: Copy + Cast<u32>> Cast<mat3<T>> for mat4<T> {
	fn to(((v11, v12, v13), (v21, v22, v23), (v31, v32, v33)): mat3<T>) -> Self {
		let (z, o) = <(T, T)>::to((0, 1));
		((v11, v12, v13, z), (v21, v22, v23, z), (v31, v32, v33, z), (z, z, z, o))
	}
}

pub trait FlattenCast<T> {
	fn flatten(self) -> Vec<T>;
}
impl<T: Copy> FlattenCast<T> for &[vec2<T>] {
	fn flatten(self) -> Vec<T> {
		self.iter().flat_map(|&(x, y)| [x, y]).collect()
	}
}
impl<T: Copy> FlattenCast<T> for &[vec3<T>] {
	fn flatten(self) -> Vec<T> {
		self.iter().flat_map(|&(x, y, z)| [x, y, z]).collect()
	}
}
impl<T: Copy> FlattenCast<T> for &[vec4<T>] {
	fn flatten(self) -> Vec<T> {
		self.iter().flat_map(|&(x, y, z, a)| [x, y, z, a]).collect()
	}
}
impl<T: Copy> FlattenCast<T> for vec2<T> {
	fn flatten(self) -> Vec<T> {
		vec![self.0, self.1]
	}
}
impl<T: Copy> FlattenCast<T> for vec3<T> {
	fn flatten(self) -> Vec<T> {
		vec![self.0, self.1, self.2]
	}
}
impl<T: Copy> FlattenCast<T> for vec4<T> {
	fn flatten(self) -> Vec<T> {
		vec![self.0, self.1, self.2, self.3]
	}
}

pub trait FlattenCastMat<T> {
	fn flatten_all(self) -> Vec<T>;
}
impl<N: Copy, T: FlattenCast<N>> FlattenCastMat<N> for vec2<T> {
	fn flatten_all(self) -> Vec<N> {
		[self.0.flatten(), self.1.flatten()].concat()
	}
}
impl<N: Copy, T: FlattenCast<N>> FlattenCastMat<N> for vec3<T> {
	fn flatten_all(self) -> Vec<N> {
		[self.0.flatten(), self.1.flatten(), self.2.flatten()].concat()
	}
}
impl<N: Copy, T: FlattenCast<N>> FlattenCastMat<N> for vec4<T> {
	fn flatten_all(self) -> Vec<N> {
		[self.0.flatten(), self.1.flatten(), self.2.flatten(), self.3.flatten()].concat()
	}
}
