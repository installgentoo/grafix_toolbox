use crate::uses::{math::*, *};

pub fn pack(w: i32, h: i32, empty: &mut Vec<Rect>, filled: &mut Vec<Rect>, min: iVec2) -> Res<Rect> {
	let (b, min_y) = Res(empty
		.iter()
		.filter(|e| e.w >= w && e.h >= h)
		.map(|e| {
			let sum = filled
				.iter()
				.fold(0, |sum, f| sum + i32(e.x == f.x2() && ((f.y - e.y) * 2 + f.h - e.h).abs() < f.h.max(e.h)));
			(e, e.y - 2 * sum)
		})
		.min_by(|(_, l_sum), (_, r_sum)| l_sum.cmp(r_sum)))?;

	let x = if b.y != min_y { b.x } else { b.x2() - w };
	filled.push(Rect { x, y: b.y, w, h });

	let b = *filled.last().valid();
	let mut i = 0;
	while i < empty.len() {
		let e = empty[i];
		if b.intersects(&e) {
			let mut push = |cond, x, y, w, h| {
				if cond && (w, h).ge(min).all() {
					empty.push(Rect { x, y, w, h })
				}
			};
			#[rustfmt::skip] push(b.x2() < e.x2(), b.x2(), e.y,    e.x2() - b.x2(), e.h);
			#[rustfmt::skip] push(b.y2() < e.y2(), e.x,    b.y2(), e.w,             e.y2() - b.y2());
			#[rustfmt::skip] push(b.x > e.x,       e.x,    e.y,    b.x - e.x,       e.h);
			#[rustfmt::skip] push(b.y > e.y,       e.x,    e.y,    e.w,             b.y - e.y);
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
	fn bb(&self) -> (iVec2, iVec2) {
		let b1 = (self.x, self.y);
		(b1, b1.sum((self.w, self.h)))
	}
	fn intersects(&self, r: &Rect) -> bool {
		let (b1, b2) = self.bb();
		let (r_b1, r_b2) = r.bb();
		!(b2.le(r_b1).any() || b1.ge(r_b2).any())
	}
	fn contains(&self, r: &Rect) -> bool {
		let (b1, b2) = self.bb();
		let (r_b1, r_b2) = r.bb();
		!(r_b1.ls(b1).any() || r_b2.gt(b2).any())
	}
}
