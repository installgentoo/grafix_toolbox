use super::*;
use crate::math::*;

#[derive_as_obj]
pub struct Model {
	#[cfg_attr(feature = "adv_fs", serde(with = "ser::as_byte_slice"))]
	idx: Box<[u32]>,
	#[cfg_attr(feature = "adv_fs", serde(with = "ser::as_byte_slice"))]
	xyz: Box<[f32]>,
	#[cfg_attr(feature = "adv_fs", serde(with = "ser::as_byte_slice"))]
	uv: Box<[f16]>,
	#[cfg_attr(feature = "adv_fs", serde(with = "ser::as_byte_slice"))]
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
				let (idx, xyz, uv, mut norm, (mut min, mut max)) = (
					m.indices.into(),
					m.positions,
					m.texcoords.into_iter().map(f16).collect(),
					m.normals.iter().map(|&v| f16(v)).collect_vec(),
					Def(),
				);
				for i in (0..xyz.len()).step_by(3) {
					let v = Vec3(&xyz[i..]);
					(min, max) = (v.fmin(min), v.fmax(max));
					if m.normals.is_empty() && i % 9 == 0 && i + 8 < xyz.len() {
						let xyz = &xyz[i..];
						let (v1, v2, v3) = Mat3((xyz, &xyz[3..], &xyz[6..]));
						let ndir = v1.sum(v2).sum(v3).div(3).sgn();
						let (v1, v2, v3) = vec3::<la::V3>::to((v1, v2, v3));
						let n = Vec3(la::normal(v1, v2, v3)).mul(ndir).pipe(<[_; 3]>::to);
						(0..9).for_each(|i| norm.push(n[i % 3]));
					}
				}
				let d = max.sub(min).max_comp();
				let (center, scale) = (max.sum(min).div(2), Vec3(1).div(d).mul(scale));
				let xyz = xyz.chunks(3).flat_map(|s| Vec3(s).sub(center).mul(scale).pipe(<[_; 3]>::to).to_vec()).collect();
				Self { idx, xyz, uv, norm: norm.into() }
			})
			.collect();
		Ok(models)
	}
	#[cfg(feature = "adv_fs")]
	pub fn new_cached(name: &str) -> Res<Self> {
		let cache = format!("{name}.obj.z");
		if let model @ Ok(_) = FS::Load::Archive(&cache).and_then(ser::from_vec) {
			return model;
		}

		Self::load_models(name, 1.)?
			.into_iter()
			.next()
			.ok_or("Empty models file")?
			.tap(|m| ser::to_vec(m).map(|v| FS::Save::Archive((cache, v, 10))).warn())
			.pipe(Ok)
	}
}
impl<T: Borrow<Model>> From<T> for AnyMesh {
	fn from(m: T) -> Self {
		let m = m.borrow();
		let (i, c, n) = (&m.idx, (3, &m.xyz[..]), (3, &m.norm[..]));
		let geom = if m.uv.is_empty() {
			Geometry::new(i, (c, (), n))
		} else {
			Geometry::new(i, (c, (2, &m.uv[..]), n))
		};

		Mesh { geom, draw: (u32(i.len()), gl::TRIANGLES) }.pipe(Box)
	}
}

impl Mesh<u16> {
	pub fn make_sphere(scale: f32, segs: u32) -> AnyMesh {
		let (xyz, uv) = {
			let (mut xyz, mut uv): (Vec<f32>, Vec<f16>) = Def();
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
				if y % 2 == y { row.collect_vec() } else { row.rev().collect_vec() }
			})
			.map(u16)
			.collect_vec();

		let geom = Geometry::new(&idx, ((3, &xyz[..]), (2, &uv[..]), (3, &xyz[..])));

		Self { geom, draw: (u16(idx.len()), gl::TRIANGLE_STRIP) }.pipe(Box)
	}
}
