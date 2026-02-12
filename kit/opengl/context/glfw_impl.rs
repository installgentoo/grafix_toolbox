use super::{event::*, window::*, *};
pub use glfw::Key;

pub type CtxDrop = ();

pub struct WindowImpl {
	window: glfw::PWindow,
	events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
	resized_hint: bool,
	info: FrameInfo,
}
impl WindowImpl {
	pub fn get(args: impl WINSize, title: &str) -> Res<Self> {
		use glfw::{WindowHint::*, *};

		let init_ctx: fn() -> Res<_> = || {
			let mut ctx = glfw::init(|e, d| ERROR!("{e}: {d}")).explain_err(|| "GLFW init failed")?;

			ctx.window_hint(ClientApi(ClientApiHint::OpenGl));
			let (maj, min, _) = GL::unigl::GL_VERSION;
			ctx.window_hint(ContextVersion(maj, min));
			ctx.window_hint(OpenGlForwardCompat(true));
			ctx.window_hint(OpenGlDebugContext(GL::unigl::IS_DEBUG));
			ctx.window_hint(OpenGlProfile(OpenGlProfileHint::Core));
			ctx.window_hint(Samples(4.pipe(Some)));
			//ctx.window_hint(DoubleBuffer(false));
			//ctx.window_hint(Resizable(false));
			Ok(ctx)
		};

		let (x, y, w, h) = args.get();
		let (mut window, events) = init_ctx()?
			.create_window(w, h, title, WindowMode::Windowed)
			.explain_err(|| "Cannot create GLFW window")?;

		window.set_pos(x, y);
		window.make_current();
		window.set_size_polling(true);
		window.set_cursor_pos_polling(true);
		window.set_mouse_button_polling(true);
		window.set_scroll_polling(true);
		window.set_key_polling(true);
		window.set_char_polling(true);
		window.glfw.set_swap_interval(SwapInterval::Sync(1));

		load_gl(|s| window.get_proc_address(s) as *const _);

		window.glfw.set_error_callback(|e, d| match e {
			glfw::Error::FormatUnavailable => (), // Emitted on bad clipboard
			_ => ERROR!("{e}: {d}"),
		});

		let info = FrameInfo::new((w, h));
		Self { window, events, resized_hint: true, info }
			.tap(|w| {
				crate::kit::policies::task::InitGLRuntime(Some(w));
			})
			.pipe(Ok)
	}
}

impl Window for WindowImpl {
	fn info(&self) -> &FrameInfo {
		&self.info
	}
	fn clipboard(&self) -> String {
		self.window.get_clipboard_string().unwrap_or_default()
	}
	fn set_clipboard(&mut self, s: &str) {
		self.window.set_clipboard_string(s)
	}
	fn set_vsync(&mut self, e: bool) {
		use glfw::SwapInterval::*;
		self.window.glfw.set_swap_interval(if e { Sync(1) } else { None });
	}
	fn resize(&mut self, size: uVec2) {
		let Self { window, resized_hint, info, .. } = self;
		*info = FrameInfo::new(size);
		let (w, h) = vec2(size);
		window.set_size(w, h);
		*resized_hint = true;
	}
	fn gl_ctx_maker(&mut self) -> impl FnOnce() + SendS {
		use glfw::{WindowHint::*, *};

		let ctx = &mut self.window;

		ctx.glfw.window_hint(Visible(false));

		let Some((mut ctx, _)) = ctx.create_shared(1, 1, "offhand_dummy", WindowMode::Windowed) else {
			ERROR!("Cannot create offhand context");
		};

		self.window.make_current();

		move || {
			ctx.make_current();
			*GL::macro_uses::gl_was_initialized() = true;
		}
	}
	fn poll_events(&mut self) -> Vec<Event> {
		let action = |a| match a {
			glfw::Action::Press | glfw::Action::Repeat => Mod::PRESS,
			glfw::Action::Release => Mod::RELEASE,
		};
		let mods = |m: glfw::Modifiers| {
			let check = |f, a| if m.contains(f) { a } else { Mod::empty() };
			check(glfw::Modifiers::Shift, Mod::SHIFT) | check(glfw::Modifiers::Control, Mod::CTRL) | check(glfw::Modifiers::Alt, Mod::ALT) | check(glfw::Modifiers::Super, Mod::WIN)
		};

		let Self { window, events, resized_hint, info } = self;
		window.glfw.poll_events();
		let collect_mods = || {
			use {Key::*, glfw::*};
			let mouse = |k| window.get_mouse_button(k) == Action::Press;
			let active = |k| window.get_key(k) == Action::Press;
			let shift = active(LeftShift) || active(RightShift);
			let ctrl = active(LeftControl) || active(RightControl);
			let alt = active(LeftAlt) || active(RightAlt);
			let win = active(LeftSuper) || active(RightSuper);
			let left = mouse(MouseButtonLeft);
			let mid = mouse(MouseButtonMiddle);
			let right = mouse(MouseButtonRight);
			let add = |s, a| if s { a } else { Mod::empty() };
			add(shift, Mod::SHIFT) | add(ctrl, Mod::CTRL) | add(alt, Mod::ALT) | add(win, Mod::WIN) | add(left, Mod::LEFT) | add(mid, Mod::MID) | add(right, Mod::RIGHT)
		};
		let mut events = glfw::flush_messages(events)
			.filter_map(|(_, event)| match event {
				glfw::WindowEvent::CursorPos(x, y) => {
					let ((x, y), s @ (w, h)) = (Vec2(2).mul((x, y)), Vec2(info.size));
					let at = (x - w, h - y).div(s.min_comp());
					let m = collect_mods();
					Event::MouseMove { at, m }.pipe(Some)
				}
				glfw::WindowEvent::MouseButton(b, a, m) => {
					let button = match b {
						glfw::MouseButtonLeft => Click::Left,
						glfw::MouseButtonRight => Click::Right,
						glfw::MouseButtonMiddle => Click::Middle,
						_ => {
							INFO!("Excessive buttons on mouse");
							Click::Middle
						}
					};

					let m = action(a) | mods(m);
					Event::MouseButton { button, m }.pipe(Some)
				}
				glfw::WindowEvent::Key(key, _, a, m) => Event::Keyboard { key, m: action(a) | mods(m) }.pipe(Some),
				glfw::WindowEvent::Scroll(x, y) => Event::Scroll { at: Vec2((x, y)), m: collect_mods() }.pipe(Some),
				glfw::WindowEvent::Char(ch) => Event::Char { ch }.pipe(Some),

				glfw::WindowEvent::Size(w, h) => {
					*info = FrameInfo::new(vec2((w, h)));
					None
				}
				e => {
					DEBUG!("Registered event not covered {e:?}");
					None
				}
			})
			.collect_vec();
		if *resized_hint {
			*resized_hint = false;
			let ((x, y), s @ (w, h)) = (Vec2(2).mul(window.get_cursor_pos()), Vec2(info.size));
			let at = (x - w, h - y).div(s.min_comp());
			let m = collect_mods();
			events.push(Event::MouseMove { at, m })
		}
		events
	}
	fn swap(&mut self) {
		use glfw::*;
		self.window.swap_buffers();
	}
}
