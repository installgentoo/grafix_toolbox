use crate::uses::*;

pub fn pack(w: i32, h: i32, empty: &mut Vec<Rect>, filled: &mut Vec<Rect>, min_w: i32, min_h: i32) -> Res<Rect> {
	let (b, min_y) = PASS!(empty
		.iter()
		.filter(|e| e.w >= w && e.h >= h)
		.map(|e| {
			let sum = filled
				.iter()
				.fold(0, |sum, f| sum + (e.x == f.x2() && ((f.y - e.y) * 2 + f.h - e.h).abs() < f.h.max(e.h)) as i32);
			(e, e.y - 2 * sum)
		})
		.min_by(|(_, l_sum), (_, r_sum)| l_sum.cmp(r_sum)));

	let x = if b.y != min_y { b.x } else { b.x2() - w };
	filled.push(Rect { x, y: b.y, w, h });

	let b = *filled.last().unwrap();
	let mut i = 0;
	while i < empty.len() {
		let e = empty[i];
		if b.intersects(&e) {
			#[cfg_attr(rustfmt, rustfmt::skip)] {
			let mut push = |cond, x, y, w, h| { if cond && w >= min_w && h >= min_h { empty.push(Rect { x, y, w, h }) } };

			push(b.x2() < e.x2(), b.x2(), e.y,    e.x2() - b.x2(), e.h);
			push(b.y2() < e.y2(), e.x,    b.y2(), e.w,             e.y2() - b.y2());
			push(b.x > e.x,       e.x,    e.y,    b.x - e.x,       e.h);
			push(b.y > e.y,       e.x,    e.y,    e.w,             b.y - e.y); }
			empty.remove(i);
		} else {
			i += 1;
		}
	}

	empty.sort_unstable_by(|l, r| if l.contains(r) { ord::Less } else { ord::Greater });
	empty.dedup_by(|r, l| l.contains(r));

	let mut i = 0;
	while i < empty.len() {
		let p = empty[i];
		let mut j = i + 1;
		while j < empty.len() {
			if p.contains(&empty[j]) {
				empty.swap_remove(j);
			}
			j += 1;
		}
		i += 1;
	}
	Ok(b)
}

#[derive(Clone, Copy)]
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
	fn unpack(&self) -> iVec4 {
		(self.x, self.y, self.w, self.h)
	}
	fn intersects(&self, r: &Rect) -> bool {
		let (x, y, w, h) = self.unpack();
		!((x + w <= r.x) || (x >= r.x + r.w) || (y + h <= r.y) || (y >= r.y + r.h))
	}
	fn contains(&self, r: &Rect) -> bool {
		let (x, y, w, h) = self.unpack();
		!((r.x < x) || (r.x + r.w > x + w) || (r.y < y) || (r.y + r.h > y + h))
	}
}
