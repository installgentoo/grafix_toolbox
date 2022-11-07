pub use crate::kit::policies::casts::result::{UniformUnwrap, UniformUnwrapOrDefault};
use crate::uses::asyn::{fs::File, pre::*, *};
use crate::uses::{sync::io, *};

pub async fn Term() -> Unblock<io::Stdout> {
	Unblock::new(io::stdout())
}

pub async fn TermErr() -> Unblock<io::Stderr> {
	Unblock::new(io::stderr())
}

pub async fn File() -> File {
	File::create("log.txt").await.expect("E| Couldn't create log file")
}

pub async fn Null() -> Unblock<io::Sink> {
	Unblock::new(io::sink())
}

#[derive(Clone, Copy)]
pub enum Level {
	ERROR = 0,
	WARNING = 1,
	INFO = 2,
	DEBUG = 3,
}

pub struct Logger;
impl Logger {
	pub fn Setup<T, Fut, Fun>(out: Fun, l: Level) -> Logger
	where
		T: AsyncWrite + Unpin + Send,
		Fut: Future<Output = T> + Send,
		Fun: FnOnce() -> Fut + Send + 'static,
	{
		Self::setup_impl(out, l);
		Logger {}
	}
	pub fn Log(msg: String) {
		Self::setup_impl(EnsureOrder, Level::INFO);
		unsafe { &LOGGER }
			.as_ref()
			.expect("E| Logger not initialized")
			.sender
			.send(Message::M(msg))
			.expect("E| failed to send into log channel");
	}
	pub fn AddPostmortem(f: impl FnOnce() + 'static) {
		Self::setup_impl(EnsureOrder, Level::INFO);
		unsafe { &LOGGER }
			.as_ref()
			.expect("E| Logger not initialized")
			.postmortem
			.lock()
			.unwrap()
			.get_mut()
			.push(Box::new(f));
	}

	pub fn level() -> i32 {
		unsafe { LEVEL as i32 }
	}
	fn setup_impl<T, Fut, Fun>(out: Fun, l: Level)
	where
		T: AsyncWrite + Unpin + Send,
		Fut: Future<Output = T> + Send,
		Fun: FnOnce() -> Fut + Send + 'static,
	{
		static INIT: Once = Once::new();
		INIT.call_once(move || {
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
			let postmortem = Mutex::new(Cell::new(vec![]));

			unsafe {
				LEVEL = l;
				LOGGER = Some(LoggerState { handle, sender, postmortem })
			}
		});
	}
}
impl Drop for Logger {
	fn drop(&mut self) {
		unsafe { &LOGGER }
			.as_ref()
			.expect("E| Logger not initialized")
			.postmortem
			.lock()
			.unwrap()
			.take()
			.into_iter()
			.for_each(|f| f());

		let LoggerState { handle, sender, .. } = unsafe { LOGGER.take() }.expect("E| You should only have one Logger");
		sender.send(Message::Close).expect("E| failed to send close into log channel");
		task::block_on(async move { handle.await })
	}
}

async fn EnsureOrder() -> Unblock<io::Stdout> {
	panic!("E| Logger called before it was set up");
}

static mut LEVEL: Level = Level::INFO;
static mut LOGGER: Option<LoggerState> = None;

struct LoggerState {
	handle: Task<()>,
	sender: Sender<Message>,
	postmortem: Mutex<Cell<Vec<Box<dyn FnOnce() + 'static>>>>,
}

enum Message {
	M(String),
	Close,
}
