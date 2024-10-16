#[macro_export]
macro_rules! LOGGER {
	($f: path, $l: ident) => {
		let ___errors_logging_main_logger_sink = logging::Logger::initialize($f, logging::Level::$l);
	};
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
	(false, $w: expr) => { ASSERT_IMPL!("{}", $w) };
	(false, $($t: tt)+) => { ASSERT_IMPL!($($t)+) };
	($e: expr, $w: literal) => { if $e {} else { ASSERT_IMPL!($w) } };
	($e: expr, $w: expr) => { if $e {} else { ASSERT_IMPL!("{}", $w) } };
	($e: expr, $($t: tt)+) => { if $e {} else { ASSERT_IMPL!($($t)+) } };
}
#[macro_export]
macro_rules! ASSERT_IMPL {
	($($t: tt)+) => {{
		use $crate::logging::*;
		Logger::log(format!("A| {} |{}:{}|{}\n", format!($($t)+), file!(), line!(), std::thread::current().name().unwrap_or("???")).red().to_string());
		std::panic::set_hook(std::boxed::Box::new(|_| {}));
		panic!();
	}};
}

#[macro_export]
macro_rules! ERROR {
	($e: literal) => { ERROR_IMPL!($e) };
	($e: expr) => { ERROR_IMPL!("{}", $e) };
	($($t: tt)+) => { ERROR_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! ERROR_IMPL {
	($($t: tt)+) => {{
		use $crate::logging::*;
		let E = "E|".red().bold();
		let bt = process_backtrace(std::backtrace::Backtrace::force_capture());
		Logger::log(format!("{E} {bt}{E} {} |{}:{}|{}\n", format!($($t)+).red(), file!(), line!(), std::thread::current().name().unwrap_or("???")));
		std::panic::set_hook(std::boxed::Box::new(|_| {}));
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
	($e: expr) => { WARN_IMPL!("{}", $e) };
	($($t: tt)+) => { WARN_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! WARN_IMPL {
	($($t: tt)+) => {{
		use $crate::logging::*;
		if Level::WARNING as i32 <= Logger::level() {
			let W = "W| ".red().to_string();
			Logger::log([&W, &format!($($t)+), " |", file!(), ":", &line!().to_string(), "\n"].concat());
		}
	}};
}

#[macro_export]
macro_rules! INFO {
	($e: literal) => { INFO_IMPL!($e) };
	($e: expr) => { INFO_IMPL!("{}", $e) };
	($($t: tt)+) => { INFO_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! INFO_IMPL {
	($($t: tt)+) => {{
		use $crate::logging::*;
		if Level::INFO as i32 <= Logger::level() {
			Logger::log(["I| ", &format!($($t)+), " |", file!(), ":", &line!().to_string(), "\n"].concat());
		}
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
	($e: expr) => { DEBUG_IMPL!("{}", $e) };
	($($t: tt)+) => { DEBUG_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! DEBUG_IMPL {
	($($t: tt)+) => {{
		use $crate::logging::*;
		if Level::DEBUG as i32 <= Logger::level() {
			Logger::log(["D| ", &format!($($t)+), "\n"].concat());
		}
	}};
}

#[macro_export]
macro_rules! PRINT {
	($e: literal) => { PRINT_IMPL!($e) };
	($e: expr) => { PRINT_IMPL!("{}", $e) };
	($($t: tt)+) => { PRINT_IMPL!($($t)+) };
}
#[macro_export]
macro_rules! PRINT_IMPL {
	($($t: tt)+) => {{
		use $crate::logging::*;
		if Level::PRINT as i32 <= Logger::level() {
			let mut msg = format!($($t)+);
			msg.push('\n');
			Logger::log( msg);
		}
	}};
}
