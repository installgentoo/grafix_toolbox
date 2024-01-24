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
		Self::logger(out, l);
		Self
	}
	pub fn log(l: Level, msg: String) {
		if (l as i32) <= Self::level() {
			Self::logger(check_order, Level::INFO)
				.get()
				.expect("E| Logger already exited")
				.sender
				.send(Message::M(msg))
				.expect("E| Failed to send log");
		}
	}
	pub fn add_postmortem(f: impl FnOnce() + 'static) {
		Self::logger(check_order, Level::INFO)
			.get()
			.expect("E| Logger already exited")
			.postmortem
			.lock()
			.unwrap()
			.push(Box(f));
	}

	pub fn level() -> i32 {
		unsafe { LEVEL as i32 }
	}
	pub fn set_level(l: Level) {
		unsafe { LEVEL = l }
	}
	fn logger<T, F>(out: impl FnOnce() -> F + SendStat, l: Level) -> &'static mut OnceLock<LoggerState>
	where
		T: AsyncWrite + Unpin + Send,
		F: Future<Output = T> + Send,
	{
		static mut LOGGER: OnceLock<LoggerState> = OnceLock::new();
		unsafe {
			LEVEL = l;
			LOGGER.get_or_init(move || {
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
				LoggerState {
					handle,
					sender,
					postmortem: Def(),
				}
			});
			&mut LOGGER
		}
	}
}
impl Drop for Logger {
	fn drop(&mut self) {
		let s = Self::logger(check_order, Level::INFO);
		s.get().expect("E| Logger already exited").postmortem.lock().unwrap().drain(..).for_each(|f| f());

		let LoggerState { handle, sender, .. } = s.take().unwrap();
		sender.send(Message::Close).expect("E| Failed to close log");
		task::block_on(handle)
	}
}

#[derive(Clone, Copy)]
pub enum Level {
	PRINT = 0,
	WARNING = 1,
	INFO = 2,
	DEBUG = 3,
}
static mut LEVEL: Level = Level::INFO;

struct LoggerState {
	handle: Task<()>,
	sender: Sender<Message>,
	postmortem: sync::Mutex<Vec<Box<dyn FnOnce() + 'static>>>,
}

enum Message {
	M(String),
	Close,
}

async fn check_order() -> Unblock<sync::io::Stdout> {
	panic!("E| No logger! Add 'LOGGER!(logging::Term, INFO);' as first line in main()");
}
