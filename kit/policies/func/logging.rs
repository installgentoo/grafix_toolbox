pub use super::result::{UniformUnwrap, UniformUnwrapOrDefault};
pub use text_color::term::*;

pub async fn Term() -> io::Stdout {
	io::stdout()
}

pub async fn TermErr() -> io::Stderr {
	io::stderr()
}

pub async fn File() -> fs::File {
	term_color::disable();
	unwrap(fs::File::create("log.txt").await, "couldn't create log file")
}

pub async fn Null() -> io::Sink {
	io::sink()
}

pub struct Logger;
impl Logger {
	pub fn initialize<F>(out: impl FnOnce() -> F + SendStat, l: Level) -> Self
	where
		F: Future<Output: AsyncWrite + Unpin + Send> + Send,
	{
		Self::init_logger(out, l);
		Self
	}
	pub fn log(msg: String) {
		Self::init_logger(check_order, Level::INFO);

		unwrap(unwrap_o(unsafe { LOGGER.get() }, "already exited").sender.send(Message::M(msg)), "failed to send");
	}
	pub fn shutdown_hook(f: impl FnOnce() + SendStat) {
		POSTMORTEM.lock().expect("E| Shutdown hook add failed").push(Box(f));
	}

	pub fn level() -> i32 {
		unsafe { LEVEL as i32 }
	}
	pub fn set_level(l: Level) {
		unsafe { LEVEL = l }
	}
	fn init_logger<F>(out: impl FnOnce() -> F + SendStat, l: Level)
	where
		F: Future<Output: AsyncWrite + Unpin + Send> + Send,
	{
		unsafe {
			LOGGER.get_or_init(move || {
				std::panic::set_hook(Box(|i| {
					let bt = process_backtrace(Backtrace::force_capture());
					let p = i.payload();
					let p = p.downcast_ref::<String>().cloned().or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()));
					let P = "P|".red().bold();
					Logger::log(format!(
						"{P} {bt}{P} {} |{}|{}\n",
						p.unwrap_or("???".into()).red(),
						i.location().map_or("???".to_string(), |s| format!("{s}")),
						thread::current().name().unwrap_or("???")
					));
				}));
				LEVEL = l;
				let (sender, reciever) = chan::unbounded::<Message>();
				let writer = task::Runtime().spawn(async move {
					let mut out = out().await;
					while let Ok(msg) = reciever.recv_async().await {
						if let Message::M(msg) = msg {
							unwrap(out.write_all(msg.as_bytes()).await, "failed write");
						} else {
							unwrap(out.flush().await, "failed flush");
							break;
						}
					}
				});
				LoggerState { writer, sender }
			});
		}
	}
}
impl Drop for Logger {
	fn drop(&mut self) {
		Self::init_logger(check_order, Level::INFO);
		POSTMORTEM.lock().expect("E| Shutdown hooks failed").drain(..).for_each(|f| f());
		let LoggerState { writer, sender } = unsafe { LOGGER.take() }.valid();
		unwrap(sender.send(Message::Close), "failed to close system");
		task::Runtime().finish(writer);
	}
}

#[derive(Clone, Copy)]
pub enum Level {
	SILENT = 0,
	PRINT = 1,
	WARNING = 2,
	INFO = 3,
	DEBUG = 4,
}
static mut LEVEL: Level = Level::INFO;
static mut LOGGER: OnceLock<LoggerState> = OnceLock::new();
static POSTMORTEM: Mutex<Vec<Box<dyn FnOnce() + Send>>> = Mutex::new(vec![]);

struct LoggerState {
	writer: Task<()>,
	sender: Sender<Message>,
}

enum Message {
	M(String),
	Close,
}

pub fn process_backtrace(bt: Backtrace) -> String {
	let ignore = [
		"          at /rustc/",
		"kit/policies/logging.rs:",
		": std::panic",
		": std::rt::lang_start",
		": std::sys_common::backtrace::",
		": core::ops::function::",
		": rust_begin_unwind",
		": core::panicking::",
		": __libc_start_",
		": _start",
		": <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call",
		": grafix_toolbox::kit::policies::logging::Logger::init_logger::",
	];

	let bt = bt
		.to_string()
		.lines()
		.filter(|l| {
			for i in ignore {
				if l.contains(i) {
					return false;
				}
			}
			true
		})
		.chain(["  ↑"])
		.fold("BACKTRACE\n".to_string(), |a, l| a + l + "\n");
	bt
}

fn unwrap<T, E: std::fmt::Debug>(v: Result<T, E>, msg: &str) -> T {
	match v {
		Ok(v) => v,
		Err(e) => {
			let bt = process_backtrace(Backtrace::force_capture());
			println!("{bt}Logger panic: {msg} {e:?}");
			panic!();
		}
	}
}
fn unwrap_o<T>(v: Option<T>, msg: &str) -> T {
	match v {
		Some(v) => v,
		None => {
			let bt = process_backtrace(Backtrace::force_capture());
			println!("{bt}Logger panic: {msg}");
			panic!();
		}
	}
}

async fn check_order() -> io::Stdout {
	panic!("E| No logger! Add 'LOGGER!(logging::Term, INFO);' as first line in main()");
}

use crate::{asyn::*, lib::*, text_color};
use std::backtrace::Backtrace;
