use super::*;

#[derive_as_trivial]
pub struct Surf {
	pub pos: Vec2,
	pub size: Vec2,
}
impl Surf {
	pub fn new(pos: Vec2, size: Vec2) -> Self {
		let s = Self { pos, size };
		debug_assert!({
			s.b_box();
			true
		});
		s
	}
	pub fn clamp_to_screen(&mut self, r: &RenderLock) {
		let a = r.aspect();
		let (lb, ru) = (Vec2(-1).div(a), Vec2(1).div(a));
		let Self { pos, size } = self;
		*size = size.clmp(0, ru.mul(2));
		*pos = pos.clmp(lb, ru.sub(size));
	}
	pub fn b_box(self) -> Geom {
		let Self { pos, size } = self;
		ASSERT!(size.min_comp() >= 0., "Surface has negative size {size:?}");
		(pos, pos.sum(size))
	}
	pub fn scale(mut self, mult: Vec2) -> Self {
		self.size = self.size.mul(mult);
		self.clmp_size()
	}
	pub fn pos(self, pos: Vec2) -> Self {
		Self { pos, ..self }
	}
	pub fn xy(self, offset: Vec2) -> Self {
		Self { pos: self.pos.sum(offset), ..self }
	}
	pub fn x(self, offset: f32) -> Self {
		self.xy((offset, 0.))
	}
	pub fn y(self, offset: f32) -> Self {
		self.xy((0., offset))
	}
	pub fn xr(self, offset: f32) -> Self {
		self.x(self.size.x() - offset)
	}
	pub fn yt(self, offset: f32) -> Self {
		self.y(self.size.y() - offset)
	}
	pub fn x_self<A>(self, mult: A) -> Self
	where
		f32: Cast<A>,
	{
		self.x(self.size.x() * f32(mult))
	}
	pub fn y_self<A>(self, mult: A) -> Self
	where
		f32: Cast<A>,
	{
		self.y(self.size.y() * f32(mult))
	}
	pub fn size(self, size: Vec2) -> Self {
		Self { size, ..self }.clmp_size()
	}
	pub fn w(self, new: f32) -> Self {
		self.size((new, self.size.y()))
	}
	pub fn h(self, new: f32) -> Self {
		self.size((self.size.x(), new))
	}
	pub fn w_scale<A>(self, mult: A) -> Self
	where
		f32: Cast<A>,
	{
		self.scale((f32(mult), 1.))
	}
	pub fn h_scale<A>(self, mult: A) -> Self
	where
		f32: Cast<A>,
	{
		self.scale((1., f32(mult)))
	}
	pub fn size_sub(self, shrink: Vec2) -> Self {
		Self { size: self.size.sub(shrink), ..self }.clmp_size()
	}
	pub fn w_sub(self, shrink: f32) -> Self {
		self.w(self.size.x() - shrink).clmp_size()
	}
	pub fn h_sub(self, shrink: f32) -> Self {
		self.h(self.size.y() - shrink).clmp_size()
	}
	fn clmp_size(mut self) -> Self {
		self.size = self.size.fmax(0);
		self
	}
}
impl From<Geom> for Surf {
	fn from((pos, size): Geom) -> Self {
		Self { pos, size }
	}
}
