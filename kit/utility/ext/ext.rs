use crate::uses::{ops::*, *};

pub type CowStr = std::borrow::Cow<'static, str>;
pub type Res<T> = Result<T, String>;
pub type Str = &'static str;

pub fn lambda<'a, T: Fn(A) -> R + 'a, A, R>(f: T) -> Box<dyn Fn(A) -> R + 'a> {
	Box::new(f)
}

pub trait UnwrapValid<T> {
	fn valid(self) -> T;
}
impl<T> UnwrapValid<T> for Option<T> {
	fn valid(self) -> T {
		#[cfg(debug_assertions)]
		{
			self.unwrap()
		}
		#[cfg(not(debug_assertions))]
		{
			unsafe { self.unwrap_unchecked() }
		}
	}
}
impl<T, E: Debug> UnwrapValid<T> for Result<T, E> {
	fn valid(self) -> T {
		#[cfg(debug_assertions)]
		{
			self.unwrap()
		}
		#[cfg(not(debug_assertions))]
		{
			unsafe { self.unwrap_unchecked() }
		}
	}
}

#[macro_export]
macro_rules! map_enum {
	($t: pat = $e: expr => $do: expr) => {{
		if let $t = $e {
			Some($do)
		} else {
			None
		}
	}};
}

pub fn Def<T: Default>() -> T {
	Default::default()
}

pub trait OrAssignment {
	fn or_def(self, filter: bool) -> Self;
	fn or_val(self, filter: bool, val: Self) -> Self;
}
impl<T: Default> OrAssignment for T {
	#[inline(always)]
	fn or_def(self, filter: bool) -> Self {
		if filter {
			self
		} else {
			Def()
		}
	}
	#[inline(always)]
	fn or_val(self, filter: bool, v: Self) -> Self {
		if filter {
			self
		} else {
			v
		}
	}
}

pub trait LerpMix: Cast<f32> {
	fn mix<M>(self, a: M, r: Self) -> Self
	where
		f32: Cast<M>;
}
impl<T: Cast<f32> + Copy + Add<Output = T> + Mul<Output = T>> LerpMix for T {
	fn mix<M>(self, a: M, r: Self) -> Self
	where
		f32: Cast<M>,
	{
		let a = f32(a);
		self * Self::to(1. - a) + r * Self::to(a)
	}
}
