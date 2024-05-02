use crate::{lib::*, math::*, slicing::*, *};
use GL::{atlas::*, tex::*};

pub struct Animation<'a, S: TexSize> {
	frames: Box<[(Vec2, VTex2dEntry<'a, S>)]>,
	c: Cell<usize>,
	a: Dummy<&'a u32>,
}
impl<'a, S: TexSize> Animation<'a, S> {
	pub fn from_file(name: &str, atlas: &'a TexAtlas<S>) -> Res<Self> {
		let anim_desc = format!("res/{name}/desc");
		let data = FS::Load::Text(&anim_desc)?;

		let mut time = 0.;
		let (starts, frames): (Vec<_>, Vec<_>) = data
			.lines()
			.filter_map(|l| {
				if "d " == slice((l, 2)) {
					time += OR_DEFAULT!(slice((2, l)).parse::<f32>(), "Animation {anim_desc} has invalid format, {}");
					None
				} else {
					let t = atlas.load(&format!("{name}/{l}"));
					let f = (time, t);
					time += 1.;
					Some(f)
				}
			})
			.unzip();

		let frames = frames
			.into_iter()
			.zip(starts.iter().zip(starts.iter().chain(&[time]).skip(1)))
			.map(|(f, (&s, &e))| ((s, e).div(time), f))
			.collect_box();

		if frames.is_empty() {
			Err(format!("Empty animation file {anim_desc}"))
		} else {
			Ok(Self { frames, c: Cell::new(0), a: Dummy })
		}
	}
	pub fn frame(&self, t: f32) -> &VTex2d<S, u8> {
		let Self { ref frames, c, .. } = self;
		ASSERT!(t <= 1., "Animation assumes time in (0..1), given {t}");

		let n = c.get();
		for (n, ((b, e), guess)) in frames.iter().skip(n).chain(frames.iter().take(n)).enumerate() {
			if t >= *b && t <= *e {
				c.replace(n);
				return guess.get();
			}
		}
		frames.last().valid().1.get()
	}
}
