use crate::glsl::*;
use crate::uses::*;
use crate::GL::{mesh::Screen, shader::*, tex::*, Fbo, *};

pub struct SdfGenerator {
	dst_t: Shader,
	dt_h: Shader,
	render: Shader,
	sampl: Rc<Sampler>,
}
impl SdfGenerator {
	pub fn new() -> Self {
		let dst_t = EXPECT!(Shader::new((mesh__2d_screen_vs, sdf__distance_transform_v_ps)));
		let dt_h = EXPECT!(Shader::new((mesh__2d_screen_vs, sdf__distance_transform_ps)));
		let render = EXPECT!(Shader::new((mesh__2d_screen_vs, mesh__2d_screen_ps)));
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
				let s = Uniforms!(dst_t, ("tex", &t), ("r", border), ("step", (0., 1. / f32(h))));

				let s = Uniform!(s, ("side", 1.));
				surf_out.bind();
				Screen::Draw();

				let _ = Uniform!(s, ("side", -1.));
				surf_in.bind();
				Screen::Draw();
			}
			let mut out = Fbo::<RGBA, f32>::new((w, h));
			{
				let to = surf_out.tex.Bind(sampl);
				let ti = surf_in.tex.Bind(sampl);
				let _ = Uniforms!(dt_h, ("tex_o", &to), ("tex_i", &ti), ("r", border), ("step", (1. / f32(w), 0.)));
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
	r"#version 330 core
	in vec2 glTexCoord;
	layout(location = 0)out vec4 glFragColor;
	uniform sampler2D tex;
	uniform int r;
	uniform vec2 step;
	uniform float side;

	void main()
	{
		for(int i=0; i<r; ++i)
		{
			vec2 o = step * float(i);
			float t = side * 0.5;
			if((side * texture(tex, glTexCoord + o).r > t) || (side * texture(tex, glTexCoord - o).r > t))
			{
				glFragColor = vec4(vec3(float(i) / r), 1.);
				return;
			}
		}

		glFragColor = vec4(1);
	}"
);

SHADER!(
	sdf__distance_transform_ps,
	r"#version 330 core
	in vec2 glTexCoord;
	layout(location = 0)out vec4 glFragColor;
	uniform sampler2D tex_i, tex_o;
	uniform int r;
	uniform vec2 step;

	void main()
	{
		float d_i = texture(tex_i, glTexCoord).r;
		float d_o = texture(tex_o, glTexCoord).r;

		for(int i=1; i<r; ++i)
		{
			float v = float(i) / r;
			vec2 o = step * float(i);
			d_o = min(d_o, min(length(vec2(v, texture(tex_o, glTexCoord + o).r)), length(vec2(v, texture(tex_o, glTexCoord - o).r))));
			d_i = min(d_i, min(length(vec2(v, texture(tex_i, glTexCoord + o).r)), length(vec2(v, texture(tex_i, glTexCoord - o).r))));
		}

		d_o = 0.5 - d_o * 0.5;
		d_i = 0.5 + d_i * 0.5;

		glFragColor = vec4(vec3(mix(d_o, d_i, float(d_i > 0.5))), 1.);
	}"
);
