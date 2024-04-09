pub use super::result::{UniformUnwrap, UniformUnwrapOrDefault};
use super::{derives::*, ext::*};
use crate::{asyn::*, sync};
use std::backtrace::Backtrace;

pub async fn Term() -> Unblock<sync::io::Stdout> {
	Unblock::new(sync::io::stdout())
}

pub async fn TermErr() -> Unblock<sync::io::Stderr> {
	Unblock::new(sync::io::stderr())
}

pub async fn File() -> fs::File {
	unwrap(fs::File::create("log.txt").await, "couldn't create log file")
}

pub async fn Null() -> Unblock<io::Sink> {
	Unblock::new(io::sink())
}

pub struct Logger;
impl Logger {
	pub fn new<T, F>(out: impl FnOnce() -> F + SendStat, l: Level) -> Self
	where
		T: AsyncWrite + Unpin + Send,
		F: Future<Output = T> + Send,
	{
		Self::init_logger(out, l);
		Self
	}
	#[inline(always)]
	pub fn log(msg: String) {
		Self::init_logger(check_order, Level::INFO);

		unwrap(unwrap_o(unsafe { LOGGER.get() }, "already exited").sender.send(Message::M(msg)), "failed to send");
	}
	pub fn shutdown_hook(f: impl FnOnce() + SendStat) {
		POSTMORTEM.lock().expect("Shutdown hook add failed").push(Box(f));
	}

	pub fn level() -> i32 {
		unsafe { LEVEL as i32 }
	}
	pub fn set_level(l: Level) {
		unsafe { LEVEL = l }
	}
	fn init_logger<T, F>(out: impl FnOnce() -> F + SendStat, l: Level)
	where
		T: AsyncWrite + Unpin + Send,
		F: Future<Output = T> + Send,
	{
		unsafe {
			LOGGER.get_or_init(move || {
				std::panic::set_hook(Box(|i| {
					let bt = process_backtrace(Backtrace::force_capture());
					let p = i.payload();
					let p = p.downcast_ref::<String>().cloned().or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()));
					Logger::log(format!(
						"P| {bt}\x1B[31mP| {} |{}|{}\x1B[0m\n",
						p.unwrap_or("???".into()),
						i.location().map_or("???".to_string(), |s| format!("{s}")),
						thread::current().name().unwrap_or("???")
					));
				}));
				LEVEL = l;
				let (sender, reciever) = chan::unbounded::<Message>();
				let handle = task::spawn(async move {
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
				LoggerState { handle, sender }
			});
		}
	}
}
impl Drop for Logger {
	fn drop(&mut self) {
		Self::init_logger(check_order, Level::INFO);
		POSTMORTEM.lock().expect("Shutdown hooks failed").drain(..).for_each(|f| f());
		let LoggerState { handle, sender } = unsafe { LOGGER.take() }.valid();
		unwrap(sender.send(Message::Close), "failed to close system");
		task::block_on(handle);
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
static POSTMORTEM: sync::Mutex<Vec<Box<dyn FnOnce() + Send + 'static>>> = sync::Mutex::new(vec![]);

struct LoggerState {
	handle: Task<()>,
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

async fn check_order() -> Unblock<sync::io::Stdout> {
	panic!("E| No logger! Add 'LOGGER!(logging::Term, INFO);' as first line in main()");
}
