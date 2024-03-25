use crate::{event::*, lib::*, math::*, sync::*, GL, GL::FrameInfo};

pub trait WindowSpec {
	unsafe fn clipboard(&self) -> String;
	fn set_clipboard(&mut self, str: &str);
	fn resize(&mut self, size: uVec2);

	fn spawn_offhand_gl(&mut self, _: impl FnOnce() + SendStat) -> JoinHandle<()>;
	fn poll_events(&mut self) -> Vec<Event>;
	fn swap(&mut self);
}

pub type Window = GlfwWindow;

pub struct GlfwWindow {
	window: glfw::PWindow,
	events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
	resized_hint: bool,
	info: FrameInfo,
}
impl GlfwWindow {
	pub fn get(args: impl WINSize, title: &str) -> Res<Self> {
		use glfw::{WindowHint::*, *};

		let init_ctx: fn() -> Res<_> = || {
			let mut ctx = Res(glfw::init(|e, d| ERROR!("{e}: {d}"))).map_err(|e| format!("GLFW initialization failed, {e}"))?; //TODO don't fail for empty clipbox

			ctx.window_hint(ClientApi(ClientApiHint::OpenGl));
			ctx.window_hint(ContextVersion(GL::unigl::GL_VERSION.0, GL::unigl::GL_VERSION.1));
			ctx.window_hint(OpenGlForwardCompat(true));
			ctx.window_hint(OpenGlDebugContext(GL::unigl::IS_DEBUG));
			ctx.window_hint(OpenGlProfile(OpenGlProfileHint::Core));
			ctx.window_hint(Samples(Some(4)));
			//ctx.window_hint(DoubleBuffer(false));
			//ctx.window_hint(Resizable(false));
			Ok(ctx)
		};

		let (x, y, w, h) = args.get();
		let (mut window, events) = Res(init_ctx()?.create_window(w, h, title, WindowMode::Windowed)).map_err(|_| "Failed to create GLFW window.")?;

		window.set_pos(x, y);
		window.make_current();
		window.set_size_polling(true);
		window.set_cursor_pos_polling(true);
		window.set_mouse_button_polling(true);
		window.set_scroll_polling(true);
		window.set_key_polling(true);
		window.set_char_polling(true);
		window.glfw.set_swap_interval(SwapInterval::Sync(1));

		gl::load_with(|s| window.get_proc_address(s) as *const _);

		let version = Res(unsafe { std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8) }.to_str())?;
		PRINT!("Initialized OpenGL, {version}");
		GL::macro_uses::gl_was_initialized(true);
		if GL::unigl::IS_DEBUG {
			GL::EnableDebugContext(GL::DebugLevel::All);
		}

		let info = FrameInfo::new((w, h));
		Ok(Self { window, events, resized_hint: true, info })
	}
	pub fn info(&self) -> &FrameInfo {
		&self.info
	}
}
impl WindowSpec for GlfwWindow {
	unsafe fn clipboard(&self) -> String {
		self.window.get_clipboard_string().unwrap_or_default()
	}
	fn set_clipboard(&mut self, s: &str) {
		self.window.set_clipboard_string(s)
	}
	fn resize(&mut self, size: uVec2) {
		let Self { window, resized_hint, info, .. } = self;
		*info = FrameInfo::new(size);
		let (w, h) = iVec2(size);
		window.set_size(w, h);
		*resized_hint = true;
	}
	fn spawn_offhand_gl(&mut self, f: impl FnOnce() + SendStat) -> JoinHandle<()> {
		use glfw::{WindowHint::*, *};
		let ctx_lock = Arc::new(Barrier::new(2));
		let ctx = &mut *self.window as *mut Window as usize;
		make_context_current(None);
		let ret = {
			thread::Builder::new()
				.name("gl_offhand".into())
				.spawn({
					let l = ctx_lock.clone();
					move || {
						let mut ctx = {
							let ctx = unsafe { &mut *(ctx as *mut Window) };
							ctx.make_current();
							ctx.glfw.window_hint(Visible(false));
							if let Some((w, _)) = ctx.create_shared(1, 1, "offhand_dummy", WindowMode::Windowed) {
								w
							} else {
								l.wait();
								FAIL!("Failed to create offhand context.");
								return;
							}
						};
						ctx.make_current();
						l.wait();
						GL::macro_uses::gl_was_initialized(true);
						f();
					}
				})
				.unwrap()
		};
		ctx_lock.wait();
		self.window.make_current();
		ret
	}
	fn poll_events(&mut self) -> Vec<Event> {
		let action = |a| match a {
			glfw::Action::Press => Mod::PRESS,
			glfw::Action::Repeat => Mod::REPEAT,
			glfw::Action::Release => Mod::RELEASE,
		};
		let mods = |m: glfw::Modifiers| {
			let check = |f, a| if m.contains(f) { a } else { Mod::empty() };
			check(glfw::Modifiers::Shift, Mod::SHIFT) | check(glfw::Modifiers::Control, Mod::CTRL) | check(glfw::Modifiers::Alt, Mod::ALT) | check(glfw::Modifiers::Super, Mod::WIN)
		};

		let Self { window, events, resized_hint, info, .. } = self;
		window.glfw.poll_events();
		let collect_mods = || {
			use {glfw::*, Key::*};
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
					let ((x, y), (w, h)) = ((2., 2.).mul((x, y)), Vec2(info.size));
					let at = (x - w, h - y).div(w.min(h));
					let state = collect_mods();
					Some(Event::MouseMove { at, state })
				}
				glfw::WindowEvent::MouseButton(b, a, m) => {
					let button = match b {
						glfw::MouseButtonLeft => Button::Left,
						glfw::MouseButtonRight => Button::Right,
						glfw::MouseButtonMiddle => Button::Middle,
						_ => {
							INFO!("Excessive buttons on mouse");
							Button::Middle
						}
					};

					let state = action(a) | mods(m);
					Some(Event::MouseButton { button, state })
				}
				glfw::WindowEvent::Key(key, _, a, m) => Some(Event::Keyboard { key, state: action(a) | mods(m) }),
				glfw::WindowEvent::Scroll(x, y) => Some(Event::Scroll { at: Vec2((x, y)), state: collect_mods() }),
				glfw::WindowEvent::Char(ch) => Some(Event::Char { ch }),

				glfw::WindowEvent::Size(w, h) => {
					*info = FrameInfo::new(uVec2((w, h)));
					None
				}
				e => {
					INFO!("Registered event not covered {e:?}");
					None
				}
			})
			.collect_vec();
		if *resized_hint {
			*resized_hint = false;
			let ((x, y), (w, h)) = ((2., 2.).mul(window.get_cursor_pos()), Vec2(info.size));
			let at = (x - w, h - y).div(w.min(h));
			let state = collect_mods();
			events.push(Event::MouseMove { at, state })
		}
		events
	}
	fn swap(&mut self) {
		use glfw::*;
		self.window.swap_buffers();
	}
}

type WArgs = (i32, i32, u32, u32);
pub trait WINSize {
	fn get(self) -> WArgs;
}
impl<A, B, C, D> WINSize for (A, B, C, D)
where
	i32: Cast<A> + Cast<B>,
	u32: Cast<C> + Cast<D>,
{
	fn get(self) -> WArgs {
		<_>::to(self)
	}
}
impl<A, B> WINSize for (A, B)
where
	u32: Cast<A> + Cast<B>,
{
	fn get(self) -> WArgs {
		(0, 0, u32(self.0), u32(self.1))
	}
}
