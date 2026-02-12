use crate::lib::*;

pub type Res<T> = Result<T, String>;

pub trait ExplainError<T>: Sized {
	fn explain_err<S: Into<String>>(self, msg: impl FnOnce() -> S) -> Res<T>;
}
impl<T> ExplainError<T> for Option<T> {
	fn explain_err<S: Into<String>>(self, msg: impl FnOnce() -> S) -> Res<T> {
		self.map_or_else(|| [&msg().into(), ": is None"].concat().pipe(Err), Ok)
	}
}
impl<T, E: Display> ExplainError<T> for Result<T, E> {
	fn explain_err<S: Into<String>>(self, msg: impl FnOnce() -> S) -> Res<T> {
		self.map_err(|e| [msg().into(), format!(":\n{e}")].concat())
	}
}

pub trait FlattenError<T> {
	fn chain_err(self) -> Res<T>;
}
impl<T, E: Display> FlattenError<T> for Result<Option<T>, E> {
	fn chain_err(self) -> Res<T> {
		self.res().and_then(|o| o.res())
	}
}
impl<T, E: Display> FlattenError<T> for Option<Result<T, E>> {
	fn chain_err(self) -> Res<T> {
		self.res().and_then(|o| o.res())
	}
}
impl<T, E1: Display, E2: Display> FlattenError<T> for Result<Result<T, E1>, E2> {
	fn chain_err(self) -> Res<T> {
		self.res().and_then(|o| o.res())
	}
}

pub trait ThreadUnwrap<T> {
	fn join_fail<S: Into<String>>(self, explanation: impl FnOnce() -> S) -> T;
	fn join_res(self) -> Res<T>;
}
impl<T> ThreadUnwrap<T> for JoinHandle<T> {
	fn join_fail<S: Into<String>>(self, ex: impl FnOnce() -> S) -> T {
		self.join()
			.map_err(|e| {
				PRINT!(ex().into());
				std::panic::resume_unwind(e)
			})
			.valid()
	}
	fn join_res(self) -> Res<T> {
		let (id, name) = self.thread().pipe(|t| (t.id(), t.name().unwrap_or("???").to_string()));
		self.join().map_err(|_| format!("Thread {name:?}<{}>({id:?}) panicked", type_name::<T>()))
	}
}
impl<T, E: Display> ThreadUnwrap<T> for Result<JoinHandle<T>, E> {
	fn join_fail<S: Into<String>>(self, e: impl FnOnce() -> S) -> T {
		match self {
			Ok(s) => s.join_fail(e),
			err @ Err(_) => {
				err.explain_err(e).fail();
				unreachable!()
			}
		}
	}
	fn join_res(self) -> Res<T> {
		self.res()?.join_res()
	}
}

pub trait UniformUnwrap<T>: Sized {
	fn res(self) -> Res<T>;
	fn uni_is_err(&self) -> bool;
	fn uni_or_else(self, op: impl FnOnce(&str) -> T) -> T;
	fn fail(self) -> T {
		self.uni_or_else(|e| ERROR!(e))
	}
	#[inline(always)]
	fn sink(self) {}
}
impl<T> UniformUnwrap<T> for Option<T> {
	fn res(self) -> Res<T> {
		self.ok_or_else(|| "Is None".into())
	}
	fn uni_is_err(&self) -> bool {
		self.is_none()
	}
	fn uni_or_else(self, op: impl FnOnce(&str) -> T) -> T {
		self.unwrap_or_else(|| op("Is None"))
	}
}
impl<T, R: Display> UniformUnwrap<T> for Result<T, R> {
	fn res(self) -> Res<T> {
		self.map_err(|e| {
			let t = type_name::<T>();
			if "String" == t { e.to_string() } else { format!("{t}: {e}") }
		})
	}
	fn uni_is_err(&self) -> bool {
		self.is_err()
	}
	fn uni_or_else(self, op: impl FnOnce(&str) -> T) -> T {
		self.unwrap_or_else(|e| op(&e.to_string()))
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
		self.unwrap_or_else(|| FAIL!({ T::default() }, "Is None"))
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
