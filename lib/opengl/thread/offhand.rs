use crate::uses::{sync::*, *};
use GL::{window::*, Fence};

pub struct Offhand<O> {
	handle: Option<JoinHandle<Res<()>>>,
	rx: Receiver<(O, Fence)>,
}
impl<O: 'static + Send> Offhand<O> {
	pub fn new<I: 'static + Send>(window: &mut Window, depth: usize, process: impl 'static + Send + Fn(I) -> O) -> (Sender<I>, Self) {
		let (data_sn, data_rx): (Sender<I>, Receiver<I>) = chan::bounded(depth);
		let (res_sn, res_rx): (Sender<(O, Fence)>, Receiver<(O, Fence)>) = chan::bounded(depth);
		let handle = window.spawn_offhand_gl(move || {
			while let Ok(msg) = data_rx.recv() {
				let res = process(msg);
				let _ = res_sn.send((res, Fence::new())).map_err(|e| FAIL!(e));
			}
		});
		let (handle, rx) = (Some(handle), res_rx);
		(data_sn, Self { handle, rx })
	}
	pub fn recv(&self) -> OffhandRes<O> {
		self.rx.recv().ok().map_or(OffhandRes(None), |r| OffhandRes(Some(r)))
	}
	pub fn try_recv(&self) -> Option<OffhandRes<O>> {
		self.rx.try_recv().ok().map(|r| OffhandRes(Some(r)))
	}
}
impl<O> Drop for Offhand<O> {
	fn drop(&mut self) {
		self.handle.take().unwrap().join().unwrap().unwrap();
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
