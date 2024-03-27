use super::{compiler::*, parsing::*, uniform::*, *};
use crate::FS;
use GL::{offhand::*, window::*};

pub struct Shader {
	name: ShdName,
	prog: ShdProg,
	uniforms: HashMap<u32, i32>,
	binds_cache: HashMap<i32, i32>,
	dynamic: bool,
}
impl Shader {
	pub fn pure<const N: usize>(args: [I; N]) -> Self {
		Self::new(args).unwrap()
	}
	pub fn new(args: impl ShaderArgs) -> Res<Self> {
		let ShaderManager { sn, rx, .. } = ShaderManager::get();
		let name = args.get();

		sn.send(Create(name)).unwrap();
		let ShdResult { prog, name } = rx.recv().unwrap().wait();
		let (uniforms, binds_cache, dynamic) = Def();

		Ok(Self { name, prog: prog?, uniforms, binds_cache, dynamic })
	}
	pub fn watch(args: impl ShaderArgs) -> Res<Self> {
		let ShaderManager { sn, .. } = ShaderManager::get();
		let mut s = Self::new(args)?;

		sn.send(Watch(s.name.clone())).unwrap();
		s.dynamic = true;

		Ok(s)
	}
	pub fn Bind(&mut self) -> ShaderBinding {
		let ShaderManager { sn, rx, mailbox, .. } = ShaderManager::get();

		if self.dynamic {
			sn.send(Rebuild).unwrap();
			while let Some(msg) = rx.try_recv() {
				let ShdResult { name, prog } = msg.wait();
				mailbox.insert(name, Some(prog));
			}
			let Self { name, prog, uniforms, binds_cache, .. } = self;
			if let Some(p @ Some(_)) = mailbox.get_mut(name) {
				let p = p.take().unwrap();
				match p {
					Err(e) => WARN!(e),
					Ok(p) => {
						*prog = p;
						(*uniforms, *binds_cache) = Def();
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
			sn.send(Forget(mem::take(name))).unwrap();
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
		let Shader { name: shd_name, prog, uniforms, binds_cache, .. } = self.shd;
		let addr = match args.kind() {
			ArgsKind::Uniform => get_addr(uniforms, (id, name), |n| {
				let addr = GLCheck!(gl::GetUniformLocation(prog.obj, n.as_ptr()));
				if addr == -1 {
					INFO!("No uniform {name:?} in shader {:?}, or it was optimized out", shd_name.join(" "));
				}
				addr
			}),
			ArgsKind::UBO => get_addr(uniforms, (id, name), |n| {
				let addr = GLCheck!(gl::GetUniformBlockIndex(prog.obj, n.as_ptr()));
				if addr == gl::INVALID_INDEX {
					INFO!("No UBO {name:?} in shader {:?}, or it was optimized out", shd_name.join(" "));
					return -1;
				}
				i32(addr)
			}),
			ArgsKind::SSBO => {
				DEBUG!("GL SSBO {name:?} bound, shader {:?}", shd_name.join(" "));
				-1
			}
		};
		if addr != -1 {
			args.pass(addr, binds_cache);
		}
	}
}
impl Drop for ShaderBinding<'_> {
	fn drop(&mut self) {
		ShaderProg::Unlock();
	}
}
fn get_addr(u: &mut HashMap<u32, i32>, (id, name): (u32, &str), addr: impl Fn(CString) -> i32) -> i32 {
	ASSERT!(uniforms_use::id(name).0 == id, "Use Uniforms!()/Unibuffers!() macro to set uniforms");
	if let Some(addr) = u.get(&id) {
		let _collision_map = LocalStatic!(HashMap<u32, String>);
		ASSERT!(_collision_map.entry(id).or_insert(name.into()) == name, "Unifrom collision at entry {name}");
		*addr
	} else {
		let addr = addr(CString::new(name).unwrap());
		u.insert(id, addr);
		addr
	}
}

pub struct ShaderManager {
	sn: Sender<ShaderTask>,
	rx: Offhand<ShdResult>,
	mailbox: HashMap<ShdName, Option<Res<ShdProg>>>,
}
impl ShaderManager {
	pub fn Initialize(w: &mut Window) -> &mut Self {
		Self::get_or_init(Some(w))
	}
	pub fn LoadSources(filename: impl Into<Astr>) {
		let n = filename.into();
		let file = load(n.clone(), FS::Lazy::Text(n));
		ShaderManager::get().sn.send(Load(file)).unwrap();
	}
	pub fn WatchSources(filename: impl Into<Astr>) {
		let n = filename.into();
		let file = load(n.clone(), FS::Watch::Text(n));
		ShaderManager::get().sn.send(Load(file)).unwrap();
	}
	pub fn CleanCache() {
		ShaderManager::get().sn.send(Clean).unwrap();
	}
	fn inline_source(name: &str, source: &str) {
		ShaderManager::get().sn.send(Inline((name.into(), CString::new(source).unwrap()))).unwrap();
	}
	fn get() -> &'static mut Self {
		Self::get_or_init(None)
	}
	fn get_or_init(w: Option<&mut Window>) -> &'static mut Self {
		LocalStatic!(ShaderManager, {
			if let Some(w) = w {
				let (sn, rx) = Offhand::from_fn(w, 64, compiler);
				Self { sn, rx, mailbox: Def() }
			} else {
				ERROR!("Must Initialize ShaderManager before first use");
			}
		})
	}
}

impl From<InlineShader> for Str {
	fn from(v: InlineShader) -> Self {
		let InlineShader(v, v_t) = v;
		ShaderManager::inline_source(v, v_t);
		v.into()
	}
}
