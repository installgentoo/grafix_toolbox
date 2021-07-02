use crate::uses::{math::*, slicing::*, *};
use crate::GL::{atlas::*, tex::*};

pub struct Animation<'a, S: TexSize> {
	frames: Vec<((f32, f32), AtlasTex2d<'a, S>)>,
	c: usize,
	a: Dummy<&'a u32>,
}
impl<'a, S: TexSize> Animation<'a, S> {
	pub fn from_file(name: &str, atlas: &'a TexAtlas<S>) -> Self {
		let anim_desc = CONCAT!("res/", name, "/desc");
		let data = EXPECT!(FS::Load::Text(&anim_desc));

		let mut time = 0.;
		let (starts, frames): (Vec<_>, Vec<_>) = data
			.lines()
			.filter_map(|l| {
				if "d " == slice((l, 2)) {
					time += EXPECT!(slice((2, l)).parse::<f32>());
					None
				} else {
					let t = atlas.load(&CONCAT!(name, "/", l));
					let f = (time, t);
					time += 1.;
					Some(f)
				}
			})
			.unzip();

		ASSERT!(!frames.is_empty(), "Empty animation file {}", anim_desc);
		let frames: Vec<_> = frames
			.into_iter()
			.zip(starts.iter().zip(starts.iter().chain(&[time]).skip(1)))
			.map(|(f, (&s, &e))| ((s, e).div(time), f))
			.collect();

		Self { frames, c: 0, a: Dummy }
	}
	pub fn frame(&mut self, t: f32) -> &VTex2d<S, u8> {
		let Self { frames, c, .. } = self;
		ASSERT!(t <= 1., "Animation assumes time in (0..1), given {}", t);

		for (n, ((b, e), guess)) in frames.iter().enumerate().skip(*c).chain(frames.iter().enumerate().take(*c)) {
			if t >= *b && t <= *e {
				*c = n;
				return guess.get();
			}
		}
		EXPECT!(frames.last()).1.get()
	}
}
