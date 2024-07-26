use super::{object::*, parsing::*, *};
use GL::Fence;

pub fn compiler(data_rx: Receiver<ShaderTask>, res_sn: Sender<(ShdResult, Fence)>) {
	let mut includes: String = Def();
	let mut files: Vec<Lazy<Vec<ShdSrc>>> = Def();
	let mut sources: HashMap<Str, ShdState> = Def();
	let mut watched: HashMap<Str, Vec<ShdName>> = Def();

	while let Ok(msg) = data_rx.recv() {
		fn coerce<T1, T2, T3, F: Fn(T1, &mut T2, &mut [T3])>(f: F) -> F {
			f
		}
		let send = coerce(|name: ShdName, s, f: &mut [_]| {
			let prog = compile(&includes, s, f, &name).map_err(|e| adjust_log(e, -i32(includes.lines().count())));
			res_sn.send((ShdResult { name, prog }, Fence::new())).fail();
		});

		match msg {
			Includes(i) => includes = parse_includes(i).warn(),
			Watch(name) => name.iter().for_each(|n| watched.entry(n.clone()).or_default().push(name.clone())),
			Forget(name) => name.iter().for_each(|n| {
				let w = watched.get_mut(n).valid();
				let idx = w.iter().position(|n| *n == name).valid();
				w.swap_remove(idx);
			}),
			Rebuild => {
				let recipients = files
					.iter_mut()
					.flat_map(|p| {
						if !p.changed() {
							return vec![];
						}

						mem::take(p.get())
							.drain(..)
							.filter_map(|ShdSrc { name, src, .. }| {
								if let Some(s) = sources.get_mut(&name) {
									if matches!(s, Source { src: s } | Compiled { src: s, .. } if *s == src) {
										return None;
									}

									*s = Source { src };
									return Some(name);
								}

								sources.insert(name, Source { src });
								None
							})
							.collect()
					})
					.collect_vec();

				for name in recipients {
					if let Some(recipients) = watched.get(&name) {
						for name in recipients {
							send(name.clone(), &mut sources, &mut files);
						}
					}
				}
			}
			Inline((name, source)) => {
				ASSERT!(
					sources.get(&name).map(|(Source { src } | Compiled { src, .. })| *src == source).unwrap_or(true),
					"Shader {name:?} already exists"
				);

				sources.insert(name, Source { src: source });
			}
			Create(name) => send(name, &mut sources, &mut files),
			Load(file) => files.push(file),
			Clean => sources.iter_mut().for_each(|(_, v)| {
				if let Compiled { src, .. } = v {
					*v = Source { src: mem::take(src) };
				}
			}),
		}
	}
}
fn compile(includes: &str, sources: &mut HashMap<Str, ShdState>, files: &mut [Lazy<Vec<ShdSrc>>], name: &[Str]) -> Res<ShdProg> {
	let get_object = |name: &Str| -> Res<_> {
		let get = |s: &mut HashMap<_, _>| {
			if let Some(state) = s.get_mut(name) {
				match state {
					Compiled { obj, .. } => return Ok(obj.obj()),
					Source { src } => {
						let c_src = CString::new([includes, src].concat()).explain_err(|| format!("Malformed string in shader {name:?}")).fail();
						let (o, new) = ShaderObj::new(name, &c_src).map(|obj| (obj.obj(), Compiled { src: mem::take(src), obj }))?;
						*state = new;
						return Ok(o);
					}
				}
			}
			Err(format!("No shader {name:?} in loaded sources"))
		};

		if let Ok(o) = get(sources) {
			return Ok(o);
		}

		files.iter_mut().for_each(|p| {
			mem::take(p.get())
				.drain(..)
				.for_each(|ShdSrc { name, src, .. }| sources.insert(name.clone(), Source { src }).map(|_| WARN!("Replacing shader source {name:?}")).sink())
		});

		get(sources)
	};

	let objects = name.iter().map(get_object).collect::<Res<Vec<_>>>()?;
	let prog = Object::new();
	let obj = prog.obj;

	objects.iter().for_each(|&o| GL!(gl::AttachShader(obj, o)));
	GL!(gl::LinkProgram(obj));
	let mut status: i32 = 0;
	GL!(gl::GetProgramiv(obj, gl::LINK_STATUS, &mut status));
	objects.iter().for_each(|&o| GL!(gl::DetachShader(obj, o)));

	if GLbool::to(status) == gl::FALSE {
		return Err(format!("Error linking program {:?}, {obj}\n{}", name.join(" "), print_shader_log(obj)));
	}

	DEBUG!("Compiled shader {}:{:?}", prog.obj, name.join(" "));
	Ok(prog)
}

pub type ShdName = Box<[Str]>;
pub type ShdProg = Object<ShaderProg>;

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
	Load(Lazy<Vec<ShdSrc>>),
	Clean,
}
pub use ShaderTask::*;

pub enum ShdState {
	Source { src: String },
	Compiled { src: String, obj: ShaderObj },
}
pub use ShdState::*;
