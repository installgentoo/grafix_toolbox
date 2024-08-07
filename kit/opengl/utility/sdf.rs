use crate::GL::{gl, mesh::*, *};
use crate::{lib::*, math::*};

pub struct SdfGenerator {
	dst_t: Shader,
	dt_h: Shader,
	sampl: Sampler,
}
impl SdfGenerator {
	pub fn new(sampl: &[(GLenum, GLenum)]) -> Self {
		let dst_t = Shader::pure([vs_mesh__2d_screen, ps_sdf__distance_transform_v]);
		let dt_h = Shader::pure([vs_mesh__2d_screen, ps_sdf__distance_transform]);
		let sampl = sampl.iter().chain(&[(gl::TEXTURE_MIN_FILTER, gl::NEAREST)]).copied().collect_vec().into();
		Self { dst_t, dt_h, sampl }
	}
	pub fn generate<S: TexSize, F: TexFmt, FROM: TexSize>(&mut self, tex: &Tex2d<S, F>, scale: i32, thickness: i32) -> Tex2d<RED, f16> {
		ASSERT!(FROM::SIZE <= S::SIZE, "Wrong sdf source channel");
		let thickness = thickness * scale;
		let TexParam { w, h, .. } = tex.param;
		GLSave!(BLEND, DEPTH_TEST);
		GLDisable!(BLEND, DEPTH_TEST);
		let Self { dst_t, dt_h, ref sampl, .. } = self;
		let surf_out = Fbo::<RED, f16>::new((w, h));
		let surf_in = Fbo::<RED, f16>::new((w, h));
		{
			let t = tex.Bind(sampl);
			let s = Uniforms!(dst_t, ("tex", t), ("iChannel", FROM::SIZE), ("iThickness", thickness), ("iStep", Vec2((0, 1)).div((w, h))));

			let s = Uniform!(s, ("iSide", 1.));
			surf_out.bind();
			Screen::Draw();

			let _ = Uniform!(s, ("iSide", -1.));
			surf_in.bind();
			Screen::Draw();
		}
		let out = Fbo::<RED, f16>::new((w, h));
		{
			let to = surf_out.tex.Bind(sampl);
			let ti = surf_in.tex.Bind(sampl);
			let _ = Uniforms!(dt_h, ("tex_o", to), ("tex_i", ti), ("iThickness", thickness), ("iStep", Vec2((1, 0)).div((w, h))));
			out.bind();
			Screen::Draw();
		}
		GLRestore!(BLEND, DEPTH_TEST);

		out.tex
	}
}
impl Default for SdfGenerator {
	fn default() -> Self {
		Self::new(&[(gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE), (gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE)])
	}
}

SHADER!(
	ps_sdf__distance_transform_v,
	r"in vec2 glUV;
	layout(location = 0) out float glFragColor;
	uniform sampler2D tex;
	uniform int iChannel;
	uniform int iThickness;
	uniform vec2 iStep;
	uniform float iSide;

	float get_tex(in vec2 p) {
		if (1 == iChannel) return texture(tex, glUV + p).r;
		if (2 == iChannel) return texture(tex, glUV + p).g;
		if (3 == iChannel) return texture(tex, glUV + p).b;
		return texture(tex, glUV + p).a;
	}

	void main() {
		const float t = iSide * .5;
		for (int _i = 0; _i < iThickness; ++_i) {
			float i = float(_i);
			vec2 o = iStep * i;
			if (iSide * get_tex(o) > t || iSide * get_tex(-o) > t) {
				glFragColor = i / iThickness;
				return;
			}
		}

		glFragColor = 1;
	}"
);

SHADER!(
	ps_sdf__distance_transform,
	r"in vec2 glUV;
	layout(location = 0) out float glFragColor;
	uniform sampler2D tex_i, tex_o;
	uniform int iThickness;
	uniform vec2 iStep;

	void main() {
		float d_i = texture(tex_i, glUV).r;
		float d_o = texture(tex_o, glUV).r;

		for (int _i = 1; _i < iThickness; ++_i) {
			float i = float(_i);
			vec2 o = i * iStep;
			vec2 o_p = glUV + o;
			vec2 o_m = glUV - o;
			i /= iThickness;
			d_o = min(d_o, min(length(vec2(i, texture(tex_o, o_p).r)), length(vec2(i, texture(tex_o, o_m).r))));
			d_i = min(d_i, min(length(vec2(i, texture(tex_i, o_p).r)), length(vec2(i, texture(tex_i, o_m).r))));
		}

		d_o = .5 - d_o * .5;
		d_i = .5 + d_i * .5;

		glFragColor = mix(d_o, d_i, float(d_i > .5));
	}"
);
