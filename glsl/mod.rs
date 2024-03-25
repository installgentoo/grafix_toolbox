SHADER!(
	vs_mesh__2d_screen,
	r"layout(location = 0) in vec4 Position;
	out vec2 glTexUV;

	void main() {
		gl_Position = vec4(Position.xy, 0, 1);
		glTexUV = Position.zw;
	}"
);

SHADER!(
	ps_mesh__2d_screen,
	r"in vec2 glTexUV;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex;

	void main() { glFragColor = texture(tex, glTexUV); }"
);
