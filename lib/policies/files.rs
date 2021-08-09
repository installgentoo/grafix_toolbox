use crate::uses::asyn::{pre::*, *};
use crate::uses::*;
use path::PathBuf;

pub mod Save {
	use super::*;

	pub fn Write(p: impl Into<PathBuf>, data: impl Into<Vec<u8>>) {
		let sender = setup_impl();
		EXPECT!(sender.send((p.into(), MessageType::Write, data.into())));
	}
	pub fn Append(p: impl Into<PathBuf>, data: impl Into<Vec<u8>>) {
		let sender = setup_impl();
		EXPECT!(sender.send((p.into(), MessageType::Append, data.into())));
	}
	pub fn Archive(args: impl CompressArgs) {
		let (p, data, level) = args.get();
		let sender = setup_impl();
		EXPECT!(sender.send((p, MessageType::ComprW(level), data)));
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
			(self.0.into(), self.1.into(), i32(self.2))
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
				while let Ok(msg) = reciever.recv_async().await {
					let disk = task::spawn(async move {
						use MessageType::*;
						let (name, operation, data) = msg;
						let file = match operation {
							Write | ComprW(_) => fs::File::create(&name).await,
							Append => fs::OpenOptions::new().append(true).create(true).open(&name).await,
							Close => return false,
						};

						if let Ok(mut file) = file {
							let data = if let ComprW(l) = operation {
								OR_DEF!(Res(zstd::stream::encode_all(&data[..], l)))
							} else {
								data
							};

							let _ = OR_DEF!(file.write_all(&data).await);
							EXPECT!(file.sync_all().await);
						} else {
							FAILED!(map_err(file, &name));
						}
						true
					});
					if !disk.await {
						break;
					}
				}
			});

			logging::Logger::AddPostmortem(move || {
				EXPECT!(setup_impl().send((PathBuf::new(), MessageType::Close, vec![])));
				task::block_on(async move { handle.await });
			});

			unsafe { SENDER = Some(sender) };
		});

		unsafe { &SENDER }.as_ref().unwrap_or_else(|| ASSERT!(false, "File loader failed"))
	}
}

pub mod Load {
	use super::*;
	pub fn File(p: impl AsRef<Path>) -> Res<Vec<u8>> {
		let p: &Path = p.as_ref();
		map_err(std::fs::read(p), p)
	}
	pub fn Text(p: impl AsRef<Path>) -> Res<String> {
		let p: &Path = p.as_ref();
		map_err(std::fs::read_to_string(p), p)
	}
	pub fn Archive(p: impl AsRef<Path>) -> Res<Vec<u8>> {
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
	($type: ident, $ret: ty, $a: ident, $b: block) => {
		pub mod $type {
			use {super::*, Resource::*};
			#[derive(Debug)]
			pub enum Resource {
				Loading(Task<Res<$ret>>),
				Done($ret),
			}
			impl Resource {
				pub fn if_ready(&mut self) -> Option<&mut $ret> {
					match self {
						Done(vec) => Some(vec),
						Loading(handle) => {
							let res = task::block_on(async move { task::poll_once(handle).await })?;
							*self = Done(OR_DEF!(res));
							self.if_ready()
						}
					}
				}
				pub fn check(&mut self) -> Res<&mut $ret> {
					match self {
						Done(vec) => Ok(vec),
						Loading(handle) => {
							let res = task::block_on(async move { handle.await });
							*self = Done(res?);
							self.check()
						}
					}
				}
				pub fn get(&mut self) -> &mut $ret {
					match self {
						Done(vec) => vec,
						Loading(handle) => {
							let res = task::block_on(async move { handle.await });
							*self = Done(OR_DEF!(res));
							self.get()
						}
					}
				}
				pub fn take(self) -> $ret {
					match self {
						Done(vec) => vec,
						Loading(handle) => OR_DEF!(task::block_on(async move { handle.await })),
					}
				}
			}
			pub fn load(p: impl Into<PathBuf>) -> $type::Resource {
				let $a = p.into();
				Resource::Loading(task::spawn(async move { Res($b) }))
			}
		}
	};
}
LOADER!(File, Vec<u8>, s, { read_file(&s).await });
LOADER!(Text, String, s, { read_text(&s).await });
LOADER!(Archive, Vec<u8>, s, {
	let data = PASS!(read_file(&s).await);
	zstd::stream::decode_all(&data[..])
});

pub async fn read_file(p: impl AsRef<Path>) -> Res<Vec<u8>> {
	async fn read(p: &Path) -> Res<Vec<u8>> {
		let mut f = PASS!(fs::File::open(p).await);
		let mut b = vec![];
		PASS!(f.read_to_end(&mut b).await);
		Ok(b)
	}
	let p = p.as_ref();
	map_err(read(p).await, p)
}
pub async fn read_text(p: impl AsRef<Path>) -> Res<String> {
	async fn read(p: &Path) -> Res<String> {
		let mut f = PASS!(fs::File::open(p).await);
		let mut b = String::new();
		PASS!(f.read_to_string(&mut b).await);
		Ok(b)
	}
	let p = p.as_ref();
	map_err(read(p).await, p)
}

fn map_err<T>(r: Result<T, impl std::fmt::Display>, p: &Path) -> Res<T> {
	r.map_err(|e| format!("Could not open file {:?}\n{}", p, e))
}
