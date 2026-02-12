use super::*;

pub struct Animation<'r, S: TexSize> {
	frames: Box<[(Vec2, VTex2dEntry<'r, S>)]>,
	c: Cell<usize>,
}
impl<'a, S: TexSize> Animation<'a, S> {
	pub fn from_file(name: &str, atlas: &'a TexAtlas<S>) -> Res<Self> {
		let anim_desc = format!("res/{name}/desc");
		let data = FS::Load::Text(&anim_desc)?;

		let mut time = 0.;
		let (starts, frames): (Vec<_>, Vec<_>) = data
			.lines()
			.filter_map(|l| {
				if "d " == l.slice(..2) {
					time += l.slice(2..).parse::<f32>().explain_err(|| format!("Malformed animation file {anim_desc:?}")).warn();
					None?
				}
				let t = atlas.load(&format!("{name}/{l}"));
				let f = (time, t);
				time += 1.;
				Some(f)
			})
			.collect();

		let frames = frames
			.into_iter()
			.zip(starts.iter().zip(starts.iter().chain(&[time]).skip(1)))
			.map(|(f, (&s, &e))| ((s, e).div(time), f))
			.collect_box();

		if frames.is_empty() {
			format!("Empty animation file {anim_desc}").pipe(Err)?
		}
		Self { frames, c: Def() }.pipe(Ok)
	}
	pub fn frame(&self, t: f32) -> &VTex2d<S, u8> {
		let Self { frames, c } = self;
		ASSERT!(t <= 1., "Animation assumes time in (0..1), given {t}");

		let n = c.get();
		for (n, ((b, e), guess)) in frames.iter().skip(n).chain(frames.iter().take(n)).enumerate() {
			if t >= *b && t <= *e {
				c.set(n);
				return guess.get();
			}
		}
		frames.last().map(|(_, tex)| tex).valid().get()
	}
}
