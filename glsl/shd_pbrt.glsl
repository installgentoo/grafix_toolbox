//--PIX vs_skybox


layout(location = 0) in vec3 Position;
uniform mat4 MVPMat;
uniform mat4 ModelViewMat;
out vec3 glTexUV;

void main() {
	vec4 pos = vec4(Position, 1);
	gl_Position = MVPMat * pos;
	glTexUV = Position;
}


//--PIX ps_skybox


in vec3 glTexUV;
layout(location = 0) out vec4 glFragColor;
uniform samplerCube skybox_tex;
uniform float iExposure;

const float gamma = 2.2;

void main() {
	vec3 c = textureLod(skybox_tex, glTexUV, 0).rgb;
	c = vec3(1) - exp(-c * iExposure);
	c = pow(c, vec3(1. / gamma));
	glFragColor = vec4(c, 1);
}


//--VER vs_material_based_render


layout(location = 0) in vec3 Position;
layout(location = 1) in vec2 TexCoord;
layout(location = 2) in vec3 Normal;
uniform mat4 MVPMat;
uniform mat4 ModelViewMat;
uniform mat3 NormalViewMat;
uniform mat3 NormalMat;
out vec3 glPos;
out vec2 glTexUV;
out vec3 glNormal;
out vec3 glNormalWorld;

void main() {
	vec4 pos = vec4(Position, 1);
	gl_Position = MVPMat * pos;
	glPos = (ModelViewMat * pos).xyz;
	glTexUV = TexCoord;
	glNormal = NormalViewMat * Normal;
	glNormalWorld = NormalMat * Normal;
}


//--PIX ps_material_based_render


in vec3 glPos;
in vec2 glTexUV;
in vec3 glNormal;
in vec3 glNormalWorld;
layout(location = 0) out vec4 glFragColor;

struct Light {
	vec3 Pos;
	vec4 Color;
};
layout(std140) uniform iLights {
	int iLightsNum;
	Light iLight[ 20 ];
};

uniform samplerCube irradiance_cubetex;
uniform samplerCube specular_cubetex;
uniform sampler2D brdf_lut_tex;
uniform vec3 iCameraWorld;

uniform vec3 iAlbedo;
uniform float iMetallicity;
uniform float iRoughness;
uniform float iExposure;
uniform float iMaxLod;

const float gamma = 2.2;
const float ao = .1;
const float refractive_index = .1;

const float PI = 3.1415927;

vec3 fresnelSchlick(float cos_theta, vec3 F0) { return F0 + (max(vec3(1. - iRoughness), F0) - F0) * pow(1. - cos_theta, 5); }

float DistributionGGX(vec3 N, vec3 H) {
	float a = iRoughness * iRoughness;
	float a2 = a * a;
	float NdotH = max(dot(N, H), 0);
	float NdotH2 = NdotH * NdotH;

	float num = a2;
	float denom = (NdotH2 * (a2 - 1) + 1);
	denom = PI * denom * denom;

	return num / denom;
}

float GeometrySchlickGGX(float NdotV) {
	float r = (iRoughness + 1);
	float k = (r * r) / 8;

	float num = NdotV;
	float denom = NdotV * (1. - k) + k;

	return num / denom;
}

float GeometrySmith(vec3 N, vec3 V, vec3 L) {
	float NdotV = max(dot(N, V), 0);
	float NdotL = max(dot(N, L), 0);
	float ggx2 = GeometrySchlickGGX(NdotV);
	float ggx1 = GeometrySchlickGGX(NdotL);

	return ggx1 * ggx2;
}

void main() {
	vec3 normal = normalize(glNormal);
	vec3 eye_vec = normalize(-glPos);

	vec3 F0 = mix(vec3(.04), iAlbedo, iMetallicity);

	vec3 Lo = vec3(0);
	for (int i = 0; i < iLightsNum; ++i) {
		vec3 light_vec = normalize(iLight[ i ].Pos - glPos);
		vec3 half_vec = normalize(eye_vec + light_vec);

		float dist = length(iLight[ i ].Pos - glPos);
		vec3 radiance = iLight[ i ].Color.xyz * iLight[ i ].Color.a / (dist * dist);

		float NDF = DistributionGGX(normal, half_vec);
		float G = GeometrySmith(normal, eye_vec, light_vec);
		vec3 F = fresnelSchlick(max(dot(half_vec, eye_vec), 0), F0);

		vec3 kS = F;
		vec3 kD = vec3(1) - kS;
		kD *= 1. - iMetallicity;

		vec3 numerator = NDF * G * F;
		float denominator = max(dot(normal, eye_vec), 0) * max(dot(normal, light_vec), 0) * 4;
		vec3 specular = numerator / max(denominator, .01);

		float NdotL = max(dot(normal, light_vec), 0);
		const float invPI = 1. / PI;
		Lo += (kD * iAlbedo * invPI + specular) * radiance * NdotL;
	}

	vec3 kS = fresnelSchlick(max(dot(normal, eye_vec), 0), F0);
	vec3 kD = 1. - kS;
	kD *= 1. - iMetallicity;

	vec3 normal_world = normalize(glNormalWorld);

	vec3 irradiance = texture(irradiance_cubetex, normal_world).rgb;
	vec3 diffuse = irradiance * iAlbedo;

	vec3 R = reflect(-iCameraWorld, normal_world);
	vec3 prefiltered = textureLod(specular_cubetex, R, iRoughness * iMaxLod).rgb;

	vec2 brdf = texture(brdf_lut_tex, vec2(max(dot(normal, eye_vec), 0), iRoughness)).rg;
	vec3 specular = prefiltered * (kS * brdf.x + brdf.y);

	vec3 ambient = (kD * diffuse + specular) * ao;
	vec3 c = ambient + Lo;

	c = vec3(1) - exp(-c * iExposure);
	c = pow(c, vec3(1. / gamma));

	glFragColor = vec4(c, 1);
}
