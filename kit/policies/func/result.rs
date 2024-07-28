use crate::lib::*;
use std::fmt::Display;

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
impl<T, E: Display> Cast<Result<T, E>> for Result<T, String> {
	fn to(v: Result<T, E>) -> Self {
		v.map_err(|e| {
			let t = type_name::<T>();
			if "String" == t {
				e.to_string()
			} else {
				format!("{t}: {e}")
			}
		})
	}
}

pub trait ExplainError<T>: Sized {
	fn explain_err<R: Into<String>>(self, msg: impl FnOnce() -> R) -> Res<T>;
}
impl<T> ExplainError<T> for Option<T> {
	fn explain_err<R: Into<String>>(self, msg: impl FnOnce() -> R) -> Res<T> {
		self.map_or_else(|| Err([&msg().into(), ": Is None"].concat()), |s| Ok(s))
	}
}
impl<T, E: Display> ExplainError<T> for Result<T, E> {
	fn explain_err<R: Into<String>>(self, msg: impl FnOnce() -> R) -> Res<T> {
		self.map_err(|e| [msg().into(), format!(":\n{e}")].concat())
	}
}

pub trait UniformUnwrap<T>: Sized {
	fn uni_is_err(&self) -> bool;
	fn uni_or_else(self, op: impl FnOnce(&str) -> T) -> T;
	fn fail(self) -> T {
		self.uni_or_else(|e| ERROR!(e))
	}
	fn sink(self);
}
impl<T> UniformUnwrap<T> for Option<T> {
	fn uni_is_err(&self) -> bool {
		self.is_none()
	}
	fn uni_or_else(self, op: impl FnOnce(&str) -> T) -> T {
		self.unwrap_or_else(|| op("Is None"))
	}
	fn sink(self) {
		let _ = self;
	}
}
impl<T, R: Display> UniformUnwrap<T> for Result<T, R> {
	fn uni_is_err(&self) -> bool {
		self.is_err()
	}
	fn uni_or_else(self, op: impl FnOnce(&str) -> T) -> T {
		self.unwrap_or_else(|e| op(&e.to_string()))
	}
	fn sink(self) {
		let _ = self;
	}
}

pub trait UniformUnwrapOrDefault<T: Default>: UniformUnwrap<T> {
	fn uni_err(self) -> (T, String);
	fn warn(self) -> T;
}
impl<T: Default> UniformUnwrapOrDefault<T> for Option<T> {
	fn uni_err(self) -> (T, String) {
		(T::default(), "Is None".into())
	}
	fn warn(self) -> T {
		self.unwrap_or_else(|| {
			FAIL!("Is None");
			T::default()
		})
	}
}
impl<T: Default, R: Display> UniformUnwrapOrDefault<T> for Result<T, R> {
	fn uni_err(self) -> (T, String) {
		(T::default(), self.err().valid().to_string())
	}
	fn warn(self) -> T {
		self.map_err(|e| FAIL!(e)).unwrap_or_default()
	}
}
