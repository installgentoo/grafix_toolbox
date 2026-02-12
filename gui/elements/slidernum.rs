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
}
impl Default for SliderNum {
	fn default() -> Self {
		let (last_value, step) = (f32::NAN, Mod::empty());
		let (slider, num, inc, dec, value) = Def();
		Self { slider, num, inc, dec, value, last_value, step }
	}
}
impl SliderNum {
	pub fn draw<'s: 'l, 'l>(&'s mut self, r: &mut RenderLock<'l>, t: &'l Theme, layout @ Surf { size, .. }: Surf, (min, max): Vec2) -> f32 {
		ASSERT!(min < max, "SliderNum range expects min < max");
		let side = size.min_comp();
		let (format, range, half_side) = (|v: &f32| format(*v, min < 0.), max - min, side * 0.5);

		let Self { slider, num, inc, dec, value, last_value, step } = self;

		let ss = layout.w_sub(half_side);
		if size.x() < size.y() {
			let v = slider.draw(r, t, ss, side) * range + min;
			*value = v;
			return v;
		}

		let adv_step = move |step: &Mod| {
			1.0.or_val(range > 2., || {
				let s = half_side / range;
				let mag = 10f32.powf(s.abs().log10().floor());
				(s / mag).round() * mag
			})
			.or_map(|_| !step.ctrl(), |s| s * 0.1.or_val(range < 1., || 20.))
			.or_map(|_| !step.shift(), |s| s * 0.01.or_val(range < 100., || 200.))
		};

		let sl = ss.size((3., 0.9).mul(side));
		let sb = ss.x_self(1).size((half_side, half_side));

		if dec.draw(r, t, sb, "-") {
			*value -= adv_step(step);
		}
		if inc.draw(r, t, sb.y(half_side), "+") {
			*value += adv_step(step);
		}

		*value = value.clamp(min, max);

		let (editing, edited) = num.edited(r);

		if value != last_value {
			num.text = format(value).into();
		} else if editing {
			num.text = "".into();
		} else if edited {
			*value = num.text.trim_start().parse::<f32>().map(|n| n.clamp(min, max)).unwrap_or(*value);
			num.text = format(value).into();
		}
		slider.pip_pos = slider.pip_pos.or_val(value == last_value, || (*value - min) / range);

		*last_value = *value;

		let v = slider.draw(r, t, ss, side) * range + min;

		*value = value.or_val(value.eps_eq(v), || v);

		num.draw(r, t, sl, NumericOnly().pipe(Some));

		r.logic(
			layout,
			move |e, _, _| {
				if let &MouseButton { m, .. } = e
					&& m.pressed()
				{
					*step = m
				}

				Pass
			},
			0,
		);
		*value
	}
}

impl<'s: 'l, 'l> Lock::SliderNum<'s, 'l, '_> {
	pub fn draw<O>(self, g: impl Into<Surf>, l: O) -> f32
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
			return n.or_map(|_| signed, |n| n[1..].into());
		}

		if n.fract().abs() < min_frac {
			return format!("{n:.0}");
		}

		let (mag, slots) = iVec2((n.abs().log10(), slots - 2));
		let slots = (slots - mag).clamp(0, slots);
		let n = format!("{n:.*}", usize(slots));
		n.or_map(|_| slots < 1, |n| n.trim_end_matches('0').into())
	})();

	let l = n.len();
	n.or_map(|_| l >= max_len, |n| [n, " ".repeat(max_len - l)].concat())
}
