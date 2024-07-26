pub use fence::*;

pub struct Offhand<O> {
	handle: Option<JoinHandle<()>>,
	rx: Receiver<(O, Fence)>,
}
impl<O: SendStat> Offhand<O> {
	pub fn new<I: SendStat>(w: &mut Window, depth: usize, process: impl SendStat + Fn(I) -> O) -> (Sender<I>, Self) {
		Self::from_fn(w, depth, move |data_rx, res_sn| {
			while let Ok(msg) = data_rx.recv() {
				let res = process(msg);
				res_sn.send((res, Fence::new())).warn();
			}
		})
	}
	pub fn from_fn<I: SendStat>(w: &mut Window, depth: usize, process: impl SendStat + Fn(Receiver<I>, Sender<(O, Fence)>)) -> (Sender<I>, Self) {
		let (data_sn, data_rx) = chan::bounded::<I>(depth);
		let (res_sn, res_rx) = chan::bounded::<(O, Fence)>(depth);
		let handle = w.spawn_offhand_gl(move || process(data_rx, res_sn));
		let (handle, rx) = (Some(handle), res_rx);
		(data_sn, Self { handle, rx })
	}
	pub fn recv(&self) -> Option<OffhandRes<O>> {
		self.rx.recv().ok().map(|r| OffhandRes(r))
	}
	pub fn try_recv(&self) -> Option<OffhandRes<O>> {
		self.rx.try_recv().ok().map(|r| OffhandRes(r))
	}
}
impl<O> Drop for Offhand<O> {
	fn drop(&mut self) {
		self.handle.take().valid().join().valid();
	}
}

pub struct OffhandRes<O>((O, Fence));
impl<O> OffhandRes<O> {
	pub fn wait(self) -> O {
		let (r, f) = self.0;
		f.Block();
		r
	}
}

mod fence;

use crate::{lib::*, sync::*, GL::window::*};
