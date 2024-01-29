use super::{atlas_pack::*, vtex::*};
use crate::{lib::*, math::*, GL::tex::*};
use std::{fmt, hash};

pub fn pack_into_atlas<K, T: Tile<F>, S: TexSize, F: TexFmt>(mut tiles: Vec<(K, T)>, max_w: i32, max_h: i32) -> (Atlas<K, S, F>, Vec<(K, T)>)
where
	K: fmt::Debug + Clone + Eq + hash::Hash,
{
	if tiles.is_empty() {
		FAIL!("Vector supplied to atlas is empty");
		return Def();
	}

	tiles.sort_by(|(_, l), (_, r)| if l.h() != r.h() { r.h().cmp(&l.h()) } else { r.w().cmp(&l.w()) });
	ASSERT!(
		tiles.iter().map(|(l, _)| l).collect::<HashSet<_>>().len() == tiles.len(),
		"Vector supplied to atlas needs unique keys"
	);

	let max_w = {
		let area = tiles.iter().fold(0, |v, (_, t)| v + usize(t.w()) * usize(t.h()));
		max_w.min(i32(2_u32.pow(u32(f64(area).sqrt().log2().ceil()))))
	};
	let (min_w, min_h) = (tiles.iter().map(|(_, e)| e.w()).min().valid(), tiles.last().valid().1.h());

	let (c, empty, filled) = (S::SIZE, &mut vec![Rect { x: 0, y: 0, w: max_w, h: max_h }], &mut vec![]);
	let (mut tail, mut atlas, mut packed) = (vec![], vec![], HashMap::new());

	let mut tiles = tiles.into_iter().map(Some).collect_vec();
	for i in 0..tiles.len() {
		if let Some((id, img)) = &tiles[i] {
			let duplicate = tiles[..i].iter().rev().flatten().take_while(|(_, e)| e.h() == img.h()).find(|(_, e)| *img == *e);
			if let Some((i, _)) = duplicate {
				packed.insert(id.clone(), *packed.get(i).valid());
				DEBUG!("Deduped {i:?}, {id:?} in atlas");
				continue;
			}

			if let Ok(b) = pack(img.w(), img.h(), empty, filled, (min_w, min_h)) {
				let (x, y, w, h) = (b.x, b.y, b.w, b.h);
				packed.insert(id.clone(), (x, y + h, x + w, y));
				atlas.resize(atlas.len().max(usize(b.y2() * max_w * c)), Def());

				for i in 0..h {
					let d = img.data();
					let b = usize(((y + i) * max_w + x) * c);
					let w = usize(w * c);
					let x = usize(i) * w;
					atlas[b..b + w].copy_from_slice(&d[x..x + w])
				}
			} else if let Some((id, img)) = tiles[i].take() {
				tail.push((id, img));
			};
		}
	}

	let max_h = atlas.len() / usize(max_w * c);

	let tex = Rc::new(Tex2d::<S, F>::new((max_w, max_h), &atlas[..]));
	let packed = packed
		.into_iter()
		.map(|(id, reg)| {
			let region = (0.5, -0.5, 0.5, -0.5).sum(reg).div((max_w, max_h, max_w, max_h));
			(id, VTex2d { region, tex: tex.clone() })
		})
		.collect();

	(packed, tail)
}

type Atlas<K, S, F> = HashMap<K, VTex2d<S, F>>;

pub trait Tile<T>: Eq {
	fn w(&self) -> i32;
	fn h(&self) -> i32;
	fn data(&self) -> &[T];
}
