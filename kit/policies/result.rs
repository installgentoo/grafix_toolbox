use super::{math::*, type_name};

pub type Res<T> = Result<T, String>;

pub fn Res<T, V>(v: T) -> Res<V>
where
	Res<V>: Cast<T>,
{
	Res::to(v)
}

impl<T> Cast<Option<T>> for Result<T, String> {
	fn to(v: Option<T>) -> Self {
		v.ok_or_else(|| "Is None".into())
	}
}
impl<T, E: std::fmt::Display> Cast<Result<T, E>> for Result<T, String> {
	fn to(v: Result<T, E>) -> Self {
		v.map_err(|e| {
			let t = type_name::<T>();
			if "String" == t {
				e.to_string()
			} else {
				format!("{t}: {e}")
			}
		}) //TODO specialization
	}
}

pub trait UniformUnwrap<T> {
	fn uni_or_else(self, op: impl FnOnce(&str) -> T) -> T;
}
impl<T> UniformUnwrap<T> for Option<T> {
	fn uni_or_else(self, op: impl FnOnce(&str) -> T) -> T {
		self.unwrap_or_else(|| op("Is None"))
	}
}
impl<T, R: std::fmt::Display> UniformUnwrap<T> for Result<T, R> {
	fn uni_or_else(self, op: impl FnOnce(&str) -> T) -> T {
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
		(T::default(), "Is None".into())
	}
}
impl<T: Default, R: std::fmt::Display> UniformUnwrapOrDefault<T> for Result<T, R> {
	fn uni_is_err(&self) -> bool {
		self.is_err()
	}
	fn uni_err(self) -> (T, String) {
		(T::default(), self.err().unwrap().to_string())
	}
}
