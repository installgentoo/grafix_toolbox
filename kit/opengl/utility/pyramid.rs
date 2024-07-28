use crate::GL::{fbo::*, mesh::*, shader::*, tex::*};
use crate::{lib::*, math::*};

pub fn pyramid(img: impl Into<Tex2d<RGBA, f32>>) -> Vec<Tex2d<RGBA, f32>> {
	let img = img.into();
	let (w, h) = (img.param.w, img.param.h);
	let levels = i32(f64(w.min(h)).log2());

	let mut render = Shader::pure([vs_mesh__2d_screen, ps_mesh__2d_screen]);
	let mut sub = Shader::pure([vs_mesh__2d_screen, ps_pyramid___sub]);
	let linear = &Sampler::linear();

	let mut img1 = img;
	let mut imgs = vec![];
	for l in 1..levels - 1 {
		let img2 = {
			let out = Fbo::<RGBA, f32>::new((w, h).div(l + 1));
			let b = img1.Bind(linear);
			let _ = Uniforms!(render, ("tex", b));
			out.bind();
			Screen::Draw();
			out.tex
		};
		imgs.push({
			let out = Fbo::<RGBA, f32>::new((w, h).div(l));
			let b1 = img1.Bind(linear);
			let b2 = img2.Bind(linear);
			let _ = Uniforms!(sub, ("tex1", b1), ("tex2", b2));
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
	let mut add = Shader::pure([vs_mesh__2d_screen, ps_pyramid___add]);
	let linear = &Sampler::linear();

	let mut img1 = pyramid.pop().valid();
	for img2 in pyramid.into_iter().rev().skip(1) {
		let (w, h) = (img2.param.w, img2.param.h);
		img1 = {
			let out = Fbo::<RGBA, f32>::new((w, h));
			let b1 = img1.Bind(linear);
			let b2 = img2.Bind(linear);
			let _ = Uniforms!(add, ("tex1", b1), ("tex2", b2));
			out.bind();
			Screen::Draw();
			out.tex
		};
	}

	img1
}

SHADER!(
	ps_pyramid___sub,
	r"in vec2 glUV;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex1, tex2;

	void main() { glFragColor = texture(tex1, glUV) - texture(tex2, glUV); }"
);
SHADER!(
	ps_pyramid___add,
	r"in vec2 glUV;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex1, tex2;

	void main() { glFragColor = texture(tex1, glUV) + texture(tex2, glUV); }"
);
