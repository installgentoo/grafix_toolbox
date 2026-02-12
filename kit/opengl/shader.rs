pub use shader_ext::*;

#[macro_use]
pub mod uniform;

#[macro_export]
macro_rules! SHADER {
	($n: ident, $($body: expr),+) => {
		#[allow(non_upper_case_globals)]
		pub const $n: $crate::GL::macro_uses::InlineShader = $crate::GL::macro_uses::InlineShader(stringify!($n), &[$($body,)+]);
	};
}

pub struct ShaderManager {
	sn: Sender<ShaderTask>,
	rx: Offhand<ShdResult>,
	mailbox: HashMap<ShdName, Res<ShdProg>>,
}
impl ShaderManager {
	pub fn Initialize<'s, W: Window>(args: impl InitArgs<'s, W>) {
		let (window, i) = args.get();
		Some(window).pipe(Self::get_or_init).sn.send(Includes(i)).valid();
	}
	pub fn Load(filenames: impl LoadArgs) {
		for n in filenames.get() {
			let file = load(n.clone(), FS::Lazy::Text(n));
			ShaderManager::get().sn.send(Load(file)).valid();
		}
	}
	pub fn Watch(filenames: impl LoadArgs) {
		for n in filenames.get() {
			let file = load(n.clone(), FS::Watch::Text(n));
			ShaderManager::get().sn.send(Load(file)).valid();
		}
	}
	pub fn CleanCache() {
		ShaderManager::get().sn.send(Clean).valid();
	}
	pub fn inline_source(name: &str, source: &[&str]) {
		ShaderManager::get().sn.send(Inline((name.into(), source.concat()))).valid();
	}
	fn get() -> &'static mut Self {
		Self::get_or_init::<WindowImpl>(None)
	}
	fn get_or_init<W: Window>(w: Option<&mut W>) -> &'static mut Self {
		LeakyStatic!(ShaderManager, {
			let w = w.unwrap_or_else(|| ERROR!("Must Initialize ShaderManager before first use"));
			let (sn, rx) = Offhand::from_fn(w, 64, compiler);
			Self { sn, rx, mailbox: Def() }
		})
	}
}

mod args;
mod compiler;
mod object;
mod parsing;
mod shader_ext;

pub struct InlineShader(pub STR, pub &'static [STR]);
impl From<I> for Str {
	fn from(v: I) -> Self {
		let InlineShader(v, v_t) = v;
		ShaderManager::inline_source(v, v_t);
		v.into()
	}
}
type I = InlineShader;

use crate::{GL::offhand::*, GL::window::*, lazy::*, lib::*, slicing::*, sync::*};
use {super::internal::*, args::*, compiler::*, parsing::*, std::ffi::CString};
