use crate::{asyn::*, lib::*};
use std::path::{Path, PathBuf};

pub mod Save {
	use super::*;

	pub fn Write(p: impl Into<Astr>, data: impl Into<Arc<[u8]>>) {
		sender().send((p.into(), MessageType::Write, data.into())).valid();
	}
	pub fn Append(p: impl Into<Astr>, data: impl Into<Arc<[u8]>>) {
		sender().send((p.into(), MessageType::Append, data.into())).valid();
	}
	pub fn Archive(args: impl CompressArgs) {
		let (p, data, level) = args.get();
		sender().send((p, MessageType::ComprW(level), data)).valid();
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
			(self.0.into(), self.1.into(), i32(self.2).clamp(1, 22))
		}
	}
	impl<T: Into<Arc<[u8]>>, F: Into<Astr>> CompressArgs for (F, T) {
		fn get(self) -> Args {
			(self.0.into(), self.1.into(), 1)
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
		use MessageType::*;
		static SENDER: OnceLock<Sender<Message>> = OnceLock::new();
		SENDER.get_or_init(|| {
			let (sn, mut rx) = chan::unbounded::<Message>();
			let writer = task::Runtime().spawn(async move || {
				while let Some(msg) = rx.recv().await {
					let (name, operation, data) = msg;
					let file = match operation {
						Close => break,
						Write | ComprW(_) => fs::File::create(&*name).await,
						Append => fs::OpenOptions::new().append(true).create(true).open(&*name).await,
					};

					let Ok(mut file) = file else {
						let name: PathBuf = (*name).into();
						FAIL!({ continue }, "{:?}", fmt_err(file, &name));
					};

					let data = if let ComprW(l) = operation {
						task::spawn_blocking(move || zstd::encode_all(&data[..], l))
							.await
							.fail()
							.explain_err(|| format!("Cannot encode file {name:?}"))
							.warn()
							.into()
					} else {
						data
					};

					file.write_all(&data).await.explain_err(|| format!("Cannot write {name:?}")).warn();
					file.sync_all().await.explain_err(|| format!("Cannot sync {name:?}")).warn();
				}
			});

			logger::Logger::shutdown_hook({
				let sn = sn.clone();
				move || {
					sn.send(("".into(), Close, Def())).expect("E| Cannot close async write system");
					task::Runtime().finish(writer);
				}
			});

			sn
		})
	}
}

pub mod Load {
	use {super::*, std::fs};
	pub fn File(p: impl AsRef<Path>) -> Res<Vec<u8>> {
		p.as_ref().pipe(|p| fmt_err(fs::read(p), p))
	}
	pub fn Text(p: impl AsRef<Path>) -> Res<String> {
		p.as_ref().pipe(|p| fmt_err(fs::read_to_string(p), p))
	}
	pub fn Archive(p: impl AsRef<Path>) -> Res<Vec<u8>> {
		p.as_ref().pipe(|p| fmt_err(fs::File::open(p).and_then(zstd::decode_all), p))
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
		lazy_read(p.clone(), read_file).map(move |data| data.pipe_as(zstd::decode_all).explain_err(|| format!("Cannot decode archive {p:?}")).warn())
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
		watch_file(p.clone(), read_file).map(move |data| data.pipe_as(zstd::decode_all).explain_err(|| format!("Cannot decode archive {p:?}")).warn())
	}
}

fn lazy_read<T: Default>(p: impl Into<Astr>, loader: impl AsyncFnOnce(APath) -> Res<T>) -> impl Stream<Item = T> {
	stream::once(async {
		let p = p.into().pipe_as(PathBuf::from).into();
		loader(p).await.warn()
	})
}

fn watch_file<T>(p: impl Into<Astr>, loader: impl AsyncFnOnce(APath) -> Res<T> + Clone) -> impl Stream<Item = T> {
	let rx = Notify::new().pipe(Arc);
	let p: APath = p.into().pipe_as(PathBuf::from).into();

	stream::unfold(None, move |w| {
		let (p, loader, _sn, rx) = (p.clone(), loader.clone(), rx.clone().pipe(Some), rx.clone());
		async move {
			let first = w.is_none();
			if let Some(_w) = w {
				let _ = rx.notified().await;
				DEBUG!("File {p:?} changed");
			}

			let t = fs::metadata(&p).await.and_then(|m| m.modified());
			let mut _sn = _sn.map(|s| (s, t));

			while !first && !p.exists() {
				task::sleep_ms(100).await;
			}

			#[allow(clippy::redundant_closure)]
			let file = p.clone().pipe(|p| loader(p)).await.map_err(|e| FAIL!(e));
			DEBUG!("File {p:?} loaded");

			let w = {
				#[cfg(feature = "fsnotify")]
				{
					while !first && !p.exists() {
						task::sleep_ms(100).await;
					}
					use notify::*;

					recommended_watcher({
						let p = p.clone();
						move |r| match r {
							Ok(_) => {
								match (_sn.as_ref(), std::fs::metadata(&p).and_then(|m| m.modified())) {
									(Some((_, Ok(mtime))), Ok(t)) if &t == mtime => return,
									_ => (),
								}
								_sn.take().map(|(s, _)| s.notify_one()).sink()
							}
							Err(e) => FAIL!("File {p:?}: {e}"),
						}
					})
					.map_err(|e| FAIL!("Watch {p:?}: {e}"))
					.ok()
					.map(|mut w| {
						w.watch(&p, RecursiveMode::NonRecursive).unwrap_or_else(|_| FAIL!("Cannot watch {p:?}"));
						w
					})
					.pipe(Some)
				}
				#[cfg(not(feature = "fsnotify"))]
				{
					WARN!("Enable fsnotify feature to watch files");
					Some(())
				}
			};

			file.ok().map(|f| (f, w))
		}
	})
}

async fn read_file(p: APath) -> Res<Vec<u8>> {
	async fn read(p: &Path) -> Res<Vec<u8>> {
		let (mut f, mut b) = (fs::File::open(p).await.res()?, vec![]);
		f.read_to_end(&mut b).await.res()?;
		Ok(b)
	}
	fmt_err(read(&p).await, &p)
}
async fn read_text(p: APath) -> Res<String> {
	async fn read(p: &Path) -> Res<String> {
		let (mut f, mut b) = (fs::File::open(p).await.res()?, Def());
		f.read_to_string(&mut b).await.res()?;
		Ok(b)
	}
	fmt_err(read(&p).await, &p)
}

fn fmt_err<T>(r: Result<T, impl Display>, p: &Path) -> Res<T> {
	r.explain_err(|| format!("Cannot open file {p:?}"))
}

type APath = Arc<Path>;

#[cfg(feature = "zstd")]
mod zstd {
	pub use zstd::stream::*;
}
#[cfg(not(feature = "zstd"))]
mod zstd {
	use super::*;
	pub fn encode_all(s: &[u8], _: i32) -> Res<Vec<u8>> {
		s.to_vec().pipe(Ok)
	}
	pub fn decode_all<R: std::io::Read>(mut s: R) -> Result<Vec<u8>, io::Error> {
		let mut b = vec![];
		s.read_to_end(&mut b)?;
		Ok(b)
	}
}
