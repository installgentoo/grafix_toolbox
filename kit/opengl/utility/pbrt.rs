use crate::glsl::*;
use crate::uses::{math::*, *};
use GL::{mesh::*, *};

pub struct EnvTex {
	pub mip_levels: f32,
	pub specular: CubeTex<RGB, f16>,
	pub irradiance: CubeTex<RGB, f16>,
}
impl<T: Borrow<Environment>> From<T> for EnvTex {
	fn from(e: T) -> Self {
		let e = e.borrow();
		let specular = CubeTex::from(&e.specular);
		let irradiance = (&e.diffuse).into();
		let mip_levels = f32(specular.param.l);
		Self { mip_levels, specular, irradiance }
	}
}

derive_common_OBJ! {pub struct Environment {
	specular: Vec<[fImage<RGB>; 6]>,
	diffuse: [fImage<RGB>; 6],
}}
impl Environment {
	#[cfg(all(feature = "adv_fs", feature = "hdr"))]
	pub fn new_cached(name: &str) -> Res<Self> {
		let cache = &format!("{name}.hdr.z");
		if let Ok(d) = FS::Load::Archive(cache) {
			if let Ok(env) = SERDE::FromVec(&d) {
				return Ok(env);
			}
		}

		let env: Res<_> = (|| {
			let file = FS::Load::File(format!("res/{name}.hdr"))?;
			let equirect = Tex2d::from(Image::<RGB, f32>::load(file)?);
			let env = Self::new(equirect);
			let _ = SERDE::ToVec(&env).map(|v| FS::Save::Archive((cache, v)));
			Ok(env)
		})();
		env
	}
	#[cfg(feature = "adv_fs")]
	pub fn lut_cached() -> Tex2d<RG, f16> {
		let cache = "brdf_lut.pbrt.z";
		if let Ok(d) = FS::Load::Archive(cache) {
			if let Ok(lut) = SERDE::FromVec(&d) {
				return fImage::into(lut);
			}
		}

		let lut = Self::lut();
		let _ = SERDE::ToVec(&lut).map(|v| FS::Save::Archive((cache, v)));
		lut.into()
	}
	pub fn lut() -> fImage<RG> {
		let mut lut = Shader::pure((mesh__2d_screen_vs, env__gen_lut_ps));
		let mut surf = Fbo::<RGBA, f32>::new((512, 512));
		{
			Screen::Prepare();
			let _ = Uniforms!(lut, ("iSamples", 4096_u32));
			surf.bind();
			Screen::Draw();
		}
		surf.tex.into()
	}
	pub fn new<S, F>(equirect: Tex2d<S, F>) -> Self {
		Screen::Prepare();
		let VP_mats = {
			use glm::vec3;
			let s = |to, up| glm::look_at(&vec3(0., 0., 0.), &to, &up);
			let proj = glm::perspective(1., 90_f32.to_radians(), 0.1, 10.);
			[
				s(vec3(1., 0., 0.), vec3(0., -1., 0.)),
				s(vec3(-1., 0., 0.), vec3(0., -1., 0.)),
				s(vec3(0., 1., 0.), vec3(0., 0., 1.)),
				s(vec3(0., -1., 0.), vec3(0., 0., -1.)),
				s(vec3(0., 0., 1.), vec3(0., -1., 0.)),
				s(vec3(0., 0., -1.), vec3(0., -1., 0.)),
			]
			.map(|side| Camera::new(proj, side).VP())
		};

		let sampl = &Sampler::linear();
		let mut equirect_shd = Shader::pure((env__gen_vs, env__unwrap_equirect_ps));
		let mut irradiance_shd = Shader::pure((env__gen_vs, env__gen_irradiance_ps));
		let mut specular_shd = Shader::pure((env__gen_vs, env__gen_spec_ps));

		let color: [_; 6] = VP_mats
			.iter()
			.map(|cam| {
				let e = equirect.Bind(sampl);
				let _ = Uniforms!(equirect_shd, ("equirect_tex", &e), ("MVPMat", cam));
				let mut surf = Fbo::<RGBA, f32>::new((512, 512));
				surf.bind();
				Skybox::Draw();
				fImage::<RGB>::from(surf.tex)
			})
			.collect_arr();
		let cubemap = CubeTex::from(&color);

		let diffuse: [_; 6] = VP_mats
			.iter()
			.map(|cam| {
				let e = cubemap.Bind(sampl);
				let _ = Uniforms!(irradiance_shd, ("env_cubetex", &e), ("MVPMat", cam), ("iDelta", 0.025));
				let mut surf = Fbo::<RGBA, f32>::new((64, 64));
				surf.bind();
				Skybox::Draw();
				fImage::<RGB>::from(surf.tex)
			})
			.collect_arr();

		let mips = TexParam::mip_levels(cubemap.param.w);
		let specular = vec![color]
			.into_iter()
			.chain(
				(1..mips)
					.map(|l| {
						let r = f32(l) / f32(mips - 1);
						let wh = cubemap.param.dim_unchecked(u32(l)).xy();
						let mip: [_; 6] = VP_mats
							.iter()
							.map(|cam| {
								let e = cubemap.Bind(sampl);
								let _ = Uniforms!(specular_shd, ("env_cubetex", &e), ("MVPMat", cam), ("iSamples", 4096_u32), ("iRoughness", r));
								let mut surf = Fbo::<RGBA, f32>::new(wh);
								surf.bind();
								Skybox::Draw();
								fImage::<RGB>::from(surf.tex)
							})
							.collect_arr();
						mip
					})
					.collect_vec()
					.into_iter(),
			)
			.collect_vec();

		Self { diffuse, specular }
	}
}

SHADER!(
	env__gen_vs,
	r"layout(location = 0) in vec3 Position;
	uniform mat4 MVPMat;
	out vec3 glTexCoord;

	void main() {
		vec4 pos = vec4(Position, 1);
		gl_Position = MVPMat * pos;
		glTexCoord = Position;
	}"
);

SHADER!(
	env__unwrap_equirect_ps,
	r"in vec3 glTexCoord;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D equirect_tex;

	void main() {
		vec3 v = normalize(glTexCoord);
		vec2 uv = vec2(atan(v.z, v.x), asin(v.y)) * vec2(.1591, .3183) + vec2(.5);
		vec3 c = texture(equirect_tex, uv).rgb;
		glFragColor = vec4(c, 1);
	}"
);

SHADER!(
	env__gen_irradiance_ps,
	r"in vec3 glTexCoord;
	layout(location = 0) out vec4 glFragColor;
	uniform samplerCube env_cubetex;
	uniform float iDelta;

	const float PI = 3.14159265358979323846;

	void main() {
		vec3 normal = normalize(glTexCoord);
		vec3 right = cross(vec3(0, 1, 0), normal);
		vec3 up = cross(normal, right);

		vec3 irradiance = vec3(0);
		float n_samples = 0;
		for (float phi = 0; phi < PI * 2; phi += iDelta) {
			for (float theta = 0; theta < .5 * PI; theta += iDelta) {
				vec3 tangent_sample = vec3(sin(theta) * cos(phi), sin(theta) * sin(phi), cos(theta));
				vec3 sample_vec = tangent_sample.x * right + tangent_sample.y * up + tangent_sample.z * normal;
				irradiance += texture(env_cubetex, sample_vec).rgb * cos(theta) * sin(theta);
				++n_samples;
			}
		}

		irradiance = PI * irradiance / n_samples;
		glFragColor = vec4(irradiance, 1);
	}"
);

const TRANSFORM: STR = r"
	const float PI_2 = 3.14159265358979323846 * 2;

	float RadicalInverse_VdC(uint bits) {
		bits = (bits << 16u) | (bits >> 16u);
		bits = ((bits & 0x55555555u) << 1u) | ((bits & 0xAAAAAAAAu) >> 1u);
		bits = ((bits & 0x33333333u) << 2u) | ((bits & 0xCCCCCCCCu) >> 2u);
		bits = ((bits & 0x0F0F0F0Fu) << 4u) | ((bits & 0xF0F0F0F0u) >> 4u);
		bits = ((bits & 0x00FF00FFu) << 8u) | ((bits & 0xFF00FF00u) >> 8u);
		return float(bits) * 2.3283064365386963e-10;   // / 0x100000000
	}

	vec2 Hammersley(uint i, uint N) { return vec2(float(i) / float(N), RadicalInverse_VdC(i)); }

	vec3 ImportanceSampleGGX(vec2 Xi, vec3 N, float roughness) {
		float a = roughness * roughness;

		float phi = Xi.x * PI_2;
		float cosTheta = sqrt((1. - Xi.y) / ((a * a - 1) * Xi.y + 1));
		float sinTheta = sqrt(1. - cosTheta * cosTheta);

		vec3 H = vec3(cos(phi) * sinTheta, sin(phi) * sinTheta, cosTheta);

		vec3 up = abs(N.z) < .999 ? vec3(0, 0, 1) : vec3(1, 0, 0);
		vec3 tangent = normalize(cross(up, N));
		vec3 bitangent = cross(N, tangent);

		vec3 sampleVec = tangent * H.x + bitangent * H.y + N * H.z;
		return normalize(sampleVec);
	}";

SHADER!(
	env__gen_spec_ps,
	r"in vec3 glTexCoord;
	layout(location = 0) out vec4 glFragColor;
	uniform samplerCube env_cubetex;
	uniform float iRoughness;
	uniform uint iSamples;
	",
	TRANSFORM,
	r"
	void main()
		{
		vec3 N = normalize(glTexCoord);

		float totalWeight = 0;
		vec3 prefilteredColor = vec3(0);
		for (uint i = 0u; i < iSamples; ++i) {
			vec2 Xi = Hammersley(i, iSamples);
			vec3 H = ImportanceSampleGGX(Xi, N, iRoughness);
			vec3 L = normalize(dot(N, H) * H * 2 - N);

			float NdotL = max(dot(N, L), 0);
			if (NdotL > 0) {
				prefilteredColor += texture(env_cubetex, L).rgb * NdotL;
				totalWeight += NdotL;
			}
		}
		prefilteredColor /= totalWeight;

		glFragColor = vec4(prefilteredColor, 1);
	}"
);

SHADER!(
	env__gen_lut_ps,
	r"in vec2 glTexCoord;
	layout(location = 0) out vec4 glFragColor;
	uniform uint iSamples;
	",
	TRANSFORM,
	r"
	float GeometrySchlickGGX(float NdotV, float roughness)
		{
		float k = (roughness * roughness) / 2;
		float denom = NdotV * (1. - k) + k;
		return NdotV / denom;
	}

	float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {
		float NdotV = max(dot(N, V), 0);
		float NdotL = max(dot(N, L), 0);
		float ggx2 = GeometrySchlickGGX(NdotV, roughness);
		float ggx1 = GeometrySchlickGGX(NdotL, roughness);

		return ggx1 * ggx2;
	}

	vec2 IntegrateBRDF(float NdotV, float roughness) {
		vec3 V = vec3(sqrt(1. - NdotV * NdotV), 0, NdotV);

		float A = 0;
		float B = 0;
		vec3 N = vec3(0, 0, 1);
		for (uint i = 0u; i < iSamples; ++i) {
			vec2 Xi = Hammersley(i, iSamples);
			vec3 H = ImportanceSampleGGX(Xi, N, roughness);
			vec3 L = normalize(dot(V, H) * H * 2 - V);

			float NdotL = max(L.z, 0);
			if (NdotL > 0) {
				float NdotH = max(H.z, 0);
				float VdotH = max(dot(V, H), 0);

				float G = GeometrySmith(N, V, L, roughness);
				float G_Vis = (G * VdotH) / (NdotH * NdotV);
				float Fc = pow(1. - VdotH, 5);

				A += (1. - Fc) * G_Vis;
				B += Fc * G_Vis;
			}
		}
		A /= float(iSamples);
		B /= float(iSamples);
		return vec2(A, B);
	}

	void main() { glFragColor = vec4(IntegrateBRDF(glTexCoord.x, glTexCoord.y), 0, 1); }"
);
