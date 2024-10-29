use super::*;

pub struct Frame9<'r> {
	pub pos: Vec2,
	pub size: Vec2,
	pub corner: f32,
	pub color: Color,
	pub theme: &'r VTex2d<RGBA, u8>,
}
impl Frame9<'_> {
	pub fn compare(&self, crop: &Geom, r: &Frame9Impl) -> State {
		let &Self { pos, size, corner, color, theme } = self;
		let xyzw = (State::XYZW | State::UV).or_def(geom_cmp(pos, size, crop, &r.base) || corner != r.corner);
		let rgba = State::RGBA.or_def(color != r.base.color);
		let ord = State::MISMATCH.or_def(!ptr::eq(theme, r.tex));
		ord | xyzw | rgba
	}
	pub fn obj(self, crop: Geom) -> Frame9Impl {
		let Self { pos, size, corner, color, theme } = self;
		Frame9Impl { base: Base { pos, size, crop, color }, corner, tex: theme }
	}
}
pub struct Frame9Impl {
	base: Base,
	corner: f32,
	tex: *const VTex2d<RGBA, u8>,
}
impl Frame9Impl {
	pub fn batchable(&self, r: &Self) -> bool {
		self.tex == r.tex
	}
}
impl Primitive for Frame9Impl {
	fn base(&self) -> &Base {
		&self.base
	}
	fn write_mesh(&self, aspect: Vec2, range: BatchedObj) {
		let (crop, &Base { pos, size, color, .. }) = (self.base.bound_box(), self.base());
		let c = size.x().min(size.y()) * self.corner.min(0.5).max(0.);
		write_sprite9((aspect, pos, size, (c, c), crop, (0., 0., 1., 1.), color), range);
	}
	fn batch_draw(&self, b: &VaoBinding<u16>, (offset, num): (u16, u16)) {
		let s = LeakyStatic!(Shader, { Shader::pure([vs_gui__pos_col_tex, ps_gui__frame]) });

		let tex = unsafe { &*self.tex };
		let t = tex.atlas.Bind(sampler());
		let (x, y, w, _) = tex.region;
		let _ = Uniforms!(s, ("tex", t), ("iThemeCoords", (x, y, w - x)));
		b.Draw((num, offset, gl::TRIANGLES));
	}

	fn vert_count(&self) -> u32 {
		16
	}
	fn gen_idxs(&self, (start, size): (u16, u16)) -> Box<[u16]> {
		sprite9_idxs((start, size))
	}
}

SHADER!(
	ps_gui__frame,
	r"in vec4 glColor;
	in vec2 glUV;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex;
	uniform vec3 iThemeCoords;

	void main() {
		float d = min(.9, length(glUV));
		vec4 c = texture(tex, iThemeCoords.xy + vec2(d * iThemeCoords.z, 0));
		glFragColor = glColor * c;
	}"
);
