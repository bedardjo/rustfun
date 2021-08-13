use gl;

use crate::vec3;
use crate::colored_shape::ColoredShape;
use crate::gl_buffers;

pub struct RenderableColoredShape {
  pub shape: ColoredShape,
  pub vao: gl::types::GLuint
}

pub fn create(shape: ColoredShape, gl: &gl::Gl) -> RenderableColoredShape {
  let vertices = vec3::flatten(&shape.vertices);
  let colors = vec3::flatten(&shape.colors);

  let vertices_id = gl_buffers::create_buffer(&vertices, gl);
  let tex_coords_id = gl_buffers::create_buffer(&colors, gl);
  let vao = gl_buffers::create_vertex_array_object(vertices_id, 3, tex_coords_id, 3, gl);

  return RenderableColoredShape {
    shape: shape,
    vao: vao
  };
}