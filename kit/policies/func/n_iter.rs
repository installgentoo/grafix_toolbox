#![allow(private_bounds)]
use crate::lib::*;

pub fn iter2d<T: Rangeable>(args: impl IterArgs2d<T>) -> impl Iterator<Item = vec2<T>> {
	let (wr, h) = args.get();
	let (mut x, w, mut y, h) = (wr.start, wr.end, h.start, h.end);
	let ident = T::to(1);
	iter::from_fn(move || {
		if y >= h {
			None?
		}
		let ret = Some((x, y));

		x += ident;
		if x >= w {
			x = wr.start;
			y += ident;
		}
		ret
	})
}

pub fn iter3d<T: Rangeable>(args: impl IterArgs3d<T>) -> impl Iterator<Item = vec3<T>> {
	let (wr, hr, d) = args.get();
	let (mut x, w, mut y, h, mut z, d) = (wr.start, wr.end, hr.start, hr.end, d.start, d.end);
	let ident = T::to(1);
	iter::from_fn(move || {
		if z >= d {
			None?
		}
		let ret = Some((x, y, z));

		x += ident;
		if x >= w {
			x = wr.start;
			y += ident;
		}
		if y >= h {
			y = hr.start;
			z += ident;
		}
		ret
	})
}

type Args2d<T> = (R<T>, R<T>);
trait IterArgs2d<T> {
	fn get(self) -> Args2d<T>;
}
impl<T> IterArgs2d<T> for Args2d<T> {
	fn get(self) -> Self {
		self
	}
}
impl<T: Rangeable> IterArgs2d<T> for R<T> {
	fn get(self) -> Args2d<T> {
		(self.clone(), self)
	}
}
impl<T: Rangeable> IterArgs2d<T> for T {
	fn get(self) -> Args2d<T> {
		(Def()..self, Def()..self)
	}
}
impl<T: Rangeable> IterArgs2d<T> for (T, T) {
	fn get(self) -> Args2d<T> {
		(Def()..self.0, Def()..self.1)
	}
}

type Args3d<T> = (R<T>, R<T>, R<T>);
trait IterArgs3d<T> {
	fn get(self) -> Args3d<T>;
}
impl<T> IterArgs3d<T> for Args3d<T> {
	fn get(self) -> Self {
		self
	}
}
impl<T: Rangeable> IterArgs3d<T> for R<T> {
	fn get(self) -> Args3d<T> {
		(self.clone(), self.clone(), self)
	}
}
impl<T: Rangeable> IterArgs3d<T> for T {
	fn get(self) -> Args3d<T> {
		(Def()..self, Def()..self, Def()..self)
	}
}
impl<T: Rangeable> IterArgs3d<T> for (T, T, T) {
	fn get(self) -> Args3d<T> {
		(Def()..self.0, Def()..self.1, Def()..self.2)
	}
}

type R<T> = ops::Range<T>;
trait_alias!(Rangeable, Default + ops::AddAssign + Copy + PartialOrd + Cast<u32>);
