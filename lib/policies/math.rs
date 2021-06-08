pub type Mat2 = (Vec2, Vec2);
pub type Mat3 = (Vec3, Vec3, Vec3);
pub type Mat4 = (Vec4, Vec4, Vec4, Vec4);
pub type Mat2x3 = (Vec3, Vec3);
pub type Mat3x2 = (Vec2, Vec2, Vec2);
pub type Mat2x4 = (Vec4, Vec4);
pub type Mat4x2 = (Vec2, Vec2, Vec2, Vec2);
pub type Mat3x4 = (Vec4, Vec4, Vec4);
pub type Mat4x3 = (Vec3, Vec3, Vec3, Vec3);

pub type Vec2 = vec2<f32>;
pub type Vec3 = vec3<f32>;
pub type Vec4 = vec4<f32>;

pub type uVec2 = vec2<u32>;
pub type uVec3 = vec3<u32>;
pub type uVec4 = vec4<u32>;

pub type iVec2 = vec2<i32>;
pub type iVec3 = vec3<i32>;
pub type iVec4 = vec4<i32>;

pub type ulVec2 = vec2<usize>;
pub type ulVec3 = vec3<usize>;
pub type ulVec4 = vec4<usize>;

pub type vec2<T> = (T, T);
pub type vec3<T> = (T, T, T);
pub type vec4<T> = (T, T, T, T);
