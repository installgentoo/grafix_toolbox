use super::policy::*;
use crate::uses::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_1D;
impl TexType for GL_TEXTURE_1D {
	const TYPE: GLenum = gl::TEXTURE_1D;
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_2D;
impl TexType for GL_TEXTURE_2D {
	const TYPE: GLenum = gl::TEXTURE_2D;
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_3D;
impl TexType for GL_TEXTURE_3D {
	const TYPE: GLenum = gl::TEXTURE_3D;
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_1D_ARRAY;
impl TexType for GL_TEXTURE_1D_ARRAY {
	const TYPE: GLenum = gl::TEXTURE_1D_ARRAY;
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_2D_ARRAY;
impl TexType for GL_TEXTURE_2D_ARRAY {
	const TYPE: GLenum = gl::TEXTURE_2D_ARRAY;
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_CUBE_MAP;
impl TexType for GL_TEXTURE_CUBE_MAP {
	const TYPE: GLenum = gl::TEXTURE_CUBE_MAP;
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_CUBE_MAP_ARRAY;
impl TexType for GL_TEXTURE_CUBE_MAP_ARRAY {
	const TYPE: GLenum = gl::TEXTURE_CUBE_MAP_ARRAY;
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_BUFFER;
impl TexType for GL_TEXTURE_BUFFER {
	const TYPE: GLenum = gl::TEXTURE_BUFFER;
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_2D_MULTISAMPLE;
impl TexType for GL_TEXTURE_2D_MULTISAMPLE {
	const TYPE: GLenum = gl::TEXTURE_2D_MULTISAMPLE;
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GL_TEXTURE_2D_MULTISAMPLE_ARRAY;
impl TexType for GL_TEXTURE_2D_MULTISAMPLE_ARRAY {
	const TYPE: GLenum = gl::TEXTURE_2D_MULTISAMPLE_ARRAY;
}
