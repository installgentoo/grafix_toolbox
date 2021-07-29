use crate::{glsl::*, uses::math::*, uses::*};
use GL::{fbo::*, mesh::*, shader::*, tex::*};

pub fn pyramid(img: impl Into<Tex2d<RGBA, f32>>) -> Vec<Tex2d<RGBA, f32>> {
	let img = img.into();
	let (w, h) = (img.param.w, img.param.h);
	let levels = i32(f64(w.min(h)).log2());

	let mut render = EXPECT!(Shader::new((mesh__2d_screen_vs, mesh__2d_screen_ps)));
	let mut sub = EXPECT!(Shader::new((mesh__2d_screen_vs, shd::sub)));
	let linear = &Sampler::linear();

	let mut img1 = img;
	let mut imgs = vec![];
	for l in 1..levels - 1 {
		let img2 = {
			let mut out = Fbo::<RGBA, f32>::new((w, h).div(l + 1));
			let b = img1.Bind(linear);
			let _ = Uniforms!(render, ("tex", &b));
			out.bind();
			Screen::Draw();
			out.tex
		};
		imgs.push({
			let mut out = Fbo::<RGBA, f32>::new((w, h).div(l));
			let b1 = img1.Bind(linear);
			let b2 = img2.Bind(linear);
			let _ = Uniforms!(sub, ("tex1", &b1), ("tex2", &b2));
			out.bind();
			Screen::Draw();
			out.tex
		});
		img1 = img2
	}
	imgs.push(img1);

	imgs
}

pub fn collapse(mut pyramid: Vec<Tex2d<RGBA, f32>>) -> Tex2d<RGBA, f32> {
	let mut add = EXPECT!(Shader::new((mesh__2d_screen_vs, shd::add)));
	let linear = &Sampler::linear();

	let mut img1 = pyramid.pop().unwrap();
	for img2 in pyramid.into_iter().rev().skip(1) {
		let (w, h) = (img2.param.w, img2.param.h);
		img1 = {
			let mut out = Fbo::<RGBA, f32>::new((w, h));
			let b1 = img1.Bind(linear);
			let b2 = img2.Bind(linear);
			let _ = Uniforms!(add, ("tex1", &b1), ("tex2", &b2));
			out.bind();
			Screen::Draw();
			out.tex
		};
	}

	img1
}

mod shd {
	SHADER!(
		sub,
		r"#version 330 core
		in vec2 glTexCoord;
		layout(location = 0)out vec4 glFragColor;
		uniform sampler2D tex1, tex2;

		void main()
		{
			glFragColor = texture(tex1, glTexCoord) - texture(tex2, glTexCoord);
		}"
	);
	SHADER!(
		add,
		r"#version 330 core
		in vec2 glTexCoord;
		layout(location = 0)out vec4 glFragColor;
		uniform sampler2D tex1, tex2;

		void main()
		{
			glFragColor = texture(tex1, glTexCoord) + texture(tex2, glTexCoord);
		}"
	);
}
