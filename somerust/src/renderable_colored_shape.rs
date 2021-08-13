use gl;

use crate::vec3;
use crate::colored_shape::ColoredShape;

pub struct RenderableColoredShape {
  pub shape: ColoredShape,
  pub vao: gl::types::GLuint
}

pub fn create(shape: ColoredShape, gl: &gl::Gl) -> RenderableColoredShape {
  let vertices = vec3::flatten(&shape.vertices);
  let mut vertices_buffer_id: gl::types::GLuint = 0;
  let colors = vec3::flatten(&shape.colors);
  let mut colors_buffer_id: gl::types::GLuint = 0;

  let mut vao: gl::types::GLuint = 0;
  unsafe {
    gl.GenBuffers(1, &mut vertices_buffer_id);

    gl.BindBuffer(gl::ARRAY_BUFFER, vertices_buffer_id);
    gl.BufferData(
        gl::ARRAY_BUFFER, // target
        (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
        vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
        gl::STATIC_DRAW, // usage
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);

    gl.GenBuffers(1, &mut colors_buffer_id);

    gl.BindBuffer(gl::ARRAY_BUFFER, colors_buffer_id);
    gl.BufferData(
        gl::ARRAY_BUFFER, // target
        (colors.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
        colors.as_ptr() as *const gl::types::GLvoid, // pointer to data
        gl::STATIC_DRAW, // usage
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);

    // set up vertex array object
    gl.GenVertexArrays(1, &mut vao);

    gl.BindVertexArray(vao);
    gl.BindBuffer(gl::ARRAY_BUFFER, vertices_buffer_id);

    gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    gl.VertexAttribPointer(
        0, // index of the generic vertex attribute ("layout (location = 0)")
        3, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        0 as gl::types::GLint, // stride (byte offset between consecutive attributes)
        std::ptr::null() // offset of the first component
    );

    gl.BindBuffer(gl::ARRAY_BUFFER, colors_buffer_id);
    gl.EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
    gl.VertexAttribPointer(
        1, // index of the generic vertex attribute ("layout (location = 0)")
        3, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        0 as gl::types::GLint, // stride (byte offset between consecutive attributes)
        0 as *const gl::types::GLvoid // offset of the first component
    );

    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);
  }
  return RenderableColoredShape {
    shape: shape,
    vao: vao
  };
}