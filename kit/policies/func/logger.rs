pub use super::result::{UniformUnwrap, UniformUnwrapOrDefault};
pub use text_color::term::*;

pub struct LogBuf<T> {
	buff: Arc<Mutex<Vec<u8>>>,
	out: T,
}
impl<T> LogBuf<T> {
	pub fn new(buff: Arc<Mutex<Vec<u8>>>, out: T) -> Self {
		Self { buff, out }
	}
}
impl<T: Writable> AsyncWrite for LogBuf<T> {
	fn poll_write(self: Pin<&mut Self>, c: &mut Context, buf: &[u8]) -> Poll<Result<usize, io::Error>> {
		let s = self.get_mut();
		let wrote = Pin::new(&mut s.out).poll_write(c, buf);
		if let Poll::Ready(Ok(wrote)) = wrote {
			s.buff.lock().extend_from_slice(&buf[..wrote]);
		}
		wrote
	}
	fn poll_flush(self: Pin<&mut Self>, c: &mut Context) -> Poll<Result<(), io::Error>> {
		Pin::new(&mut self.get_mut().out).poll_flush(c)
	}
	fn poll_shutdown(self: Pin<&mut Self>, c: &mut Context) -> Poll<Result<(), io::Error>> {
		Pin::new(&mut self.get_mut().out).poll_shutdown(c)
	}
}
use std::{pin::Pin, task::Context, task::Poll};

pub fn Term() -> io::Stdout {
	io::stdout()
}

pub fn TermErr() -> io::Stderr {
	io::stderr()
}

pub fn File() -> fs::File {
	term_color::disable();
	let f = unwrap(std::fs::File::create("log.txt"), "couldn't create log file");
	fs::File::from_std(f)
}

pub fn Null() -> io::Sink {
	io::sink()
}

pub struct Logger;
impl Logger {
	pub fn shutdown_hook(f: impl FnOnce() + SendS) {
		POSTMORTEM.lock().push(Box(f));
	}
	pub fn init<O: Writable>(out: impl FnOnce() -> O + SendS, l: Level) -> Self {
		Self::init_log(out, l);
		Self
	}
	pub fn log(msg: String) {
		let log = Self::init_log(check_order, Level::INFO).pipe(|l| unwrap_o(l.get(), "already exited"));
		unwrap(log.sender.send(Message::M(msg)), "send failed");
	}
	#[inline(always)]
	pub fn level() -> i32 {
		unsafe { LEVEL as i32 }
	}
	pub fn set_level(l: Level) {
		unsafe { LEVEL = l }
	}
	fn init_log<O: Writable>(out: impl FnOnce() -> O + SendS, l: Level) -> &'static Log {
		static L: Log = OnceLock::new();
		L.get_or_init(move || {
			std::panic::set_hook(Box(|i| {
				let (bt, p, P) = (
					Backtrace::force_capture().pipe(process_backtrace),
					i.payload_as_str().unwrap_or("???").red(),
					"P|".red().bold(),
				);

				Logger::log(format!(
					"{P} {bt}{P} {p} |{}|{}\n",
					i.location().map_or("???".into(), |s| format!("{s}")),
					thread::current().name().unwrap_or("???")
				))
			}));
			unsafe { LEVEL = l };
			let (sender, mut rx) = chan::unbounded::<Message>();
			let writer = task::Runtime()
				.spawn(async move || {
					let mut out = out();
					while let Some(Message::M(msg)) = rx.recv().await {
						unwrap(out.write_all(msg.as_bytes()).await, "write failed");
					}
					unwrap(out.flush().await, "flush failed")
				})
				.into();
			LogState { writer, sender }
		});
		&L
	}
}
impl Drop for Logger {
	fn drop(&mut self) {
		let SELF = Self::init_log(check_order, Level::INFO);
		POSTMORTEM.lock().drain(..).for_each(|f| f());
		let LogState { writer, sender } = SELF.get().valid();
		unwrap(sender.send(Message::Close), "failed to close system");
		task::Runtime().finish(writer.replace(Task::new_uninit()));
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
static POSTMORTEM: Mutex<Vec<Box<dyn FnOnce() + Send>>> = Mutex::new(vec![]);

type Log = OnceLock<LogState>;
struct LogState {
	writer: Cell<Task<()>>,
	sender: Sender<Message>,
}
unsafe impl Sync for LogState {}

enum Message {
	M(String),
	Close,
}

pub fn process_backtrace(bt: Backtrace) -> String {
	let ignore = [
		"          at /rustc/",
		".rustup/toolchains",
		"kit/policies/func/log",
		": std::",
		": core::",
		": alloc::",
		": rust_begin_unwind",
		": __libc_start_",
		": _start",
		"::{{closure}}",
		"core::ops::function::",
		"::logger::Logger::init_log::",
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
		.fold("BACKTRACE\n".into(), |a, l| a + l + "\n");
	bt
}

fn unwrap<T, E: Debug>(v: Result<T, E>, msg: &str) -> T {
	v.unwrap_or_else(|e| panic!("{}Logger panic: {msg} {e:?}", Backtrace::force_capture().pipe(process_backtrace)))
}
fn unwrap_o<T>(v: Option<T>, msg: &str) -> T {
	v.unwrap_or_else(|| panic!("{}Logger panic: {msg}", Backtrace::force_capture().pipe(process_backtrace)))
}
fn check_order() -> io::Stdout {
	panic!("E| No logger! Add 'LOGGER!(logging::Term, INFO);' as first line in main()");
}

trait_alias!(pub Writable, AsyncWrite + Unpin + Send);

use crate::{asyn::*, lib::*, text_color};
use std::backtrace::Backtrace;
