use crate::uses::{ops::*, *};

pub type CowStr = std::borrow::Cow<'static, str>;
pub type Res<T> = Result<T, String>;
pub type Str = &'static str;

#[macro_export]
macro_rules! map_enum {
	($t: pat = $e: expr => $do: expr) => {
		if let $t = $e {
			Some($do)
		} else {
			None
		}
	};
}

#[macro_export]
macro_rules! impl_trait_for {
	($trait: ty = $($types: ty),+) => {
		$(impl $trait for $types {})+
	};
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
