use gl;

use crate::texture;
use crate::gl_buffers;

pub struct Sprite {
  pub tex : texture::Texture,
  pub vao :  gl::types::GLuint
}

pub fn create_sprite(image: &str, pixels_per_unit: f32, gl: &gl::Gl) -> Sprite {
  let tex = texture::create_texture(image, gl);
  let width = tex.width as f32 / pixels_per_unit;
  let height = tex.height as f32/ pixels_per_unit;

  let vertices = vec![
    -width * 0.5, -height * 0.5, 0.0,
    -width * 0.5, height * 0.5, 0.0,
    width * 0.5, height * 0.5, 0.0,
    
    width * 0.5, height * 0.5, 0.0,
    width * 0.5, -height * 0.5, 0.0,
    -width * 0.5, -height * 0.5, 0.0,
  ];

  let texture_coordinates = vec![
    0.0, 1.0,
    0.0, 0.0,
    1.0, 0.0,
    
    1.0, 0.0,
    1.0, 1.0,
    0.0, 1.0,
  ];

  let vertices_id = gl_buffers::create_buffer(&vertices, gl);
  let tex_coords_id = gl_buffers::create_buffer(&texture_coordinates, gl);
  let vao = gl_buffers::create_vertex_array_object(vertices_id, 3, tex_coords_id, 2, gl);

  return Sprite {
    tex: tex,
    vao: vao
  };
}