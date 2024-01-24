use crate::{lib::*, GL};

pub trait State {
	fn bound_obj() -> &'static mut u32;
	fn tracked_obj() -> &'static mut u32;
	unsafe fn bind(_: u32) {}
	unsafe fn gen(obj: &mut u32) {
		GL::unigl::glCreateBuffer(obj);
	}
	unsafe fn del(obj: &mut u32) {
		gl::DeleteBuffers(1, obj);
		debug_assert!({
			Self::crossbindcheck_map().values_mut().for_each(|b| {
				b.iter_mut().for_each(|o| {
					if *o == *obj {
						*o = 0;
					}
				})
			});
			true
		});
	}
	fn crossbindcheck_map() -> &'static mut HashMap<u32, Vec<u32>> {
		LocalStatic!(HashMap<u32, Vec<u32>>)
	}
	fn checkcrossbinds(obj: &u32) {
		debug_assert!({
			if let Ok(_pos) = Self::crossbindcheck_map()
				.get(obj)
				.unwrap_or_else(|| ASSERT!(false, "No {} buffer bound to GL object {obj}", type_name!(Self)))
				.binary_search(&0)
			{
				ASSERT!(false, "{} buffer bound to GL object {obj} at position {_pos} was invalidated", type_name!(Self));
			}
			true
		});
	}

	fn Lock(obj: u32) {
		debug_assert!({
			let _tracked = *Self::tracked_obj();
			ASSERT!(_tracked == 0, "Tried to bind GL {} object {obj} while {_tracked} ", type_name!(Self));
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
		DEBUG!("Created GL {} obj {obj}", type_name!(Self));
		obj
	}
	fn Bind(obj: u32) {
		let bound_obj = Self::bound_obj();
		if *bound_obj != obj {
			DEBUG!("Binding GL {} obj {obj}", type_name!(Self));
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
		DEBUG!("Deleting GL {} obj {obj}", type_name!(Self));
		GLCheck!(Self::del(&mut obj));
	}
}
