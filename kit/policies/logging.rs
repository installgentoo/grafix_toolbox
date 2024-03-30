pub use super::result::{UniformUnwrap, UniformUnwrapOrDefault};
use super::{derives::*, ext::*};
use crate::{asyn::*, sync};

pub async fn Term() -> Unblock<sync::io::Stdout> {
	Unblock::new(sync::io::stdout())
}

pub async fn TermErr() -> Unblock<sync::io::Stderr> {
	Unblock::new(sync::io::stderr())
}

pub async fn File() -> fs::File {
	fs::File::create("log.txt").await.expect("E| Couldn't create log file")
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

		unsafe { LOGGER.get() }
			.expect("E| Logger already exited")
			.sender
			.send(Message::M(msg))
			.expect("E| Failed to send log");
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
				LEVEL = l;
				let (sender, reciever) = chan::unbounded::<Message>();
				let handle = task::spawn(async move {
					let mut out = out().await;
					while let Ok(msg) = reciever.recv_async().await {
						if let Message::M(msg) = msg {
							out.write_all(msg.as_bytes()).await.expect("E| Failed log write");
						} else {
							out.flush().await.expect("E| Failed log flush");
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
		sender.send(Message::Close).expect("E| Failed to close logging system");
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

async fn check_order() -> Unblock<sync::io::Stdout> {
	panic!("E| No logger! Add 'LOGGER!(logging::Term, INFO);' as first line in main()");
}
