pub trait UnwrapValid<T> {
	fn valid(self) -> T;
}
impl<T> UnwrapValid<T> for Option<T> {
	fn valid(self) -> T {
		#[cfg(debug_assertions)]
		{
			self.expect("E: Not valid: None")
		}
		#[cfg(not(debug_assertions))]
		{
			unsafe { self.unwrap_unchecked() }
		}
	}
}
impl<T, E: std::fmt::Debug> UnwrapValid<T> for Result<T, E> {
	fn valid(self) -> T {
		#[cfg(debug_assertions)]
		{
			self.expect("E: Not valid: Err")
		}
		#[cfg(not(debug_assertions))]
		{
			unsafe { self.unwrap_unchecked() }
		}
	}
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
			Self::default()
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
