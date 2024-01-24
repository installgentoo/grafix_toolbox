#![allow(dead_code, clippy::too_many_arguments)]
use crate::lib::*;

#[cfg(not(feature = "gl45"))]
pub const GL_VERSION: (u32, u32, STR) = (3, 3, "#version 330 core\n");
#[cfg(feature = "gl45")]
pub const GL_VERSION: (u32, u32, STR) = (4, 5, "#version 450 core\n");

pub const GLSL_VERSION: STR = GL_VERSION.2;

#[cfg(not(debug_assertions))]
pub const IS_DEBUG: bool = false;
#[cfg(debug_assertions)]
pub const IS_DEBUG: bool = true;

#[cfg(not(feature = "gl45"))]
macro_rules! G {
	($g33: expr, $g45: expr) => {
		$g33
	};
}
#[cfg(feature = "gl45")]
macro_rules! G {
	($g33: expr, $g45: expr) => {
		$g45
	};
}

pub unsafe fn glCreateBuffer(obj: &mut u32) {
	G!(gl::GenBuffers(1, obj), gl::CreateBuffers(1, obj));
}
pub unsafe fn glCreateVao(obj: &mut u32) {
	G!(gl::GenVertexArrays(1, obj), gl::CreateVertexArrays(1, obj));
}
pub unsafe fn glCreateTexture(_typ: GLenum, obj: &mut u32) {
	G!(gl::GenTextures(1, obj), gl::CreateTextures(_typ, 1, obj));
}
pub unsafe fn glDeleteTexture(obj: &mut u32) {
	G!(
		{
			if *obj == *bound_tex33() {
				*bound_tex33() = 0;
			}
			gl::DeleteTextures(1, obj);
		},
		gl::DeleteTextures(1, obj)
	);
}
pub unsafe fn glCreateFramebuff(obj: &mut u32) {
	G!(gl::GenFramebuffers(1, obj), gl::CreateFramebuffers(1, obj));
}
pub unsafe fn glCreateRenderbuff(obj: &mut u32) {
	G!(gl::GenRenderbuffers(1, obj), gl::CreateRenderbuffers(1, obj));
}
pub unsafe fn glBufferStorage(_typ: GLenum, obj: u32, size: isize, data: *const GLvoid, _usage: GLenum) {
	G!(
		{
			gl::BindBuffer(_typ, obj);
			gl::BufferData(_typ, size, data, gl::DYNAMIC_DRAW);
		},
		gl::NamedBufferStorage(obj, size, data, _usage)
	);
}
pub unsafe fn glBufferSubData(_typ: GLenum, obj: u32, offset: isize, size: isize, data: *const GLvoid) {
	G!(
		{
			gl::BindBuffer(_typ, obj);
			gl::BufferSubData(_typ, offset, size, data);
		},
		gl::NamedBufferSubData(obj, offset, size, data)
	);
}
pub unsafe fn glMapBufferRange(_typ: GLenum, obj: u32, offset: isize, length: isize, access: GLbitfield) -> *mut GLvoid {
	G!(
		{
			gl::BindBuffer(_typ, obj);
			gl::MapBufferRange(_typ, offset, length, access)
		},
		gl::MapNamedBufferRange(obj, offset, length, access)
	)
}
pub unsafe fn glUnmapBuffer(_typ: GLenum, obj: u32) -> GLbool {
	G!(
		{
			gl::BindBuffer(_typ, obj);
			gl::UnmapBuffer(_typ)
		},
		gl::UnmapNamedBuffer(obj)
	)
}
pub unsafe fn glVaoElementBuffer(vao: u32, buf: u32) {
	G!(
		{
			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buf);
			gl::BindVertexArray(0);
		},
		gl::VertexArrayElementBuffer(vao, buf)
	);
}
pub unsafe fn glVertexAttribFormat(vao: u32, buf: u32, idx: u32, size: u32, typ: GLenum, normalized: GLbool, stride: u32, offset: u32, t_size: u32) {
	G!(
		{
			gl::BindVertexArray(vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, buf);
			gl::EnableVertexAttribArray(idx);
			gl::VertexAttribPointer(idx, i32(size), typ, normalized, i32(stride), (offset * t_size) as *const GLvoid);
			gl::BindVertexArray(0);
		},
		{
			gl::EnableVertexArrayAttrib(vao, idx);
			gl::VertexArrayVertexBuffer(vao, idx, buf, 0, i32((size + stride) * t_size));
			gl::VertexArrayAttribFormat(vao, idx, i32(size), typ, normalized, offset * t_size);
			gl::VertexArrayAttribBinding(vao, idx, idx);
		}
	);
}
pub unsafe fn glTextureBuffer(_typ: GLenum, tex: u32, fmt: GLenum, buf: u32) {
	G!(
		{
			gl::BindTexture(_typ, tex);
			gl::TexBuffer(_typ, fmt, buf);
			gl::BindTexture(_typ, *bound_tex33());
		},
		gl::TextureBuffer(tex, fmt, buf)
	);
}
pub unsafe fn glTextureStorage1D(_typ: GLenum, tex: u32, levels: i32, fmt: GLenum, w: i32) {
	G!(
		{
			gl::BindTexture(_typ, tex);
			let mut w = w;
			for lvl in 0..levels {
				gl::TexImage1D(_typ, lvl, formatDepth45to33(fmt), w, 0, gl::RGBA, gl::FLOAT, 0 as *const GLvoid);
				w = 1.max(w / 2);
			}
			gl::BindTexture(_typ, *bound_tex33());
		},
		gl::TextureStorage1D(tex, levels, fmt, w)
	);
}
pub unsafe fn glTextureStorage2D(_typ: GLenum, tex: u32, levels: i32, fmt: GLenum, w: i32, h: i32) {
	G!(
		{
			gl::BindTexture(_typ, tex);
			let (mut w, mut h) = (w, h);
			for lvl in 0..levels {
				if _typ == gl::TEXTURE_CUBE_MAP {
					for f in 0..6 {
						let f = gl::TEXTURE_CUBE_MAP_POSITIVE_X + f;
						gl::TexImage2D(f, lvl, formatDepth45to33(fmt), w, h, 0, gl::RGBA, gl::FLOAT, 0 as *const GLvoid);
					}
				} else {
					gl::TexImage2D(_typ, lvl, formatDepth45to33(fmt), w, h, 0, gl::RGBA, gl::FLOAT, 0 as *const GLvoid);
				}
				w = 1.max(w / 2);
				h = 1.max(h / 2);
			}
			gl::BindTexture(_typ, *bound_tex33());
		},
		gl::TextureStorage2D(tex, levels, fmt, w, h)
	);
}
pub unsafe fn glTextureStorage3D(_typ: GLenum, tex: u32, levels: i32, fmt: GLenum, w: i32, h: i32, d: i32) {
	G!(
		{
			gl::BindTexture(_typ, tex);
			let (mut w, mut h, mut d) = (w, h, d);
			for lvl in 0..levels {
				gl::TexImage3D(_typ, lvl, formatDepth45to33(fmt), w, h, d, 0, gl::RGBA, gl::FLOAT, 0 as *const GLvoid);
				w = 1.max(w / 2);
				h = 1.max(h / 2);
				d = 1.max(d / 2);
			}
			gl::BindTexture(_typ, *bound_tex33());
		},
		gl::TextureStorage3D(tex, levels, fmt, w, h, d)
	);
}
pub unsafe fn glTextureSubImage1D(_typ: GLenum, tex: u32, lvl: i32, x: i32, w: i32, fmt: GLenum, t: GLenum, data: *const GLvoid) {
	G!(
		{
			gl::BindTexture(_typ, tex);
			gl::TexSubImage1D(_typ, lvl, x, w, fmt, t, data);
			gl::BindTexture(_typ, *bound_tex33());
		},
		gl::TextureSubImage1D(tex, lvl, x, w, fmt, t, data)
	);
}
pub unsafe fn glTextureSubImage2D(_typ: GLenum, tex: u32, lvl: i32, x: i32, y: i32, w: i32, h: i32, fmt: GLenum, t: GLenum, data: *const GLvoid) {
	G!(
		{
			gl::BindTexture(_typ, tex);
			gl::TexSubImage2D(_typ, lvl, x, y, w, h, fmt, t, data);
			gl::BindTexture(_typ, *bound_tex33());
		},
		gl::TextureSubImage2D(tex, lvl, x, y, w, h, fmt, t, data)
	);
}
pub unsafe fn glTextureSubImage3D(_typ: GLenum, tex: u32, lvl: i32, x: i32, y: i32, z: i32, w: i32, h: i32, d: i32, fmt: GLenum, t: GLenum, data: *const GLvoid) {
	G!(
		{
			gl::BindTexture(_typ, tex);
			if _typ == gl::TEXTURE_CUBE_MAP {
				gl::TexSubImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + u32(z), lvl, x, y, w, h, fmt, t, data);
			} else {
				gl::TexSubImage3D(_typ, lvl, x, y, z, w, h, d, fmt, t, data);
			}
			gl::BindTexture(_typ, *bound_tex33());
		},
		gl::TextureSubImage3D(tex, lvl, x, y, z, w, h, d, fmt, t, data)
	);
}
pub unsafe fn glBindTextureUnit(_typ: GLenum, unit: u32, tex: u32) {
	G!(
		{
			gl::ActiveTexture(gl::TEXTURE0 + unit);
			gl::BindTexture(_typ, tex);
			*bound_tex33() = tex;
		},
		gl::BindTextureUnit(unit, tex)
	);
}
pub unsafe fn glGenMipmaps(_typ: GLenum, tex: u32) {
	G!(
		{
			gl::BindTexture(_typ, tex);
			gl::GenerateMipmap(_typ);
			gl::BindTexture(_typ, *bound_tex33());
		},
		gl::GenerateTextureMipmap(tex)
	);
}
pub unsafe fn glGetTexture(_typ: GLenum, tex: u32, lvl: i32, fmt: GLenum, t: GLenum, _size: i32, data: *mut GLvoid) {
	G!(
		{
			gl::BindTexture(_typ, tex);
			gl::GetTexImage(_typ, lvl, fmt, t, data);
			gl::BindTexture(_typ, *bound_tex33());
		},
		gl::GetTextureImage(tex, lvl, fmt, t, _size, data)
	);
}
pub unsafe fn glClearFramebuff(fb: u32, typ: GLenum, buffidx: i32, val: *const f32) {
	G!(
		{
			gl::BindFramebuffer(gl::FRAMEBUFFER, fb);
			gl::ClearBufferfv(typ, buffidx, val);
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		},
		gl::ClearNamedFramebufferfv(fb, typ, buffidx, val)
	);
}
pub unsafe fn glFramebuffTex(fb: u32, tex: u32, attach: GLenum) {
	G!(
		{
			gl::BindFramebuffer(gl::FRAMEBUFFER, fb);
			gl::FramebufferTexture(gl::FRAMEBUFFER, attach, tex, 0);
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		},
		gl::NamedFramebufferTexture(fb, attach, tex, 0)
	);
}
pub unsafe fn glFramebuffRenderbuff(fb: u32, rb: u32, attach: GLenum) {
	G!(
		{
			gl::BindFramebuffer(gl::FRAMEBUFFER, fb);
			gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, attach, gl::RENDERBUFFER, rb);
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		},
		gl::NamedFramebufferRenderbuffer(fb, attach, gl::RENDERBUFFER, rb)
	);
}
pub unsafe fn glRenderbuffStorage(fb: u32, sampl: i32, fmt: GLenum, w: i32, h: i32) {
	G!(
		{
			gl::BindRenderbuffer(gl::RENDERBUFFER, fb);
			if sampl == 1 {
				gl::RenderbufferStorage(gl::RENDERBUFFER, fmt, w, h);
			} else {
				gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, sampl, fmt, w, h);
			}
			gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
		},
		if sampl == 1 {
			gl::NamedRenderbufferStorage(fb, fmt, w, h);
		} else {
			gl::NamedRenderbufferStorageMultisample(fb, sampl, fmt, w, h);
		}
	);
}

fn formatDepth45to33(fmt: GLenum) -> i32 {
	i32(if fmt == gl::DEPTH_COMPONENT32F || fmt == gl::DEPTH_COMPONENT24 || fmt == gl::DEPTH_COMPONENT16 {
		WARN!("Using unspecified GL_DEPTH_COMPONENT size");
		gl::DEPTH_COMPONENT
	} else {
		fmt
	})
}

fn bound_tex33() -> &'static mut u32 {
	static mut STATE: u32 = 0;
	unsafe { &mut STATE }
}
