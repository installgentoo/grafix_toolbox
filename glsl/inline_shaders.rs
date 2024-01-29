SHADER!(
	vs_mesh__2d_screen,
	r"layout(location = 0) in vec4 Position;
	out vec2 glTexCoord;

	void main() {
		gl_Position = vec4(Position.xy, 0, 1);
		glTexCoord = Position.zw;
	}"
);

SHADER!(
	ps_mesh__2d_screen,
	r"in vec2 glTexCoord;
	layout(location = 0) out vec4 glFragColor;
	uniform sampler2D tex;

	void main() { glFragColor = texture(tex, glTexCoord); }"
);
