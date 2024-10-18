use super::*;

pub struct Sprite9<'r, S> {
	pub pos: Vec2,
	pub size: Vec2,
	pub corner: f32,
	pub color: Color,
	pub tex: &'r VTex2d<S, u8>,
}
impl<S: TexSize> Sprite9<'_, S> {
	pub fn compare(&self, crop: &Crop, r: &Sprite9Impl<S>) -> State {
		let &Self { pos, size, corner, color, tex: tex_new } = self;
		let xyzw = (State::XYZW | State::UV).or_def(geom_cmp(pos, size, crop, &r.base) || corner != r.corner);
		let rgba = State::RGBA.or_def(color != r.base.color);
		let ord = State::MISMATCH.or_def(!ptr::eq(r.tex, tex_new) || (!rgba.is_empty() && ordering_cmp::<S, _>(color, r)));
		ord | xyzw | rgba
	}
	pub fn obj(self, crop: Crop) -> Sprite9Impl<S> {
		let Self { pos, size, corner, color, tex } = self;
		Sprite9Impl { base: Base { pos, size, crop, color }, corner, tex }
	}
}
pub struct Sprite9Impl<S> {
	base: Base,
	corner: f32,
	tex: *const VTex2d<S, u8>,
}
impl<S: TexSize> Sprite9Impl<S> {
	pub fn batchable(&self, r: &Self) -> bool {
		self.ordered() == r.ordered() && atlas_cmp(self.tex, r.tex)
	}
}
impl<S: TexSize> Object for Sprite9Impl<S> {
	fn base(&self) -> &Base {
		&self.base
	}
	fn write_mesh(&self, to_clip: Vec2, range: BatchedObj) {
		let (crop, &Base { pos, size, color, .. }, (u1, v1, u2, v2)) = (self.base.bound_box(), self.base(), unsafe { &*self.tex }.region);
		let c = size.x().min(size.y()) * self.corner.min(0.5).max(0.);
		write_sprite9((to_clip, pos, size, (c, c), crop, (u1, v2, u2, v1), color), range);
	}
	fn batch_draw(&self, b: &VaoBinding<u16>, (offset, num): (u16, u16)) {
		let s = LocalStatic!(Shader, { Shader::pure([vs_gui__pos_col_tex, ps_gui__col_tex]) });

		let t = unsafe { &*self.tex }.atlas.Bind(sampler());
		let _ = Uniforms!(s, ("src", t));
		b.Draw((num, offset, gl::TRIANGLES));
	}

	fn vert_count(&self) -> u32 {
		16
	}
	fn ordered(&self) -> bool {
		S::TYPE == gl::RGBA || Object::ordered(self)
	}
	fn gen_idxs(&self, (start, size): (u16, u16)) -> Box<[u16]> {
		sprite9_idxs((start, size))
	}
}

type Sprite9Desc = (Vec2, Vec2, Vec2, Vec2, Crop, TexCoord, Color);
pub fn write_sprite9((to_clip, pos, size, corner, (crop1, crop2), (u1, v1, u2, v2), color): Sprite9Desc, BatchedObj { z, state, xyzw, rgba, uv }: BatchedObj) {
	if state.contains(State::XYZW) {
		let (((x1, y1), (x2, y2), (m1x, m1y), (m2x, m2y)), (u1, v1, u2, v2), (m1u, m1v, m2u, m2v)) = <_>::to({
			let (xy1, xy2) = (pos, pos.sum(size));
			let (m1, m2, ms) = (xy1.sum(corner), xy2.sub(corner), corner);
			let (uv, muv) = {
				let wh = (u2 - u1, v2 - v1).div(ms);
				let (u1m, v1m) = (u1, v1).sum(wh.mul(m1.sub(crop2)).mul(crop2.ls(m1)));
				let (u2m, v2m) = (u1, v1).sum(wh.mul(crop1.sub(m2)).mul(crop1.gt(m2)));
				let (u1, v1) = (u2, v2).sub(wh.mul(crop1.sub(xy1)));
				let (u2, v2) = (u2, v2).sub(wh.mul(xy2.sub(crop2)));
				((u1, v2, u2, v1), (u1m, v2m, u2m, v1m))
			};
			(
				(
					crop1.mul(to_clip),
					crop2.mul(to_clip),
					m1.clmp(crop1, crop2).mul(to_clip),
					m2.clmp(crop1, crop2).mul(to_clip),
				),
				uv,
				muv,
			)
		});
		let O = f16::ZERO;

		if state.contains(State::XYZW) {
			#[rustfmt::skip]
			xyzw[..64].copy_from_slice(&[x1, y1,  z, O,  m1x, y1,  z, O,  m2x, y1,  z, O,  x2, y1,  z, O,
										 x1, m1y, z, O,  m1x, m1y, z, O,  m2x, m1y, z, O,  x2, m1y, z, O,
										 x1, m2y, z, O,  m1x, m2y, z, O,  m2x, m2y, z, O,  x2, m2y, z, O,
										 x1, y2,  z, O,  m1x, y2,  z, O,  m2x, y2,  z, O,  x2, y2,  z, O]);
		}

		if state.contains(State::UV) {
			#[rustfmt::skip]
			uv[..32].copy_from_slice(&[u1, v2,   m1u, v2,   m2u, v2,   u2, v2,
									   u1, m2v,  m1u, m2v,  m2u, m2v,  u2, m2v,
									   u1, m1v,  m1u, m1v,  m2u, m1v,  u2, m1v,
									   u1, v1,   m1u, v1,   m2u, v1,   u2, v1]);
		}
	}

	if state.contains(State::RGBA) {
		let (r, g, b, a) = vec4::to(color.mul(255).clmp(0, 255).round());
		#[rustfmt::skip]
		rgba[..64].copy_from_slice(&[r, g, b, a,  r, g, b, a,  r, g, b, a,  r, g, b, a,
									 r, g, b, a,  r, g, b, a,  r, g, b, a,  r, g, b, a,
									 r, g, b, a,  r, g, b, a,  r, g, b, a,  r, g, b, a,
									 r, g, b, a,  r, g, b, a,  r, g, b, a,  r, g, b, a]);
	}
}

pub fn sprite9_idxs((start, size): (u16, u16)) -> Box<[u16]> {
	(start..(start + size))
		.step_by(16)
		.flat_map(|i| {
			let s = |j| i + j;
			#[rustfmt::skip] let s =
			[s(0), s(1), s(4), s(4), s(1), s(5),     s(5), s(1), s(2), s(2), s(5), s(6),       s(6), s(2), s(3), s(3), s(6), s(7),
			 s(7), s(6), s(11), s(11), s(6), s(10),  s(10), s(6), s(5), s(5), s(10), s(9),     s(9), s(5), s(4), s(4), s(9), s(8),
			 s(8), s(9), s(12), s(12), s(9), s(13),  s(13), s(9), s(10), s(10), s(13), s(14),  s(14), s(10), s(11), s(11), s(14), s(15)];
			s
		})
		.collect()
}
