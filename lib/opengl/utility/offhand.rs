use crate::uses::{sync::chan::*, threads::*, *};
use crate::GL::{window::WindowPolicy, Fence};

pub struct Offhand<O> {
	handle: Option<JoinHandle<Res<()>>>,
	rx: Receiver<(O, Fence)>,
}
impl<O: 'static + Send> Offhand<O> {
	pub fn new<I: 'static + Send, F: 'static + Send + Fn(I) -> O, W: WindowPolicy>(window: &mut W, depth: usize, process: F) -> (Sender<I>, Self) {
		let (data_sn, data_rx): (Sender<I>, Receiver<I>) = chan::bounded(depth);
		let (res_sn, res_rx): (Sender<(O, Fence)>, Receiver<(O, Fence)>) = chan::bounded(depth);
		let handle = window.spawn_offhand_gl(move || {
			while let Ok(msg) = data_rx.recv() {
				let res = process(msg);
				let _ = res_sn.send((res, Fence::new()));
			}
		});
		let (handle, rx) = (Some(handle), res_rx);
		(data_sn, Self { handle, rx })
	}
	pub fn recv(&self) -> OffhandRes<O> {
		self.rx.recv().ok().map_or(OffhandRes(None), |r| OffhandRes(Some(r)))
	}
	pub fn try_recv(&self) -> Result<OffhandRes<O>, chan::TryRecvError> {
		self.rx.try_recv().map(|r| OffhandRes(Some(r)))
	}
}
impl<O> Drop for Offhand<O> {
	fn drop(&mut self) {
		let _ = self.handle.take().unwrap().join();
	}
}

pub struct OffhandRes<O>(Option<(O, Fence)>);
impl<O> OffhandRes<O> {
	pub fn wait(self) -> Option<O> {
		self.0.map(|(r, f)| {
			f.Block();
			r
		})
	}
}
