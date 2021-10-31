#[macro_export]
macro_rules! LOGGER {
	($f: expr, $l: ident) => {
		let ___errors_logging_main_logger_sink = logging::Logger::Setup($f, logging::Level::$l);
	};
}

#[macro_export]
macro_rules! conc {
	($($t: expr),+) => {
		[$($t),+].concat()
	};
}

#[macro_export]
macro_rules! EXPECT {
	($e: expr) => {{
		use logging::UniformUnwrap;
		$e.uni_or_else(|_e| ASSERT!(false, "{}", _e))
	}};
	($e: expr, $($t: tt)+) => {{
		use logging::UniformUnwrap;
		$e.uni_or_else(|e| { PRINT!(e); ASSERT!(false, $($t)+) })
	}};
}

#[macro_export]
macro_rules! OR_DEFAULT {
	($e: expr) => {{
		EXPECT_OR_DEF_IMPL!($e, "{}")
	}};
	($e: expr, $($t: tt)+) => {{
		EXPECT_OR_DEF_IMPL!($e, $($t)+)
	}}
}
#[macro_export]
macro_rules! EXPECT_OR_DEF_IMPL {
	($e: expr, $($t: tt)+) => {{
		let e = $e;
		use logging::UniformUnwrapOrDefault;
		if e.uni_is_err() {
			let (v, e) = e.uni_err();
			FAIL!($($t)+, e);
			v
		} else {
			e.unwrap()
		}
	}};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! ASSERT {
	(false, $($t: tt)+) => {{
		unreachable!()
	}};
	($e: expr, $($t: tt)+) => {{}};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! ASSERT {
	(false, $w: expr) => {{
		ASSERT_IMPL!("{:?}", $w);
	}};
	(false, $($t: tt)+) => {{
		ASSERT_IMPL!($($t)+);
	}};
	($e: expr, $w: expr) => {{
		if $e {} else {
			ASSERT_IMPL!("{:?}", $w);
		}
	}};
	($e: expr, $($t: tt)+) => {{
		if $e {} else {
			ASSERT_IMPL!($($t)+);
		}
	}}
}
#[macro_export]
macro_rules! ASSERT_IMPL {
	($($t: tt)+) => {{
		logging::Logger::Log(conc!("A| ", &format!($($t)+), " at ", file!(), ":", &line!().to_string(), "\n"));
		panic!()
	}}
}

#[macro_export]
macro_rules! ERROR {
	($e: expr) => {{ ERROR_IMPL!("{:?}", $e); }};
	($($t: tt)+) => {{ ERROR_IMPL!($($t)+); }}
}
#[macro_export]
macro_rules! ERROR_IMPL {
	($($t: tt)+) => {{
		use logging::*;
		Logger::Log(conc!("E| ", &format!($($t)+), " at ", file!(), ":", &line!().to_string(), "\n"));
		Logger::Log(format!("{:?}", backtrace::Backtrace::new()));
		panic!();
	}};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! FAIL {
	($($t: tt)+) => {{
		WARN!($($t)+);
	}}
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! FAIL {
	($($t: tt)+) => {{
		ASSERT!(0 == 1, $($t)+);
	}}
}

#[macro_export]
macro_rules! WARN {
	($e: expr) => {{ WARN_IMPL!("{:?}", $e); }};
	($($t: tt)+) => {{ WARN_IMPL!($($t)+); }}
}
#[macro_export]
macro_rules! WARN_IMPL {
	($($t: tt)+) => {{
		use logging::*;
		if (Level::WARNING as i32) <= Logger::level() {
			Logger::Log(conc!("W| ", &format!($($t)+), " at ", file!(), ":", &line!().to_string(), "\n"));
		}
	}};
}

#[macro_export]
macro_rules! INFO {
	($e: expr) => {{ INFO_IMPL!("{:?}", $e); }};
	($($t: tt)+) => {{ INFO_IMPL!($($t)+); }}
}
#[macro_export]
macro_rules! INFO_IMPL {
	($($t: tt)+) => {{
		use logging::*;
		if (Level::INFO as i32) <= Logger::level() {
			Logger::Log(conc!("I| ", &format!($($t)+), " at ", file!(), ":", &line!().to_string(), "\n"));
		}
	}}
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! DEBUG {
	($e: expr) => {{}};
	($($t: tt)+) => {{}};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! DEBUG {
	($e: expr) => {{ DEBUG_IMPL!("{:?}", $e); }};
	($($t: tt)+) => {{ DEBUG_IMPL!($($t)+); }}
}
#[macro_export]
macro_rules! DEBUG_IMPL {
	($($t: tt)+) => {{
		use logging::*;
		if (Level::DEBUG as i32) <= Logger::level() {
			Logger::Log(conc!("D| ", &format!($($t)+), "\n"));
		}
	}}
}

#[macro_export]
macro_rules! PRINT {
	($e: expr) => {{ PRINT_IMPL!("{:?}", $e); }};
	($($t: tt)+) => {{ PRINT_IMPL!($($t)+); }}
}
#[macro_export]
macro_rules! PRINT_IMPL {
	($($t: tt)+) => {{
		logging::Logger::Log(conc!(&format!($($t)+), "\n"));
	}}
}
