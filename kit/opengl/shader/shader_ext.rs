use super::{args::*, compiler::*, parsing::*, uniform::*, *};
use GL::offhand::*;

pub struct Shader {
	name: ShdName,
	prog: ShdProg,
	uniforms: Uniforms,
	dynamic: bool,
}
impl Shader {
	pub fn pure<const N: usize>(args: [I; N]) -> Self {
		Self::new(args).fail()
	}
	pub fn new(args: impl CompileArgs) -> Res<Self> {
		let ShaderManager { sn, rx, .. } = ShaderManager::get();
		let name = args.get();

		sn.send(Create(name)).valid();
		let ShdResult { prog, name } = rx.recv().valid().wait();
		let (uniforms, dynamic) = Def();

		Ok(Self { name, prog: prog?, uniforms, dynamic })
	}
	pub fn watch(args: impl CompileArgs) -> Res<Self> {
		let ShaderManager { sn, .. } = ShaderManager::get();
		let mut s = Self::new(args)?;

		sn.send(Watch(s.name.clone())).valid();
		s.dynamic = true;

		Ok(s)
	}
	pub fn Bind(&mut self) -> ShaderBinding {
		let ShaderManager { sn, rx, mailbox, .. } = ShaderManager::get();

		if self.dynamic {
			sn.send(Rebuild).valid();
			while let Some(msg) = rx.try_recv() {
				let ShdResult { name, prog } = msg.wait();
				mailbox.insert(name, Some(prog));
			}
			let Self { name, prog, uniforms, .. } = self;
			if let Some(p @ Some(_)) = mailbox.get_mut(name) {
				let p = p.take().valid();
				match p {
					Err(e) => WARN!(e),
					Ok(p) => {
						*prog = p;
						*uniforms = Def();
						PRINT!(format!("Rebuilt shader {}", name.join(" ")).green().bold())
					}
				}
			}
		}

		ShaderBinding::new(self)
	}
}
impl Drop for Shader {
	fn drop(&mut self) {
		let Self { name, dynamic, .. } = self;
		if *dynamic {
			let ShaderManager { sn, .. } = ShaderManager::get();
			sn.send(Forget(mem::take(name))).valid();
		}
	}
}

pub struct ShaderBinding<'l> {
	shd: &'l mut Shader,
}
impl<'l> ShaderBinding<'l> {
	pub fn new(o: &'l mut Shader) -> Self {
		ShaderProg::Lock(o.prog.obj);
		ShaderProg::Bind(o.prog.obj);
		Self { shd: o }
	}
	pub fn is_fresh(&self) -> bool {
		self.shd.uniforms.is_empty()
	}
	pub fn Uniform(&mut self, (id, name): (u32, &str), args: impl UniformArgs) {
		let Shader { name: shd_name, prog, uniforms, .. } = self.shd;
		let (addr, cached) = match args.kind() {
			ArgsKind::Uniform => get_addr(uniforms, (id, name), |n| {
				let addr = GL!(gl::GetUniformLocation(prog.obj, n.as_ptr()));
				if addr == -1 {
					INFO!("No uniform {name:?} in shader {:?}, or it was optimized out", shd_name.join(" "));
				}
				addr
			}),
			ArgsKind::Ubo => get_addr(uniforms, (id, name), |n| {
				let addr = GL!(gl::GetUniformBlockIndex(prog.obj, n.as_ptr()));
				if addr == gl::INVALID_INDEX {
					INFO!("No UBO {name:?} in shader {:?}, or it was optimized out", shd_name.join(" "));
					return -1;
				}
				-2 - i32(addr)
			}),
			ArgsKind::Ssbo => return DEBUG!("GL SSBO {name:?} bound, shader {:?}", shd_name.join(" ")),
		};
		if addr != -1 {
			args.apply(addr, cached);
		}
	}
}
impl Drop for ShaderBinding<'_> {
	fn drop(&mut self) {
		ShaderProg::Unlock();
	}
}
fn get_addr<'l>(u: &'l mut Uniforms, (id, name): (u32, &str), addr: impl Fn(CString) -> i32) -> (i32, &'l mut Option<CachedUni>) {
	ASSERT!(uniforms_use::id(name).0 == id, "Use Uniforms!()/Unibuffers!() macro to set uniforms");
	ASSERT!(
		LocalStatic!(HashMap<u32, String>).entry(id).or_insert(name.into()) == name,
		"Unifrom collision at entry {name}"
	);

	let (addr, val) = u.entry(id).or_insert_with(|| (addr(CString::new(name).valid()), None));
	(*addr, val)
}

pub struct ShaderManager {
	sn: Sender<ShaderTask>,
	rx: Offhand<ShdResult>,
	mailbox: HashMap<ShdName, Option<Res<ShdProg>>>,
}
impl ShaderManager {
	pub fn Initialize<'s>(args: impl InitArgs<'s>) {
		let (window, i) = args.get();
		Self::get_or_init(Some(window)).sn.send(Includes(i)).valid();
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
	pub(super) fn inline_source(name: &str, source: &[&str]) {
		ShaderManager::get().sn.send(Inline((name.into(), source.concat()))).valid();
	}
	fn get() -> &'static mut Self {
		Self::get_or_init(None)
	}
	fn get_or_init(w: Option<&mut Window>) -> &'static mut Self {
		LeakyStatic!(ShaderManager, {
			let Some(w) = w else {
				ERROR!("Must Initialize ShaderManager before first use");
			};

			let (sn, rx) = Offhand::from_fn(w, 64, compiler);
			Self { sn, rx, mailbox: Def() }
		})
	}
}

type Uniforms = HashMap<u32, (i32, Option<CachedUni>)>;
