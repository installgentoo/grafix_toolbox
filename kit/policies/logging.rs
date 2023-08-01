pub use crate::kit::policies::casts::result::{UniformUnwrap, UniformUnwrapOrDefault};
use crate::uses::{asyn::*, sync::io, *};

pub async fn Term() -> Unblock<io::Stdout> {
	Unblock::new(io::stdout())
}

pub async fn TermErr() -> Unblock<io::Stderr> {
	Unblock::new(io::stderr())
}

pub async fn File() -> fs::File {
	fs::File::create("log.txt").await.expect("E| Couldn't create log file")
}

pub async fn Null() -> Unblock<io::Sink> {
	Unblock::new(io::sink())
}

pub struct Logger;
impl Logger {
	pub fn new<T, Fut, Fun>(out: Fun, l: Level) -> Self
	where
		T: AsyncWrite + Unpin + Send,
		Fut: Future<Output = T> + Send,
		Fun: FnOnce() -> Fut + Send + 'static,
	{
		Self::logger(out, l);
		Self
	}
	pub fn Log(l: Level, msg: String) {
		if (l as i32) <= Self::level() {
			Self::logger(CheckOrder, Level::INFO)
				.get()
				.expect("E| Logger already exited")
				.sender
				.send(Message::M(msg))
				.expect("E| Failed to send log");
		}
	}
	pub fn AddPostmortem(f: impl FnOnce() + 'static) {
		Self::logger(CheckOrder, Level::INFO)
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
	fn logger<T, Fut, Fun>(out: Fun, l: Level) -> &'static mut OnceLock<LoggerState>
	where
		T: AsyncWrite + Unpin + Send,
		Fut: Future<Output = T> + Send,
		Fun: FnOnce() -> Fut + Send + 'static,
	{
		static mut LOGGER: OnceLock<LoggerState> = OnceLock::new();
		unsafe {
			LEVEL = l;
			LOGGER.get_or_init(move || {
				let (sender, reciever): (Sender<Message>, Receiver<Message>) = chan::unbounded();
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
		let s = Self::logger(CheckOrder, Level::INFO);
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
	postmortem: Mutex<Vec<Box<dyn FnOnce() + 'static>>>,
}

enum Message {
	M(String),
	Close,
}

async fn CheckOrder() -> Unblock<io::Stdout> {
	panic!("E| No logger! Add 'LOGGER!(logging::Term, INFO);' as first line in main()");
}
