use crate::{asyn::*, lib::*};
use std::path::{Path, PathBuf};

pub mod Save {
	use super::*;

	pub fn Write(p: impl Into<Astr>, data: impl Into<Arc<[u8]>>) {
		sender().send((p.into(), MessageType::Write, data.into())).expect(FAILED_WRITE);
	}
	pub fn Append(p: impl Into<Astr>, data: impl Into<Arc<[u8]>>) {
		sender().send((p.into(), MessageType::Append, data.into())).expect(FAILED_WRITE);
	}
	pub fn Archive(args: impl CompressArgs) {
		let (p, data, level) = args.get();
		sender().send((p, MessageType::ComprW(level), data)).expect(FAILED_WRITE);
	}

	type Args = (Astr, Arc<[u8]>, i32);
	pub trait CompressArgs {
		fn get(self) -> Args;
	}
	impl<T: Into<Arc<[u8]>>, F: Into<Astr>, C> CompressArgs for (F, T, C)
	where
		i32: Cast<C>,
	{
		fn get(self) -> Args {
			(self.0.into(), self.1.into(), i32(self.2))
		}
	}
	impl<T: Into<Arc<[u8]>>, F: Into<Astr>> CompressArgs for (F, T) {
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
	type Message = (Astr, MessageType, Arc<[u8]>);
	fn sender() -> &'static Sender<Message> {
		static SENDER: OnceLock<Sender<Message>> = OnceLock::new();
		SENDER.get_or_init(move || {
			let (sn, rx) = chan::unbounded::<Message>();
			let handle = task::spawn(async move {
				while let Ok(msg) = rx.recv_async().await {
					let disk = task::spawn(async move {
						use MessageType::*;
						let (name, operation, data) = msg;
						let file = match operation {
							Write | ComprW(_) => fs::File::create(&*name).await,
							Append => fs::OpenOptions::new().append(true).create(true).open(&*name).await,
							Close => return false,
						};

						if let Ok(mut file) = file {
							let data = if let ComprW(l) = operation {
								OR_DEFAULT!(zstd::stream::encode_all(&data[..], l)).into()
							} else {
								data
							};

							let _ = file.write_all(&data).await;
							EXPECT!(file.sync_all().await);
						} else {
							let name: PathBuf = (*name).into();
							FAIL!("{:?}", fmt_err(file, &name));
						}
						true
					});
					if !disk.await {
						break;
					}
				}
			});

			logging::Logger::add_postmortem(move || {
				sender().send(("".into(), MessageType::Close, vec![].into())).expect("E| Failed to close write");
				task::block_on(handle);
			});

			sn
		})
	}
}

pub mod Load {
	use super::*;
	pub fn File(p: impl AsRef<Path>) -> Res<Vec<u8>> {
		let p = p.as_ref();
		fmt_err(std::fs::read(p), p)
	}
	pub fn Text(p: impl AsRef<Path>) -> Res<String> {
		let p = p.as_ref();
		fmt_err(std::fs::read_to_string(p), p)
	}
	pub fn Archive(p: impl AsRef<Path>) -> Res<Vec<u8>> {
		let decode = |p| -> Res<_> {
			let f = Res(std::fs::File::open(p))?;
			let b = Res(zstd::stream::decode_all(f))?;
			Ok(b)
		};
		let p = p.as_ref();
		fmt_err(decode(p), p)
	}
}

pub mod Lazy {
	use super::*;
	pub fn File(p: impl Into<Astr>) -> impl Stream<Item = Vec<u8>> {
		lazy_read(p, read_file)
	}
	pub fn Text(p: impl Into<Astr>) -> impl Stream<Item = String> {
		lazy_read(p, read_text)
	}
	pub fn Archive(p: impl Into<Astr>) -> impl Stream<Item = Vec<u8>> {
		let p = p.into();
		lazy_read(p.clone(), read_file).map(move |data| OR_DEFAULT!(zstd::stream::decode_all(&data[..]), "Failed to decode archive {p:?}: {}"))
	}
}

pub mod Watch {
	use super::*;
	pub fn File(p: impl Into<Astr>) -> impl Stream<Item = Vec<u8>> {
		watch_file(p, read_file)
	}
	pub fn Text(p: impl Into<Astr>) -> impl Stream<Item = String> {
		watch_file(p, read_text)
	}
	pub fn Archive(p: impl Into<Astr>) -> impl Stream<Item = Vec<u8>> {
		let p = p.into();
		watch_file(p.clone(), read_file).map(move |data| OR_DEFAULT!(zstd::stream::decode_all(&data[..]), "Failed to decode archive {p:?}: {}"))
	}
}

fn lazy_read<T: Default, F: Future<Output = Res<T>>>(p: impl Into<Astr>, loader: impl FnOnce(Arc<Path>) -> F) -> impl Stream<Item = T> {
	stream::once_future(async move {
		let p = PathBuf::from(&*p.into()).into();
		match loader(p).await {
			Ok(file) => file,
			e @ Err(_) => OR_DEFAULT!(e),
		}
	})
}

fn watch_file<T, F: Future<Output = Res<T>>>(p: impl Into<Astr>, loader: impl FnOnce(Arc<Path>) -> F + Clone) -> impl Stream<Item = T> {
	let (sn, rx) = chan::bounded::<()>(1);
	let p: Arc<Path> = PathBuf::from(&*p.into()).into();

	stream::unfold(None, move |w| {
		let (p, l, _sn, rx) = (p.clone(), loader.clone(), sn.clone(), rx.clone());
		async move {
			if let Some(_w) = w {
				rx.recv_async().await.ok().sink();
			}

			let w = {
				#[cfg(feature = "fsnotify")]
				{
					use notify::*;
					let p = p.clone();
					let mut w = {
						let p = p.clone();
						Res(recommended_watcher(move |r| match r {
							Ok(_) => _sn.try_send(()).ok().sink(),
							Err(e) => FAIL!("File {p:?}: {e}"),
						}))
					}
					.map_err(|e| FAIL!("Watch {p:?}: {e}"))
					.ok();

					w.as_mut().map(|w| w.watch(&p, RecursiveMode::NonRecursive).unwrap_or_else(|_| FAIL!("Cannot watch {p:?}")));
					Some(w)
				}
				#[cfg(not(feature = "fsnotify"))]
				{
					FAIL!("Enable fsnotify feature to watch files");
					Some(())
				}
			};

			let file = l(p).await.map_err(|e| FAIL!(e));
			file.ok().map(|f| (f, w))
		}
	})
}

async fn read_file(p: Arc<Path>) -> Res<Vec<u8>> {
	async fn read(p: &Path) -> Res<Vec<u8>> {
		let (mut f, mut b) = (Res(fs::File::open(p).await)?, vec![]);
		Res(f.read_to_end(&mut b).await)?;
		Ok(b)
	}
	fmt_err(read(&p).await, &p)
}
async fn read_text(p: Arc<Path>) -> Res<String> {
	async fn read(p: &Path) -> Res<String> {
		let (mut f, mut b) = (Res(fs::File::open(p).await)?, String::new());
		Res(f.read_to_string(&mut b).await)?;
		Ok(b)
	}
	fmt_err(read(&p).await, &p)
}

fn fmt_err<T>(r: Result<T, impl std::fmt::Display>, p: &Path) -> Res<T> {
	r.map_err(|e| format!("Could not open file {p:?} - {e}"))
}

const FAILED_WRITE: STR = "E| Failed to send write";

#[cfg(not(feature = "zstd"))]
mod zstd {
	pub mod stream {
		use super::super::*;
		pub fn encode_all(s: &[u8], _: i32) -> Res<Vec<u8>> {
			Ok(s.to_vec())
		}
		pub fn decode_all<R: std::io::Read>(mut s: R) -> Res<Vec<u8>> {
			let mut b = vec![];
			Res(s.read_to_end(&mut b))?;
			Ok(b)
		}
	}
}
