use super::super::*;

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
