use crate::uses::Async::{chan::*, fs, pre::*, sync::Once, task::*};
use crate::uses::*;
use path::PathBuf;

pub mod Save {
	use super::*;

	pub fn Write<P: Into<PathBuf>>(p: P, data: impl Into<Vec<u8>>) {
		let sender = setup_impl();
		EXPECT!(sender.try_send((p.into(), MessageType::Write, data.into())));
	}
	pub fn Append<P: Into<PathBuf>>(p: P, data: impl Into<Vec<u8>>) {
		let sender = setup_impl();
		EXPECT!(sender.try_send((p.into(), MessageType::Append, data.into())));
	}
	pub fn Archive(args: impl CompressArgs) {
		let (p, data, level) = args.get();
		let sender = setup_impl();
		EXPECT!(sender.try_send((p, MessageType::ComprW(level), data)));
	}

	type Args = (PathBuf, Vec<u8>, i32);
	pub trait CompressArgs {
		fn get(self) -> Args;
	}
	impl<T: Into<Vec<u8>>, F: Into<PathBuf>, C> CompressArgs for (F, T, C)
	where
		i32: Cast<C>,
	{
		fn get(self) -> Args {
			(self.0.into(), self.1.into(), i32::to(self.2))
		}
	}
	impl<T: Into<Vec<u8>>, F: Into<PathBuf>> CompressArgs for (F, T) {
		fn get(self) -> Args {
			(self.0.into(), self.1.into(), 0)
		}
	}

	enum MessageType {
		Write,
		Append,
		ComprW(i32),
		Close,
	}
	type Message = (PathBuf, MessageType, Vec<u8>);
	fn setup_impl() -> &'static Sender<Message> {
		static INIT: Once = Once::new();
		static mut SENDER: Option<Sender<Message>> = None;
		INIT.call_once(move || {
			let (sender, reciever): (Sender<Message>, Receiver<Message>) = chan::unbounded();
			let handle = task::spawn(async move {
				while let Ok(msg) = reciever.recv().await {
					use MessageType::*;
					let (name, operation, data) = msg;
					let file = match operation {
						Write | ComprW(_) => fs::File::create(&name).await,
						Append => fs::OpenOptions::new().append(true).create(true).open(&name).await,
						Close => return,
					};

					if let Ok(mut file) = file {
						let data = if let ComprW(l) = operation {
							OR_DEF!(Res::to(unblock(move || zstd::stream::encode_all(&data[..], l)).await))
						} else {
							data
						};

						let _ = OR_DEF!(file.write_all(&data).await);
						EXPECT!(file.sync_all().await);
					} else {
						FAILED!(map_err(file, &name));
					}
				}
			});

			logging::Logger::AddPostmortem(move || {
				task::block_on(async move {
					EXPECT!(setup_impl().try_send((PathBuf::new(), MessageType::Close, vec![])));
					handle.await
				});
			});

			unsafe { SENDER = Some(sender) };
		});

		&unsafe { &SENDER }.as_ref().unwrap_or_else(|| ASSERT!(false, "File loader failed"))
	}
}

pub mod Load {
	use super::*;
	pub fn File<P: AsRef<Path>>(p: P) -> Res<Vec<u8>> {
		let p: &Path = p.as_ref();
		map_err(std::fs::read(p), p)
	}
	pub fn Text<P: AsRef<Path>>(p: P) -> Res<String> {
		let p: &Path = p.as_ref();
		map_err(std::fs::read_to_string(p), p)
	}
	pub fn Archive<P: AsRef<Path>>(p: P) -> Res<Vec<u8>> {
		let decode = |p| -> Res<_> {
			let f = PASS!(std::fs::File::open(p));
			let b = PASS!(zstd::stream::decode_all(f));
			Ok(b)
		};
		let p = p.as_ref();
		map_err(decode(p), p)
	}
}

pub mod Preload {
	use super::*;
	pub use Archive::load as Archive;
	pub use File::load as File;
	pub use Text::load as Text;
}
macro_rules! LOADER {
	($n: ident, $t: ty, $a: ident, $b: block) => {
		pub mod $n {
			use {super::*, Resource::*};
			#[derive(Debug)]
			pub enum Resource {
				Loading(Task<Res<$t>>),
				Done($t),
			}
			impl Resource {
				pub fn if_ready(&mut self) -> Option<&mut $t> {
					match self {
						Done(vec) => Some(vec),
						Loading(handle) => {
							let res = task::block_on(async move { task::poll_once(handle).await });
							if res.is_none() {
								return None;
							}
							*self = Done(OR_DEF!(res.unwrap()));
							self.if_ready()
						}
					}
				}
				pub fn check(&mut self) -> Res<&mut $t> {
					match self {
						Done(vec) => Ok(vec),
						Loading(handle) => {
							let res = task::block_on(async move { handle.await });
							*self = Done(res?);
							self.check()
						}
					}
				}
				pub fn get(&mut self) -> &mut $t {
					match self {
						Done(vec) => vec,
						Loading(handle) => {
							let res = task::block_on(async move { handle.await });
							*self = Done(OR_DEF!(res));
							self.get()
						}
					}
				}
				pub fn take(self) -> $t {
					match self {
						Done(vec) => vec,
						Loading(handle) => OR_DEF!(task::block_on(async move { handle.await })),
					}
				}
			}
			pub fn load<P: Into<PathBuf>>(p: P) -> $n::Resource {
				let $a = p.into();
				Resource::Loading(task::spawn(async move { Res::to($b) }))
			}
		}
	};
}
LOADER!(File, Vec<u8>, s, { read_file(&s).await });
LOADER!(Text, String, s, { read_text(&s).await });
LOADER!(Archive, Vec<u8>, s, {
	let data = PASS!(read_file(&s).await);
	unblock(move || zstd::stream::decode_all(&data[..])).await
});

pub async fn read_file<P: AsRef<Path>>(p: P) -> Res<Vec<u8>> {
	async fn read(p: &Path) -> Res<Vec<u8>> {
		let mut f = PASS!(fs::File::open(p).await);
		let mut b = vec![];
		PASS!(f.read_to_end(&mut b).await);
		Ok(b)
	};
	let p = p.as_ref();
	map_err(read(p).await, p)
}
pub async fn read_text<P: AsRef<Path>>(p: P) -> Res<String> {
	async fn read(p: &Path) -> Res<String> {
		let mut f = PASS!(fs::File::open(p).await);
		let mut b = String::new();
		PASS!(f.read_to_string(&mut b).await);
		Ok(b)
	};
	let p = p.as_ref();
	map_err(read(p).await, p)
}

fn map_err<T, E: std::fmt::Display>(r: Result<T, E>, p: &Path) -> Res<T> {
	let r = Res::to(r);
	r.map_err(|e| format!("Could not open file '{:?}'\nErr: {}", p, e))
}
