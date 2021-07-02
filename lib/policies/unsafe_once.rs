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

#[macro_export]
macro_rules! UnsafeLocal {
	($t: ty, $b: block) => {{
		use std::cell::UnsafeCell;
		thread_local!(static S: UnsafeCell<Option<$t>> = UnsafeCell::new(None));
		let mut r = None;
		unsafe {
			S.with(|f| {
				let f = f.get();
				if (*f).is_some() {
				} else {
					*f = Some($b);
				}
				r = Some(f)
			});
			(*r.unwrap()).as_mut().unwrap()
		}
	}};
}

#[macro_export]
macro_rules! FnStatic {
	($n: ident, $t: ty) => {
		fn $n() -> &'static mut $t {
			UnsafeOnce!($t, { Def() })
		}
	};
}

#[macro_export]
macro_rules! FnLocal {
	($n: ident, $t: ty) => {
		fn $n() -> &'static mut $t {
			UnsafeLocal!($t, { Def() })
		}
	};
}
