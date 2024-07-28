#[macro_use]
pub mod pointer;

pub mod cached;
pub mod cached_str;
pub mod lazy;
pub mod memoized;
pub mod prefetch;

pub mod ext {
	pub type STR = &'static str;
	pub type Str = Box<str>;
	pub type Astr = std::sync::Arc<str>;

	pub fn Box<T>(v: T) -> Box<T> {
		Box::new(v)
	}

	pub fn Def<T: Default>() -> T {
		Default::default()
	}
}
