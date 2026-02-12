use crate::GL::{gl, *};
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
		let sampl = sampl
			.iter()
			.chain(&[(gl::TEXTURE_MAG_FILTER, gl::NEAREST), (gl::TEXTURE_MIN_FILTER, gl::NEAREST)])
			.copied()
			.collect_vec()
			.into();
		Self { dst_t, dt_h, sampl }
	}
	pub fn generate<S: TexSize, F: TexFmt, FROM: TexSize>(&mut self, tex: &Tex2d<S, F>, border: i32) -> Tex2d<RED, f16> {
		ASSERT!(FROM::SIZE <= S::SIZE, "Wrong sdf source channel");
		let s = tex.whdl().xy();
		GLSave!(BLEND, DEPTH_TEST);
		GLDisable!(BLEND, DEPTH_TEST);
		let Self { dst_t, dt_h, sampl } = self;
		let surf_out = Fbo::<RED, f16>::new(s);
		let surf_in = Fbo::<RED, f16>::new(s);
		{
			let t = tex.Bind(sampl);
			let s = Uniforms!(dst_t, ("iTex", t), ("iChannel", i32(FROM::SIZE)), ("iBorder", border), ("iStep", Vec2((0, 1)).div(s)));

			let s = Uniform!(s, ("iSide", 1.));
			surf_out.bind();
			Screen::Draw();

			let _ = Uniform!(s, ("iSide", -1.));
			surf_in.bind();
			Screen::Draw();
		}
		let out = Fbo::<RED, f16>::new(s);
		{
			let to = surf_out.tex.Bind(sampl);
			let ti = surf_in.tex.Bind(sampl);
			let _ = Uniforms!(dt_h, ("iOut", to), ("iIn", ti), ("iBorder", border), ("iStep", Vec2((1, 0)).div(s)));
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
	uniform sampler2D iTex;
	uniform int iChannel, iBorder;
	uniform vec2 iStep;
	uniform float iSide;

	float get_tex(in vec2 p) {
		if (1 == iChannel) return texture(iTex, glUV + p).r;
		if (2 == iChannel) return texture(iTex, glUV + p).g;
		if (3 == iChannel) return texture(iTex, glUV + p).b;
		return texture(iTex, glUV + p).a;
	}

	void main() {
		const float t = iSide * .5;
		for (int _i = 0; _i < iBorder; ++_i) {
			float i = float(_i);
			vec2 o = iStep * i;
			if (iSide * get_tex(o) > t || iSide * get_tex(-o) > t) {
				glFragColor = i / iBorder;
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
	uniform sampler2D iIn, iOut;
	uniform int iBorder;
	uniform vec2 iStep;

	void main() {
		float d_i = texture(iIn, glUV).r;
		float d_o = texture(iOut, glUV).r;

		for (int _i = 1; _i < iBorder; ++_i) {
			float i = float(_i);
			vec2 o = i * iStep;
			vec2 o_p = glUV + o;
			vec2 o_m = glUV - o;
			i /= iBorder;
			d_o = min(d_o, min(length(vec2(i, texture(iOut, o_p).r)), length(vec2(i, texture(iOut, o_m).r))));
			d_i = min(d_i, min(length(vec2(i, texture(iIn, o_p).r)), length(vec2(i, texture(iIn, o_m).r))));
		}

		d_o = .5 - d_o * .5;
		d_i = .5 + d_i * .5;

		glFragColor = mix(d_o, d_i, float(d_i > .5));
	}"
);
