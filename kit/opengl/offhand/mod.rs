pub use fence::*;

pub struct Offhand<O> {
	handle: Option<JoinHandle<()>>,
	rx: Receiver<(O, Fence)>,
}
impl<O: SendStat> Offhand<O> {
	pub fn new<I: SendStat>(window: &mut Window, depth: usize, process: impl SendStat + Fn(I) -> O) -> (Sender<I>, Self) {
		let (data_sn, data_rx) = chan::bounded::<I>(depth);
		let (res_sn, res_rx) = chan::bounded::<(O, Fence)>(depth);
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
		self.handle.take().unwrap().join().unwrap();
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

mod fence;

use crate::{lib::*, sync::*, GL::window::*};
