use super::*;

#[derive(Debug)]
pub struct SliderNum {
	slider: Slider,
	num: LineEdit,
	inc: Button,
	dec: Button,
	pub value: f32,
	last_value: f32,
	step: Mod,
	num_edited: bool,
}
impl Default for SliderNum {
	fn default() -> Self {
		let (last_value, step) = (f32::NAN, Mod::empty());
		let (slider, num, inc, dec, value, num_edited) = Def();
		Self { slider, num, inc, dec, value, last_value, step, num_edited }
	}
}
impl SliderNum {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, Surface { pos, size }: Surface, (min, max): Vec2) -> f32 {
		ASSERT!(size.x() > size.y(), "Not impl");
		ASSERT!(min < max, "SliderNum range expects min < max");
		let id = ref_UUID(self);

		let Self { slider, num, inc, dec, value, last_value, step, num_edited } = self;
		let side = size.y().min(size.x());
		let range = max - min;

		let adv_step = move |step: &Mod| {
			let mut s = if range < 2. {
				let s = side * 0.5 / range;
				let mag = 10f32.powf(s.abs().log10().floor());
				(s / mag).round() * mag
			} else {
				1.
			};
			if step.ctrl() {
				if range < 1. {
					s /= 10.;
				} else {
					s *= 10.;
				}
			}
			if step.shift() {
				if range <= 100. {
					s /= 100.;
				} else {
					s *= 100.;
				}
			}
			s
		};

		let ss = Surface { pos, size: size.sub((side * 0.5, 0)) };
		let sl = ss.size((side * 3., side * 0.8));
		let sb = ss.x(ss.size.x()).size((side, side).mul(0.5));

		if dec.draw(r, t, sb, "-") {
			*value -= adv_step(step);
		}
		if inc.draw(r, t, sb.y(side * 0.5), "+") {
			*value += adv_step(step);
		}

		*value = value.clamp(min, max);

		let signed = min < 0.;
		let editing = r.focused(ref_UUID(num));
		*num_edited |= num.text.check();

		if value != last_value {
			*num.text.str() = format(*value, signed);
			*num_edited = false;
		} else if editing && !*num_edited {
			*num.text.str() = "".into();
		} else if !editing && *num_edited {
			if let Ok(n) = num.text.trim_start().parse::<f32>() {
				*value = n.clamp(min, max);
			}
			*num.text.str() = format(*value, signed);
			*num_edited = false;
		}

		if value != last_value {
			slider.pip_pos = (*value - min) / range;
		}
		*last_value = *value;

		let v = slider.draw(r, t, ss, side) * range + min;

		if !editing && !value.eps_eq(v) {
			*value = v;
		}

		num.draw(r, t, sl, Some(&NumericOnly()));

		let v = *value;
		r.logic(
			(pos, pos.sum(size)),
			move |e, _, _| {
				match *e {
					MouseButton { state, .. } if state.pressed() => *step = state,
					Keyboard { key, state } if state.pressed() => {
						*step = state;
						let step = adv_step(step);
						match key {
							Key::Left | Key::Down | Key::Minus => *value -= step,
							Key::Right | Key::Up | Key::Equal => *value += step,
							_ => (),
						}
					}
					_ => (),
				}
				Pass
			},
			id,
		);
		v
	}
}

impl<'s: 'l, 'l> Lock::SliderNum<'s, 'l, '_> {
	pub fn draw<O>(self, g: impl Into<Surface>, l: O) -> f32
	where
		Vec2: Cast<O>,
	{
		let Self { s, r, t } = self;
		s.draw(r, t, g.into(), Vec2(l))
	}
}

fn format(n: f32, signed: bool) -> String {
	let max_len = 6;

	let slots = max_len - u32(signed);
	let max_len = usize(max_len);

	let n = (|| {
		if n == 0. {
			return "0".into();
		}

		let min_frac = 10_f32.powi(-i32(slots - 2));
		if n >= f32(10_u32.pow(slots)) || n.abs() <= min_frac {
			let n = format!("{n:+.*e}", usize(slots - 4));
			return if signed { n } else { n[1..].into() };
		}

		if n.fract().abs() < min_frac {
			return format!("{n:.0}");
		}

		let mag = i32(n.abs().log10().floor());
		let slots = i32(slots - 2);
		let slots = (slots - mag).clamp(0, slots);
		let n = format!("{n:.*}", usize(slots));
		if slots < 1 {
			n
		} else {
			n.trim_end_matches('0').into()
		}
	})();

	if n.len() < max_len {
		[" ".repeat(max_len - n.len()), n].concat()
	} else {
		n
	}
}
