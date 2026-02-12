use super::{window::*, *};
use crate::sync::*;

pub struct Offhand<O> {
	handle: Option<JoinHandle<()>>,
	rx: Receiver<(O, Fence)>,
}
impl<O: SendS> Offhand<O> {
	pub fn new<I: SendS>(w: &mut impl Window, depth: usize, process: impl SendS + Fn(I) -> O) -> (Sender<I>, Self) {
		Self::from_fn(w, depth, move |data_rx, res_sn| {
			while let Ok(msg) = data_rx.recv() {
				let res = process(msg);
				res_sn.send((res, Fence::new())).warn();
			}
		})
	}
	pub fn from_fn<I: SendS>(w: &mut impl Window, depth: usize, process: impl SendS + Fn(Receiver<I>, Sender<(O, Fence)>)) -> (Sender<I>, Self) {
		let (data_sn, data_rx) = chan::bounded::<I>(depth);
		let (res_sn, rx) = chan::bounded::<(O, Fence)>(depth);

		let init = w.gl_ctx_maker();
		let handle = thread::Builder::new()
			.name("gl_offhand".into())
			.spawn(move || {
				let _ctx = init();
				process(data_rx, res_sn)
			})
			.explain_err(|| "Cannot spawn offhand")
			.fail()
			.pipe(Some);

		(data_sn, Self { handle, rx })
	}
	pub fn recv(&self) -> Option<OffhandRes<O>> {
		self.rx.recv().ok().map(OffhandRes)
	}
	pub fn try_recv(&self) -> Option<OffhandRes<O>> {
		self.rx.try_recv().ok().map(OffhandRes)
	}
}
impl<O> Drop for Offhand<O> {
	fn drop(&mut self) {
		self.handle.take().valid().join_fail(|| format!("GL Offhand<{}> panicked", type_name::<O>()));
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
