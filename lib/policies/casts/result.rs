use crate::uses::*;

impl<T> Cast<Option<T>> for Result<T, String> {
	fn to(v: Option<T>) -> Self {
		v.ok_or("Is None".into())
	}
}
impl<T, E: std::fmt::Display> Cast<Result<T, E>> for Result<T, String> {
	fn to(v: Result<T, E>) -> Self {
		v.map_err(|e| {
			let t = type_name!(T);
			if "String" == t {
				e.to_string()
			} else {
				CONCAT!(&t, ": ", &e.to_string())
			}
		}) //TODO specialization
	}
}

pub trait UniformUnwrap<T> {
	fn uni_or_else<F: FnOnce(&str) -> T>(self, op: F) -> T;
}
impl<T> UniformUnwrap<T> for Option<T> {
	fn uni_or_else<F: FnOnce(&str) -> T>(self, op: F) -> T {
		self.unwrap_or_else(|| op("Is None"))
	}
}
impl<T, R: std::fmt::Display> UniformUnwrap<T> for Result<T, R> {
	fn uni_or_else<F: FnOnce(&str) -> T>(self, op: F) -> T {
		self.unwrap_or_else(|e| op(&e.to_string()))
	}
}

pub trait UniformUnwrapOrDefault<T: Default> {
	fn uni_is_err(&self) -> bool;
	fn uni_err(self) -> (T, String);
}
impl<T: Default> UniformUnwrapOrDefault<T> for Option<T> {
	fn uni_is_err(&self) -> bool {
		self.is_none()
	}
	fn uni_err(self) -> (T, String) {
		(Def(), "Is None".into())
	}
}
impl<T: Default, R: std::fmt::Display> UniformUnwrapOrDefault<T> for Result<T, R> {
	fn uni_is_err(&self) -> bool {
		self.is_err()
	}
	fn uni_err(self) -> (T, String) {
		(Def(), self.err().unwrap().to_string())
	}
}
