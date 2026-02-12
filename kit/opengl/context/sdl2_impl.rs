use super::{event::*, window, *};
use sdl2::video::*;

pub struct CtxDrop(Window, GLContext);
unsafe impl Send for CtxDrop {}

pub struct WindowImpl {
	mods: Mod,
	ctx: GLContext,
	window: Window,
	events: sdl2::EventPump,
	info: FrameInfo,
}
impl WindowImpl {
	pub fn get(args: impl window::WINSize, title: &str) -> Res<Self> {
		use WindowPos::Positioned as P;

		let sdl = sdl2::init().explain_err(|| "SLD2 init failed")?;
		let video = sdl.video().explain_err(|| "SDL2 video failed")?;

		let gl = video.gl_attr();
		gl.set_accelerated_visual(true);
		gl.set_context_profile(GLProfile::Core);
		let (maj, min, _) = GL::unigl::GL_VERSION;
		gl.set_context_version(maj as u8, min as u8);
		gl.set_context_flags()
			.forward_compatible()
			.pipe(|a| if GL::unigl::IS_DEBUG { a.debug() } else { a })
			.set();
		gl.set_multisample_buffers(1);
		gl.set_multisample_samples(4);

		let (x, y, w, h) = args.get();
		let window = video
			.window(title, w, h)
			.opengl()
			.resizable()
			.build()
			.explain_err(|| "Cannot create SDL2 window")?
			.tap(|w| w.set_position(P(x), P(y)));

		video.text_input().start();

		let ctx = window.gl_create_context().explain_err(|| "SDL2 context failed")?;

		gl.set_share_with_current_context(true);

		load_gl(|s| video.gl_get_proc_address(s) as *const _);

		let events = sdl.event_pump().unwrap();

		let info = FrameInfo::new((w, h));
		Self { mods: Mod::empty(), ctx, window, events, info }
			.tap(|w| {
				crate::kit::policies::task::InitGLRuntime(Some(w));
				sdl.pipe(Box).pipe(Box::leak);
			})
			.pipe(Ok)
	}
}
impl window::Window for WindowImpl {
	fn info(&self) -> &FrameInfo {
		&self.info
	}
	fn clipboard(&self) -> String {
		self.window.subsystem().clipboard().clipboard_text().unwrap_or_default()
	}
	fn set_clipboard(&mut self, s: &str) {
		self.window.subsystem().clipboard().set_clipboard_text(&s[..s.len().min(60_000)]).warn()
	}
	fn set_vsync(&mut self, e: bool) {
		use SwapInterval::*;
		self.window.subsystem().gl_set_swap_interval(if e { VSync } else { Immediate }).warn()
	}
	fn resize(&mut self, size: uVec2) {
		let Self { window, info, .. } = self;
		*info = FrameInfo::new(size);
		let (w, h) = vec2(size);
		window.set_size(w, h).warn();
	}
	fn gl_ctx_maker(&mut self) -> impl SendS + FnOnce() -> CtxDrop {
		let Self { ctx, window, .. } = self;

		let w = window
			.subsystem()
			.window("offhand", 1, 1)
			.opengl()
			.borderless()
			.hidden()
			.build()
			.explain_err(|| "Cannot create offhand SDL2 window")
			.fail();
		let c = w.gl_create_context().explain_err(|| "SDL2 gl failed").fail();
		let ctxdrop = CtxDrop(w, c);

		window.gl_make_current(ctx).fail();

		move || {
			let CtxDrop(window, ctx) = &ctxdrop;
			window.gl_make_current(ctx).fail();
			*GL::macro_uses::gl_was_initialized() = true;
			ctxdrop
		}
	}
	fn poll_events(&mut self) -> Vec<Event> {
		use sdl2::{event::Event::*, event::WindowEvent::Resized, keyboard::Mod as M};
		use {Click as C, Event as E, Key as K};

		let Self { mods, events, info, .. } = self;
		let modkey = |k| match k {
			K::LShift | K::RShift => Mod::SHIFT,
			K::LCtrl | K::RCtrl => Mod::CTRL,
			K::LAlt | K::RAlt => Mod::ALT,
			K::LGui | K::RGui => Mod::WIN,
			_ => Mod::empty(),
		};
		let getmod = |m: M| {
			let i = |c| m.intersects(c);
			Mod::SHIFT.or_def(i(M::LSHIFTMOD | M::RSHIFTMOD))
				| Mod::CTRL.or_def(i(M::LCTRLMOD | M::RCTRLMOD))
				| Mod::ALT.or_def(i(M::LALTMOD | M::RALTMOD))
				| Mod::WIN.or_def(i(M::LGUIMOD | M::RGUIMOD))
		};

		events
			.poll_iter()
			.flat_map(|event| match event {
				MouseMotion { mousestate: m, x, y, .. } => {
					let ((x, y), s @ (w, h)) = (Vec2(2).mul((x, y)), Vec2(info.size));
					let at = (x - w, h - y).div(s.min_comp());

					let m = Mod::LEFT.or_def(m.left()) | Mod::RIGHT.or_def(m.right()) | Mod::MID.or_def(m.middle());
					let m = m | *mods;

					vec![E::MouseMove { at, m }]
				}
				MouseButtonDown { mouse_btn: b, .. } | MouseButtonUp { mouse_btn: b, .. } => {
					use sdl2::mouse::MouseButton as B;
					let button = match b {
						B::Left => C::Left,
						B::Right => C::Right,
						B::Middle => C::Middle,
						_ => {
							WARN!("Excessive buttons on mouse");
							C::Middle
						}
					};

					let m = matches!(event, MouseButtonDown { .. });
					let m = if m { Mod::PRESS } else { Mod::RELEASE };
					let m = m | *mods;

					vec![E::MouseButton { button, m }]
				}
				KeyDown { keycode: Some(key), keymod: m, .. } => {
					*mods = mods.union(modkey(key));
					vec![E::Keyboard { key, m: Mod::PRESS | getmod(m) }]
				}
				KeyUp { keycode: Some(key), keymod: m, .. } => {
					*mods = mods.difference(modkey(key));
					vec![E::Keyboard { key, m: Mod::RELEASE | getmod(m) }]
				}
				MouseWheel { precise_x: x, precise_y: y, .. } => vec![E::Scroll { at: (-x, y), m: *mods }],
				TextInput { text, .. } => text.chars().map(|ch| E::Char { ch }).collect(),
				Window { win_event: Resized(w, h), .. } => {
					*info = FrameInfo::new(vec2((w, h)));
					vec![]
				}
				e => {
					DEBUG!("Registered event not covered {e:?}");
					vec![]
				}
			})
			.collect()
	}
	fn swap(&mut self) {
		self.window.gl_swap_window();
	}
}
