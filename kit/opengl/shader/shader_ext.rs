use super::{uniform::*, *};

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

		Self { name, prog: prog?, uniforms, dynamic }.pipe(Ok)
	}
	pub fn watch(args: impl CompileArgs) -> Res<Self> {
		let ShaderManager { sn, .. } = ShaderManager::get();
		Self::new(args)?
			.tap(|Self { name, dynamic, .. }| {
				*dynamic = true;
				sn.send(Watch(name.clone())).fail();
			})
			.pipe(Ok)
	}
	pub fn Bind(&mut self) -> ShaderBind {
		let ShaderManager { sn, rx, mailbox } = ShaderManager::get();

		if self.dynamic {
			sn.send(Rebuild).valid();
			while let Some(msg) = rx.try_recv() {
				let ShdResult { name, prog } = msg.wait();
				mailbox.insert(name, prog);
			}

			let Self { name, prog, uniforms, .. } = self;
			if let Some(p) = mailbox.remove(name)
				&& let Ok(p) = p.map_err(|e| WARN!(e))
			{
				(*prog, *uniforms) = (p, Def());
				PRINT!(format!("Rebuilt shader {}", name.join(" ")).green().bold())
			}
		}

		ShaderBind::new(self)
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

pub struct ShaderBind<'l> {
	shd: &'l mut Shader,
}
impl<'l> ShaderBind<'l> {
	fn new(o: &'l mut Shader) -> Self {
		ShaderT::Lock(o.prog.obj);
		ShaderT::Bind(o.prog.obj);
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
					INFO!("No uniform {name:?} in {:?}", shd_name.join(" "));
				}
				addr
			}),
			ArgsKind::Ubo => get_addr(uniforms, (id, name), |n| {
				let addr = GL!(gl::GetUniformBlockIndex(prog.obj, n.as_ptr()));
				if addr == gl::INVALID_INDEX {
					INFO!("No UBO {name:?} in {:?}", shd_name.join(" "));
					return -1;
				}
				-2 - i32(addr)
			}),
			ArgsKind::Ssbo => return DEBUG!("GL SSBO {name:?} bound in {:?}", shd_name.join(" ")),
		};
		if addr != -1 {
			args.apply(addr, cached);
		}
	}
}
impl Drop for ShaderBind<'_> {
	fn drop(&mut self) {
		ShaderT::Unlock();
	}
}
fn get_addr<'l>(u: &'l mut Uniforms, (id, name): (u32, &str), addr: impl FnOnce(CString) -> i32) -> (i32, &'l mut Option<CachedUni>) {
	ASSERT!(uniforms_use::id(name).0 == id, "Use Uniforms!()/Unibuffers!() macro to set uniforms");
	ASSERT!(
		LocalStatic!(HashMap<u32, String>).entry(id).or_insert(name.into()) == name,
		"Unifrom name collision {name:?}"
	);

	let (addr, val) = u.entry(id).or_insert_with(|| (name.pipe(CString::new).valid().pipe(addr), None));
	(*addr, val)
}

type Uniforms = HashMap<u32, (i32, Option<CachedUni>)>;
