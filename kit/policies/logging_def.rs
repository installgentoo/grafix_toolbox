#[macro_export]
macro_rules! LOGGER {
	($f: path, $l: ident) => {
		let ___errors_logging_main_logger_sink = logging::Logger::new($f, logging::Level::$l);
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
	($e: expr) => { EXPECT_OR_DEF_IMPL!($e, "{}") };
	($e: expr, $($t: tt)+) => { EXPECT_OR_DEF_IMPL!($e, $($t)+) };
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
			unsafe{ e.unwrap_unchecked() }
		}
	}};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! ASSERT {
	(false, $($t: tt)+) => {{
		unreachable!();
	}};
	($e: expr, $($t: tt)+) => {{}};
}
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! ASSERT {
	(false, $w: literal) => { ASSERT_IMPL!($w) };
	(false, $w: expr) => { ASSERT_IMPL!("{:?}", $w) };
	(false, $($t: tt)+) => { ASSERT_IMPL!($($t)+) };
	($e: expr, $w: literal) => {{ if $e {} else { ASSERT_IMPL!($w) } }};
	($e: expr, $w: expr) => {{ if $e {} else { ASSERT_IMPL!("{:?}", $w) } }};
	($e: expr, $($t: tt)+) => {{ if $e {} else { ASSERT_IMPL!($($t)+) } }};
}
#[macro_export]
macro_rules! ASSERT_IMPL {
	($($t: tt)+) => {{
		use logging::*;
		Logger::Log(Level::PRINT, ["A| ", &format!($($t)+), " at ", file!(), ":", &line!().to_string(), "\n"].concat());
		panic!();
	}};
}

#[macro_export]
macro_rules! ERROR {
	($e: literal) => { ERROR_IMPL!($e) };
	($e: expr) => { ERROR_IMPL!("{:?}", $e) };
	($($t: tt)+) => { ERROR_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! ERROR_IMPL {
	($($t: tt)+) => {{
		let bt = std::backtrace::Backtrace::force_capture();
		use logging::*;
		Logger::Log(Level::PRINT, ["E| ", &format!($($t)+), &format!(" at {}:{}\n{bt}", file!(), line!())].concat());
		panic!();
	}};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! FAIL {
	($($t: tt)+) => { WARN!($($t)+) };
}
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! FAIL {
	($($t: tt)+) => { ASSERT!(0 == 1, $($t)+) };
}

#[macro_export]
macro_rules! WARN {
	($e: literal) => { WARN_IMPL!($e) };
	($e: expr) => { WARN_IMPL!("{:?}", $e) };
	($($t: tt)+) => { WARN_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! WARN_IMPL {
	($($t: tt)+) => {{
		use logging::*;
		Logger::Log(Level::WARNING, ["W| ", &format!($($t)+), " at ", file!(), ":", &line!().to_string(), "\n"].concat());
	}};
}

#[macro_export]
macro_rules! INFO {
	($e: literal) => { INFO_IMPL!($e) };
	($e: expr) => { INFO_IMPL!("{:?}", $e) };
	($($t: tt)+) => { INFO_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! INFO_IMPL {
	($($t: tt)+) => {{
		use logging::*;
		Logger::Log(Level::INFO, ["I| ", &format!($($t)+), " at ", file!(), ":", &line!().to_string(), "\n"].concat());
	}};
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! DEBUG {
	($($t: tt)+) => {{}};
}
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! DEBUG {
	($e: literal) => { DEBUG_IMPL!($e) };
	($e: expr) => { DEBUG_IMPL!("{:?}", $e) };
	($($t: tt)+) => { DEBUG_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! DEBUG_IMPL {
	($($t: tt)+) => {{
		use logging::*;
		Logger::Log(Level::DEBUG, ["D| ", &format!($($t)+), "\n"].concat());
	}};
}

#[macro_export]
macro_rules! PRINT {
	($e: literal) => { PRINT_IMPL!($e) };
	($e: expr) => { PRINT_IMPL!("{:?}", $e) };
	($($t: tt)+) => { PRINT_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! PRINT_IMPL {
	($($t: tt)+) => {{
		let mut msg = format!($($t)+);
		msg.push('\n');
		use logging::*;
		Logger::Log(Level::PRINT, msg);
	}};
}
