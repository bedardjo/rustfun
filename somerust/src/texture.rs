use gl;

use image::{DynamicImage, GenericImageView};

pub struct Texture {
  pub id: gl::types::GLuint,
  pub width: u32,
  pub height: u32,
}

pub fn create_texture(image: &str, gl: &gl::Gl) -> Texture {
  let img = image::open(image).unwrap();

  let w = img.width();
  let h = img.height();

  let image_data = match img {
    DynamicImage::ImageRgba8(i) => i,
    img => img.to_rgba8()
  };
  let raw_data = image_data.into_raw();

  let mut id: gl::types::GLuint = 0;
  unsafe {
    gl.GenTextures(1, &mut id);
    gl.BindTexture(gl::TEXTURE_2D, id);

    gl.ActiveTexture(gl::TEXTURE0);

    gl.TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as f32);
    gl.TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as f32);

    gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, w as i32, h as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, raw_data.as_ptr() as *const core::ffi::c_void);

    gl.BindTexture(gl::TEXTURE_2D, 0);
  }

  return Texture {
    id: id,
    width: w,
    height: h
  }
}