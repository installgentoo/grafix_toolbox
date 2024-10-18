use super::*;
use crate::math::*;

#[derive(Default, Debug, Clone)]
pub struct Model {
	idxs: Box<[u32]>,
	xyz: Box<[f32]>,
	uv: Box<[f16]>,
	norm: Box<[f16]>,
}
#[cfg(feature = "obj")]
impl Model {
	pub fn load_models(file: &str, scale: f32) -> Res<Vec<Self>> {
		let file = &format!("res/{file}.obj");
		let (models, _) = tobj::load_obj(
			file,
			&tobj::LoadOptions {
				single_index: true,
				triangulate: true,
				ignore_points: true,
				ignore_lines: true,
			},
		)
		.explain_err(|| format!("Cannot load models from {file:?}"))?;
		let models = models
			.into_iter()
			.map(|m| {
				let m = m.mesh;
				let (idxs, xyz, uv, mut norm) = (
					m.indices.into(),
					m.positions,
					m.texcoords.into_iter().map(f16).collect(),
					m.normals.iter().map(|&v| f16(v)).collect_vec(),
				);
				let (mut min, mut max) = ((0., 0., 0.), (0., 0., 0.));
				for i in (0..xyz.len()).step_by(3) {
					let v = Vec3(&xyz[i..]);
					min = min.fmin(v);
					max = max.fmax(v);
					if m.normals.is_empty() && i % 9 == 0 && i + 8 < xyz.len() {
						let xyz = &xyz[i..];
						let (v1, v2, v3) = vec3::<Vec3>::to((xyz, &xyz[3..], &xyz[6..]));
						let ndir = v1.sum(v2).sum(v3).div(3).sgn();
						let (v1, v2, v3) = vec3::<la::V3>::to((v1, v2, v3));
						let n = <[_; 3]>::to(hVec3(Vec3(la::normal(v1, v2, v3)).mul(ndir)));
						(0..9).for_each(|i| norm.push(n[i % 3]));
					}
				}
				let d: Vec3 = max.sub(min);
				let (center, scale) = (max.sum(min).div(2), (1., 1., 1.).div(d.x().max(d.y()).max(d.z())).mul(scale));
				let xyz = xyz.chunks(3).flat_map(|s| <[_; 3]>::to((Vec3(s)).sub(center).mul(scale)).to_vec()).collect();
				Self { idxs, xyz, uv, norm: norm.into() }
			})
			.collect();
		Ok(models)
	}
	#[cfg(feature = "adv_fs")]
	pub fn new_cached(name: &str) -> Res<Self> {
		let cache = format!("{name}.obj.z");
		if let Ok(d) = FS::Load::Archive(&cache) {
			if let Ok(model) = ser::SERDE::FromVec(&d) {
				return Ok(model);
			}
		}

		let model: Res<Self> = (|| {
			let m = Self::load_models(name, 1.)?.into_iter().next().ok_or("Empty models file")?;
			let _ = ser::SERDE::ToVec(&m).map(|v| FS::Save::Archive((cache, v, 22)));
			Ok(m)
		})();
		model
	}
}
impl<T: Borrow<Model>> From<T> for Mesh<u32, f32, f16, f16> {
	fn from(m: T) -> Self {
		let m = m.borrow();
		let (i, c, n) = (&m.idxs, &m.xyz, &m.norm);
		if m.uv.is_empty() {
			Self::new((i, c, n, gl::TRIANGLES))
		} else {
			Self::new((i, c, &m.uv, n, gl::TRIANGLES))
		}
	}
}

impl Mesh<u16, f32, f16, f32> {
	pub fn make_sphere(scale: f32, segs: u32) -> Self {
		let (xyz, uv) = {
			let (mut xyz, mut uv) = (vec![], vec![]);
			iter2d(0..1 + segs).for_each(|(x, y)| {
				let (sx, sy) = Vec2((x, y)).div(segs);
				let (rx, ry) = (sx.to_radians(), sy.to_radians());
				let (x, y, z) = ((rx * 360.).cos() * (ry * 180.).sin(), (ry * 180.).cos(), (rx * 360.).sin() * (ry * 180.).sin()).mul(scale);
				let (sx, sy) = hVec2((sx, sy).norm());
				xyz.extend(&[x, y, z]);
				uv.extend(&[sx, sy]);
			});
			(xyz, uv)
		};

		let idx = (0..segs)
			.flat_map(|y| {
				let s = segs + 1;
				let row = (0..s).flat_map(|x| vec![y * s + x, (y + 1) * s + x]);
				if y % 2 == y {
					row.collect_vec()
				} else {
					row.rev().collect_vec()
				}
			})
			.map(u16)
			.collect_vec();

		let draw = (u32(idx.len()), gl::TRIANGLE_STRIP);
		let idx = IdxArr::new(&idx[..]);
		let xyz = AttrArr::new(&xyz[..]);
		let uv = AttrArr::new(&uv[..]);

		let mut vao = Vao::new();
		vao.BindIdxs(&idx);
		vao.AttribFmt(&xyz, (0, 3));
		vao.AttribFmt(&uv, (1, 2));
		vao.AttribFmt(&xyz, (2, 3));
		let buff = (idx, xyz, Some(uv), Def());

		Self { vao, buff, draw }
	}
}

#[cfg(feature = "adv_fs")]
mod serde {
	use super::{ser::*, *};
	impl Serialize for Model {
		fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
			self.to_bytes().serialize(s)
		}
	}
	impl<'de> Deserialize<'de> for Model {
		fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
			Ok(Self::from_bytes(<&[u8]>::deserialize(d)?))
		}
	}
}

impl Model {
	pub fn to_bytes(&self) -> Box<[u8]> {
		let Self { idxs, xyz, uv, norm } = self;
		let il: [_; 8] = (idxs.len() * type_size::<u32>()).to_le_bytes();
		let cl: [_; 8] = (xyz.len() * type_size::<f32>()).to_le_bytes();
		let tl: [_; 8] = (uv.len() * type_size::<f16>()).to_le_bytes();
		let (_, i, _) = unsafe { idxs.align_to() };
		let (_, c, _) = unsafe { xyz.align_to() };
		let (_, t, _) = unsafe { uv.align_to() };
		let (_, n, _) = unsafe { norm.align_to() };
		[&il, &cl, &tl, i, c, t, n].concat().into()
	}
	pub fn from_bytes(v: &[u8]) -> Self {
		let il = 24 + usize::from_le_bytes(v[0..8].try_into().valid());
		let cl = il + usize::from_le_bytes(v[8..16].try_into().valid());
		let tl = cl + usize::from_le_bytes(v[16..24].try_into().valid());
		let idxs = unsafe { v[24..il].align_to() }.1.into();
		let xyz = unsafe { v[il..cl].align_to() }.1.into();
		let uv = unsafe { v[cl..tl].align_to() }.1.into();
		let norm = unsafe { v[tl..].align_to() }.1.into();
		Self { idxs, xyz, uv, norm }
	}
}
