use crate::GL::{fbo::*, mesh::*, shader::*, tex::*};
use crate::{lib::*, math::*};

pub fn pyramid(img: impl Into<Tex2d<RGBA, f32>>) -> Vec<Tex2d<RGBA, f32>> {
	let img = img.into();
	let s = img.whdl().xy();
	let levels = img.param().mips_max();

	let mut render = [vs_mesh__2d_screen, ps_mesh__2d_screen].pipe(Shader::pure);
	let mut sub = [vs_mesh__2d_screen, ps_pyramid___sub].pipe(Shader::pure);
	let linear = &Sampler::linear();

	let mut img1 = img;
	let mut imgs = vec![];
	for l in 1..levels {
		let img2 = {
			let out = s.div(l + 1).pipe(Fbo::new);
			let b = img1.Bind(linear);
			let _ = Uniforms!(render, ("iTex", b));
			out.bind();
			Screen::Draw();
			out.tex
		};
		imgs.push({
			let out = s.div(l).pipe(Fbo::new);
			let b1 = img1.Bind(linear);
			let b2 = img2.Bind(linear);
			let _ = Uniforms!(sub, ("iTex1", b1), ("iTex2", b2));
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
	let mut add = [vs_mesh__2d_screen, ps_pyramid___add].pipe(Shader::pure);
	let linear = &Sampler::linear();

	let mut img1 = pyramid.pop().valid();
	for img2 in pyramid.into_iter().rev().skip(1) {
		img1 = {
			let out = img2.whdl().xy().pipe(Fbo::new);
			let b1 = img1.Bind(linear);
			let b2 = img2.Bind(linear);
			let _ = Uniforms!(add, ("iTex1", b1), ("iTex2", b2));
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
	uniform sampler2D iTex1, iTex2;

	void main() { glFragColor = texture(iTex1, glUV) - texture(iTex2, glUV); }"
);
SHADER!(
	ps_pyramid___add,
	r"in vec2 glUV;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D iTex1, iTex2;

	void main() { glFragColor = texture(iTex1, glUV) + texture(iTex2, glUV); }"
);
