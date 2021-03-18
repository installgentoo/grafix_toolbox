use super::mesh::*;
use crate::uses::{math::*, serde_impl::*, GL::buffer::*, *};

#[derive(Default)]
pub struct Model {
	idxs: Vec<u32>,
	xyz: Vec<f32>,
	uv: Vec<f16>,
	norm: Vec<f16>,
}
impl Model {
	pub fn load_models(file: &str, scale: f32) -> Res<Vec<Model>> {
		let file = &CONCAT!["res/", file, ".obj"];
		let (models, _) = tobj::load_obj(file, true).map_err(|e| format!("Can't load models in {}, {:?}", file, e))?;
		let models = models
			.into_iter()
			.map(|m| {
				let mesh = m.mesh;
				ASSERT!(!mesh.num_face_indices.iter().any(|n| *n != 3), "Model was not triangulated!");
				let idxs = (0..mesh.num_face_indices.len()).flat_map(|f| mesh.indices[f * 3..f * 3 + 3].to_vec()).collect::<Vec<_>>();
				let (mut xyz, mut uv, mut norm) = (vec![], vec![], vec![]);
				let (mut min, mut max) = ((0., 0., 0.), (0., 0., 0.));
				for i in 0..mesh.positions.len() / 3 {
					let i = (i * 3) as usize;
					let c = &mesh.positions[i..i + 3];
					xyz.extend(c);
					let v = Vec3::to(&mesh.positions[i..]);
					min = min.fmin(v);
					max = max.fmax(v);
					if !mesh.texcoords.is_empty() {
						let t = mesh.texcoords[i..i + 3].iter().map(|v| f16::to(*v));
						uv.extend(t);
					}
					if !mesh.normals.is_empty() {
						let n = mesh.normals[i..i + 3].iter().map(|v| f16::to(*v));
						norm.extend(n);
					} else if i % 9 == 0 && i + 8 < mesh.positions.len() {
						let xyz = &mesh.positions[i..];
						let (v1, v2, v3) = (Vec3::to(&xyz[..]), Vec3::to(&xyz[3..]), Vec3::to(&xyz[6..]));
						use glm::Vec3 as V3;
						let ndir = v1.sum(v2).sum(v3).div(3).sgn();
						let n = <[f16; 3]>::to(vec3::<f16>::to(Vec3::to(glm::triangle_normal(&V3::to(v1), &V3::to(v2), &V3::to(v3))).mul(ndir)));
						(0..9).for_each(|i| norm.push(n[i % 3]));
					}
				}
				let d: Vec3 = max.sub(min);
				let scale = (1., 1., 1.).div(d.x().max(d.y()).max(d.z())).mul(scale);
				let center = max.sum(min).div(2);
				let xyz = xyz
					.chunks(3)
					.flat_map(|s| <[_; 3]>::to((Vec3::to(s)).sub(center).mul(scale)).to_vec())
					.collect::<Vec<f32>>();
				Model { idxs, xyz, uv, norm }
			})
			.collect();
		Ok(models)
	}
	pub fn new_cached(name: &str) -> Res<Self> {
		let cache = &CONCAT![name, ".obj.z"];
		if let Ok(d) = FS::Load::Archive(cache) {
			if let Ok(model) = SERDE::FromVec(&d) {
				return Ok(model);
			}
		}

		let model: Res<Model> = (|| {
			let m = Model::load_models(name, 1.)?.into_iter().next().ok_or("Empty models file")?;
			let v = EXPECT!(SERDE::ToVec(&m));
			FS::Save::Archive((cache, v));
			Ok(m)
		})();
		model
	}
}
impl<T: Borrow<Model>> From<T> for Mesh<u32, f32, f16, f16> {
	fn from(m: T) -> Self {
		let m = m.borrow();
		let (i, c, n) = (&m.idxs[..], &m.xyz[..], &m.norm[..]);
		if m.uv.is_empty() {
			Self::new((i, c, n, gl::TRIANGLES))
		} else {
			Self::new((i, c, &m.uv[..], n, gl::TRIANGLES))
		}
	}
}

impl Mesh<u16, f32, f16, f32> {
	pub fn make_sphere(scale: f32, segs: u32) -> Self {
		let (xyz, uv) = {
			let (mut xyz, mut uv) = (vec![], vec![]);
			for y in 0..1 + segs {
				for x in 0..1 + segs {
					let (sx, sy) = Vec2::to((x, y)).div(segs);
					let (rx, ry) = (sx.to_radians(), sy.to_radians());
					let (x, y, z) = ((rx * 360.).cos() * (ry * 180.).sin(), (ry * 180.).cos(), (rx * 360.).sin() * (ry * 180.).sin()).mul(scale);
					let (sx, sy) = vec2::<f16>::to(Vec2::to(glm::normalize(&glm::Vec2::to((sx, sy)))));
					xyz.extend(&[x, y, z]);
					uv.extend(&[sx, sy]);
				}
			}
			(xyz, uv)
		};

		let idx = (0..segs)
			.flat_map(|y| {
				let s = segs + 1;
				let row = (0..1 + segs).flat_map(|x| vec![y * s + x, (y + 1) * s + x]);
				if y % 2 == y {
					row.collect::<Vec<_>>()
				} else {
					row.rev().collect::<Vec<_>>()
				}
			})
			.map(|i| u16::to(i))
			.collect::<Vec<_>>();

		let draw = (u32::to(idx.len()), gl::TRIANGLE_STRIP);
		let idx = IdxArr::new(&idx);
		let xyz = AttrArr::new(&xyz);
		let uv = AttrArr::new(&uv);
		let norm = AttrArr::default();

		let mut vao = Vao::new();
		vao.BindIdxs(&idx);
		vao.AttribFmt(&xyz, (0, 3));
		vao.AttribFmt(&uv, (1, 2));
		vao.AttribFmt(&xyz, (2, 3));
		let buff = (idx, xyz, Some(uv), norm);

		Self { vao, buff, draw }
	}
}

impl Serialize for Model {
	fn serialize<SE: Serializer>(&self, serializer: SE) -> Result<SE::Ok, SE::Error> {
		serializer.serialize_bytes(&self.to_bytes())
	}
}
impl<'de> Deserialize<'de> for Model {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		struct V;
		impl<'de> Visitor<'de> for V {
			type Value = Model;

			fn expecting(&self, formatter: &mut Formatter) -> FmtRes {
				formatter.write_str("Model bytes")
			}
			fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
				Ok(Self::Value::from_bytes(v))
			}
		}

		deserializer.deserialize_bytes(V)
	}
}

impl Model {
	pub fn to_bytes(&self) -> Vec<u8> {
		let mut v = vec![];
		let Self { idxs, xyz, uv, norm } = self;
		let il: [u8; 8] = (idxs.len() * type_size!(u32)).to_le_bytes();
		let cl: [u8; 8] = (xyz.len() * type_size!(f32)).to_le_bytes();
		let tl: [u8; 8] = (uv.len() * type_size!(f16)).to_le_bytes();
		let (_, i, _) = unsafe { idxs.align_to() };
		let (_, c, _) = unsafe { xyz.align_to() };
		let (_, t, _) = unsafe { uv.align_to() };
		let (_, n, _) = unsafe { norm.align_to() };
		v.extend(il.iter().chain(cl.iter()).chain(tl.iter()).chain(i).chain(c).chain(t).chain(n));
		v
	}
	pub fn from_bytes(v: &[u8]) -> Self {
		let il = 24 + usize::from_le_bytes(v[0..8].try_into().unwrap());
		let cl = il + usize::from_le_bytes(v[8..16].try_into().unwrap());
		let tl = cl + usize::from_le_bytes(v[16..24].try_into().unwrap());
		let idxs = unsafe { v[24..il].align_to() }.1.to_vec();
		let xyz = unsafe { v[il..cl].align_to() }.1.to_vec();
		let uv = unsafe { v[cl..tl].align_to() }.1.to_vec();
		let norm = unsafe { v[tl..].align_to() }.1.to_vec();
		Self { idxs, xyz, uv, norm }
	}
}
