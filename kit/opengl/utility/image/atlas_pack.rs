use super::*;

pub fn pack(w: i32, h: i32, empty: &mut Vec<Rect>, filled: &mut Vec<Rect>, min: iVec2) -> Res<Rect> {
	let (b, min_y) = empty
		.iter()
		.filter(|e| e.w >= w && e.h >= h)
		.map(|e| {
			let sum = filled
				.iter()
				.fold(0, |sum, f| sum + i32(e.x == f.x2() && ((f.y - e.y) * 2 + f.h - e.h).abs() < f.h.max(e.h)));
			(e, e.y - 2 * sum)
		})
		.min_by(|(_, l_sum), (_, r_sum)| l_sum.cmp(r_sum))
		.res()?;

	let x = b.x.or_val(b.y != min_y, || b.x2() - w);
	filled.push(Rect { x, y: b.y, w, h });

	let &b = filled.last().valid();
	(0..empty.len()).for_each(|i| {
		let &e = empty.at(i);

		if !intersects(b.bb(), e.bb()) {
			return;
		}

		let mut push = |cond, x, y, w, h| {
			if cond && (w, h).ge(min).all() {
				empty.push(Rect { x, y, w, h })
			}
		};
		#[rustfmt::skip] push(b.x2() < e.x2(), b.x2(), e.y,    e.x2() - b.x2(), e.h);
		#[rustfmt::skip] push(b.y2() < e.y2(), e.x,    b.y2(), e.w,             e.y2() - b.y2());
		#[rustfmt::skip] push(b.x > e.x,       e.x,    e.y,    b.x - e.x,       e.h);
		#[rustfmt::skip] push(b.y > e.y,       e.x,    e.y,    e.w,             b.y - e.y);
		*empty.at_mut(i) = Def();
	});

	empty.retain(|e| e != &Def());
	empty.dedup_by(|a, b| contains(b.bb(), a.bb()));

	Ok(b)
}

#[derive_as_trivial]
pub struct Rect {
	pub x: i32,
	pub y: i32,
	pub w: i32,
	pub h: i32,
}
impl Rect {
	pub fn x2(&self) -> i32 {
		self.x + self.w
	}
	pub fn y2(&self) -> i32 {
		self.y + self.h
	}
	fn bb(&self) -> (iVec2, iVec2) {
		let b1 = (self.x, self.y);
		(b1, b1.sum((self.w, self.h)))
	}
}
