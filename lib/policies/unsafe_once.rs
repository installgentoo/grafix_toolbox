#[macro_export]
macro_rules! UnsafeOnce {
	($t: ty, $b: block) => {{
		static mut S: Option<$t> = None;
		unsafe {
			if S.is_some() {
			} else {
				S = Some($b);
			}
			S.as_mut()
		}
		.unwrap()
	}};
}
