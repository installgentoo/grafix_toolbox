use super::{object::*, parsing::*, *};
use GL::Fence;

pub fn compiler(task_rx: Receiver<ShaderTask>, res_sn: Sender<(ShdResult, Fence)>) {
	let (mut includes, mut files, mut sources, mut watched): (Str, Vec<ShdFile>, HashMap<_, _>, HashMap<_, Vec<_>>) = Def();

	let mut tasks = task_rx
		.iter()
		.flat_map(|m| iter::once(Some(m)).chain(task_rx.try_iter().map(Some)).chain(Some(None)))
		.peekable();

	while let Some(task) = tasks.next() {
		let Some(task) = task else { continue };

		if let (Rebuild, Some(Some(Rebuild))) = (&task, tasks.peek()) {
			continue;
		}

		let send = coerce(|name: ShdName, s, f: &mut [_]| {
			let prog = compile(&includes, s, f, &name).map_err(|e| adjust_log(e, 1 - i32(includes.lines().count())));
			res_sn.send((ShdResult { name, prog }, Fence::new())).fail();
		});

		match task {
			Includes(i) => includes = parse_includes(i).warn().into(),
			Watch(name) => name.iter().for_each(|n| watched.entry(n.clone()).or_default().push(name.clone())),
			Forget(name) => name.iter().for_each(|n| {
				let w = watched.get_mut(n).valid();
				let idx = w.iter().position(|n| *n == name).valid();
				w.swap_remove(idx);
			}),
			Rebuild => files
				.iter_mut()
				.filter_map(|p| {
					p.try_lock()?
						.try_take()?
						.drain(..)
						.filter_map(|ShdSrc { name, src }| {
							let Some(s) = sources.get_mut(&name) else {
								sources.insert(name, Source { src });
								None?
							};

							if matches!(s, Source { src: s } | Compiled { src: s, .. } if *s == src) {
								None?
							}

							*s = Source { src };
							Some(name)
						})
						.collect_vec()
						.into()
				})
				.flatten()
				.filter_map(|name| watched.get(&name))
				.flatten()
				.collect_vec()
				.into_iter()
				.for_each(|name| send(name.clone(), &mut sources, &mut files)),
			Inline((name, source)) => {
				ASSERT!(
					sources.get(&name).map(|(Source { src } | Compiled { src, .. })| *src == source).unwrap_or(true),
					"Shader {name:?} already exists"
				);

				sources.insert(name, Source { src: source });
			}
			Create(name) => send(name, &mut sources, &mut files),
			Load(file) => files.push(file),
			Clean => sources
				.iter_mut()
				.for_each(|(_, v)| or_map!(v = Compiled { src, .. } => Source { src: mem::take(src) })),
		}
	}
}
fn compile(includes: &str, sources: &mut HashMap<Str, ShdState>, files: &mut [ShdFile], name: &[Str]) -> Res<ShdProg> {
	let get_object = |name: &Str| -> Res<_> {
		let get = |s: &mut HashMap<_, _>| {
			let state = s.get_mut(name).ok_or_else(|| format!("No shader {name:?} in loaded sources"))?;

			match state {
				Compiled { obj, .. } => obj.obj(),
				Source { src } => {
					let c_src = [includes, src]
						.concat()
						.pipe(CString::new)
						.explain_err(|| format!("Malformed string in shader {name:?}"))
						.fail();
					let (o, new) = ShaderObj::new(name, &c_src).map(|obj| (obj.obj(), Compiled { src: mem::take(src), obj }))?;
					*state = new;
					o
				}
			}
			.pipe(Ok)
		};

		if let o @ Ok(_) = get(sources) {
			return o;
		}

		files.iter_mut().filter_map(|p| p.lock().try_take()).flatten().for_each(|ShdSrc { name, src }| {
			sources
				.insert(name.clone(), Source { src })
				.map(|_| WARN!("Replacing shader source {name:?}"))
				.sink()
		});

		get(sources)
	};

	let objects = name.iter().map(get_object).collect::<Res<Vec<_>>>()?;
	let prog = Obj::new();
	let obj = prog.obj;

	objects.iter().for_each(|&o| GL!(gl::AttachShader(obj, o)));
	GL!(gl::LinkProgram(obj));
	let status = 0_i32.tap(|s| GL!(gl::GetProgramiv(obj, gl::LINK_STATUS, s)));
	objects.iter().for_each(|&o| GL!(gl::DetachShader(obj, o)));

	if GLbool::to(status) == gl::FALSE {
		return format!("Error linking program {:?}, {obj}\n{}", name.join(" "), print_shader_log(obj)).pipe(Err);
	}

	DEBUG!("Compiled GL shader {}:{:?}", prog.obj, name.join(" "));
	Ok(prog)
}
fn coerce<T1, T2, T3, F: Fn(T1, &mut T2, &mut [T3])>(f: F) -> F {
	f
}

pub type ShdName = Box<[Str]>;
pub type ShdProg = Obj<ShaderT>;
pub type ShdFile = Feed<Option<Vec<ShdSrc>>>;

pub struct ShdResult {
	pub name: ShdName,
	pub prog: Res<ShdProg>,
}
pub enum ShaderTask {
	Includes(Vec<Astr>),
	Watch(ShdName),
	Forget(ShdName),
	Rebuild,
	Inline((Str, String)),
	Create(ShdName),
	Load(ShdFile),
	Clean,
}
pub use ShaderTask::*;

pub enum ShdState {
	Source { src: String },
	Compiled { src: String, obj: ShaderObj },
}
pub use ShdState::*;
