use crate::GL::shader_use;

SHADER!(
	mesh__2d_screen_vs,
	r"#version 330 core
layout(location = 0)in vec4 Position;
out vec2 glTexCoord;

void main()
{
gl_Position = vec4(Position.xy, 0., 1.);
glTexCoord = Position.zw;
}"
);

SHADER!(
	mesh__2d_screen_ps,
	r"#version 330 core
in vec2 glTexCoord;
layout(location = 0)out vec4 glFragColor;
uniform sampler2D tex;

void main()
{
glFragColor = texture(tex, glTexCoord);
}"
);

SHADER!(
	mesh__2d_test,
	r"#version 330 core
in vec2 glTexCoord;
layout(location = 0)out vec4 glFragColor;
uniform sampler2D tex;
uniform mat3x4 color;

void main()
{
glFragColor = vec4(textureLod(tex, glTexCoord, 0.3).rgb, 1.) * color[1];
}"
);
