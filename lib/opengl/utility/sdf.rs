#![allow(dead_code)]
use crate::glsl::*;
use crate::uses::*;
use GL::{mesh::Screen, shader::*, tex::*, Fbo, *};

pub struct SdfGenerator {
	dst_t: Shader,
	dt_h: Shader,
	render: Shader,
	sampl: Rc<Sampler>,
}
impl SdfGenerator {
	pub fn new() -> Self {
		let dst_t = Shader::pure((mesh__2d_screen_vs, sdf__distance_transform_v_ps));
		let dt_h = Shader::pure((mesh__2d_screen_vs, sdf__distance_transform_ps));
		let render = Shader::pure((mesh__2d_screen_vs, mesh__2d_screen_ps));
		let sampl = Sampler::linear();
		Self { dst_t, dt_h, render, sampl }
	}
	pub fn generate<S: TexSize, F: TexFmt>(&mut self, tex: Tex2d<S, F>, scale: i32, border: i32) -> Tex2d<S, F> {
		let border = border * scale;
		let TexParam { w, h, .. } = tex.param;
		GLSave!(BLEND, MULTISAMPLE, DEPTH_WRITEMASK);
		GLDisable!(BLEND, MULTISAMPLE, DEPTH_WRITEMASK);
		let Self { dst_t, dt_h, render, sampl } = self;
		let tex = {
			let mut surf_out = Fbo::<RGBA, f32>::new((w, h));
			let mut surf_in = Fbo::<RGBA, f32>::new((w, h));
			{
				let t = tex.Bind(sampl);
				let s = Uniforms!(dst_t, ("tex", &t), ("iBorder", border), ("iStep", (0., 1. / f32(h))));

				let s = Uniform!(s, ("iSide", 1.));
				surf_out.bind();
				Screen::Draw();

				let _ = Uniform!(s, ("iSide", -1.));
				surf_in.bind();
				Screen::Draw();
			}
			let mut out = Fbo::<RGBA, f32>::new((w, h));
			{
				let to = surf_out.tex.Bind(sampl);
				let ti = surf_in.tex.Bind(sampl);
				let _ = Uniforms!(dt_h, ("tex_o", &to), ("tex_i", &ti), ("iBorder", border), ("iStep", (1. / f32(w), 0.)));
				out.bind();
				Screen::Draw();
			}
			out.tex
		};
		let mut out = Fbo::<S, F>::new((w / scale, h / scale));
		let t = tex.Bind(sampl);
		let _ = Uniforms!(render, ("tex", &t));
		out.bind();
		Screen::Draw();

		GLRestore!(BLEND, MULTISAMPLE, DEPTH_WRITEMASK);

		out.tex
	}
}

SHADER!(
	sdf__distance_transform_v_ps,
	r"in vec2 glTexCoord;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex;
	uniform int iBorder;
	uniform vec2 iStep;
	uniform float iSide;

	void main() {
		for (int i = 0; i < iBorder; ++i) {
			vec2 o = iStep * float(i);
			float t = iSide * .5;
			if (iSide * texture(tex, glTexCoord + o).r > t || iSide * texture(tex, glTexCoord - o).r > t) {
				glFragColor = vec4(vec3(float(i) / iBorder), 1);
				return;
			}
		}

		glFragColor = vec4(1);
	}"
);

SHADER!(
	sdf__distance_transform_ps,
	r"in vec2 glTexCoord;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex_i, tex_o;
	uniform int iBorder;
	uniform vec2 iStep;

	void main() {
		float d_i = texture(tex_i, glTexCoord).r;
		float d_o = texture(tex_o, glTexCoord).r;

		for (int i = 1; i < iBorder; ++i) {
			float v = float(i) / iBorder;
			vec2 o = iStep * float(i);
			d_o = min(d_o, min(length(vec2(v, texture(tex_o, glTexCoord + o).r)), length(vec2(v, texture(tex_o, glTexCoord - o).r))));
			d_i = min(d_i, min(length(vec2(v, texture(tex_i, glTexCoord + o).r)), length(vec2(v, texture(tex_i, glTexCoord - o).r))));
		}

		d_o = .5 - d_o * .5;
		d_i = .5 + d_i * .5;

		glFragColor = vec4(vec3(mix(d_o, d_i, float(d_i > .5))), 1);
	}"
);
