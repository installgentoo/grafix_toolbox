use crate::uses::*;

pub trait State {
	fn bound_obj() -> &'static mut u32;
	fn tracked_obj() -> &'static mut u32;
	unsafe fn bind(_: u32) {}
	unsafe fn gen(obj: &mut u32) {
		GL::glCreateBuffer(obj);
	}
	unsafe fn del(obj: &mut u32) {
		gl::DeleteBuffers(1, obj);
	}
	fn Lock(obj: u32) {
		debug_assert!({
			let tracked = *Self::tracked_obj();
			ASSERT!(tracked == 0, "Tried to bind GL {} object {} while {} ", type_name!(Self), obj, tracked);
			*Self::tracked_obj() = obj;
			true
		});
	}
	fn Unlock() {
		debug_assert!({
			*Self::tracked_obj() = 0;
			true
		});
	}
	fn New() -> u32 {
		let mut obj = 0;
		GLCheck!(Self::gen(&mut obj));
		ASSERT!(obj != 0, "GL {} not initilized", type_name!(Self));
		DEBUG!("Created GL {} obj {}", type_name!(Self), obj);
		obj
	}
	fn Bind(obj: u32) {
		let bound_obj = Self::bound_obj();
		if *bound_obj != obj {
			DEBUG!("Binding GL {} obj {}", type_name!(Self), obj);
			*bound_obj = obj;
			GLCheck!(Self::bind(obj));
		}
	}
	fn Drop(obj: u32) {
		ASSERT!(obj != 0, "GL {} zero before drop", type_name!(Self));
		if *Self::bound_obj() == obj {
			*Self::bound_obj() = 0;
		}
		let mut obj = obj;
		DEBUG!("Deleting GL {} obj {}", type_name!(Self), obj);
		GLCheck!(Self::del(&mut obj));
	}
}
