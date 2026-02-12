pub mod prim {
	pub use super::{Frame9, Rect, Sprite, Sprite9, Text};
}

pub use {frame9::*, rect::*, sprite::*, sprite9::*, text::*};

mod frame9;
mod rect;
mod sprite;
mod sprite9;
mod text;

pub trait Primitive {
	fn base(&self) -> &Base;
	fn write_mesh(&self, aspect: Vec2, _: BatchedObj);
	fn batch_draw(&self, _: &VaoBind<u16>, range: (u16, u16));

	fn vert_count(&self) -> u32 {
		4
	}
	fn ordered(&self) -> bool {
		true
	}
	fn gen_idxs(&self, (start, size): (u16, u16)) -> Box<[u16]> {
		rect_idxs((start, size))
	}
}
pub struct BatchedObj<'a> {
	pub z: f16,
	pub state: State,
	pub xyzw: &'a mut [f16],
	pub rgba: &'a mut [u8],
	pub uv: &'a mut [f16],
}
bitflags! {#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct State: u8 {
	const BATCH_RESIZED = 0x2;
	const XYZW = 0x10;
	const RGBA = 0x20;
	const UV = 0x40;
	const FULL = Self::XYZW.bits() | Self::RGBA.bits() | Self::UV.bits();
	const MISMATCH = 0x1 | Self::FULL.bits() | Self::BATCH_RESIZED.bits();
}}

#[derive(Debug)]
pub struct Base {
	pos: Vec2,
	size: Vec2,
	crop: Geom,
	color: Color,
}
impl Base {
	pub fn bound_box(&self) -> Geom {
		let Self { pos, size, crop: (p1, p2), .. } = *self;
		(pos.clmp(p1, p2), pos.sum(size).clmp(p1, p2))
	}
	pub fn intersects(&self, r: &Self) -> bool {
		intersects(self.bound_box(), r.bound_box())
	}
}

fn ordered(c: Color) -> bool {
	c.3 < 0.996
}

fn geom_cmp(pos: Vec2, size: Vec2, bb: &Geom, r: &Base) -> bool {
	pos != r.pos || size != r.size || *bb != r.crop
}

fn ordering_cmp<S: TexSize, T: Primitive>(c: Color, r: &T) -> bool {
	(S::TYPE == gl::RGBA || ordered(c)) != r.ordered()
}

fn atlas_cmp<S, F>(l: *const VTex2d<S, F>, r: *const VTex2d<S, F>) -> bool {
	unsafe { (*l).eq_atlas(&*r) }
}

fn bound_uv(crop @ (p1, p2): Geom, base @ (xy1, xy2): Geom, uv @ (u1, v1, u2, v2): TexCoord) -> TexCoord {
	if contains(crop, base) {
		return uv;
	}

	if !intersects(crop, base) {
		return Def();
	}

	let wh = (u2 - u1, v2 - v1).div(xy2.sub(xy1));
	let (u1, v1) = (u1, v1).sum(wh.mul(p1.sub(xy1)).mul(p1.gt(xy1)));
	let (u2, v2) = (u2, v2).sub(wh.mul(xy2.sub(p2)).mul(p2.ls(xy2)));
	(u1, v1, u2, v2)
}

fn rect_idxs((start, size): (u16, u16)) -> Box<[u16]> {
	(start..(start + size)).step_by(4).flat_map(|i| [i, i + 1, i + 3, i + 3, i + 1, i + 2]).collect()
}

use super::*;
