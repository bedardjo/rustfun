use gl;

pub fn create_buffer(vertices: &Vec<f32>, gl: &gl::Gl) -> gl::types::GLuint {
  let mut buffer_id: gl::types::GLuint = 0;
  unsafe {
    gl.GenBuffers(1, &mut buffer_id);
    gl.BindBuffer(gl::ARRAY_BUFFER, buffer_id);
    gl.BufferData(
        gl::ARRAY_BUFFER, // target
        (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
        vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
        gl::STATIC_DRAW, // usage
    );
    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
  }
  return buffer_id;
}

pub fn create_vertex_array_object(
  buff_id_1: gl::types::GLuint, n_components_1: gl::types::GLint,
  buff_id_2: gl::types::GLuint, n_components_2: gl::types::GLint,
  gl: &gl::Gl
) -> gl::types::GLuint {
  let mut vao: gl::types::GLuint = 0;

  unsafe {
    gl.GenVertexArrays(1, &mut vao);
    gl.BindVertexArray(vao);

    gl.BindBuffer(gl::ARRAY_BUFFER, buff_id_1);
    gl.EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
    gl.VertexAttribPointer(
        0, // index of the generic vertex attribute ("layout (location = 0)")
        n_components_1, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        0 as gl::types::GLint, // stride (byte offset between consecutive attributes)
        std::ptr::null() // offset of the first component
    );

    gl.BindBuffer(gl::ARRAY_BUFFER, buff_id_2);
    gl.EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
    gl.VertexAttribPointer(
        1, // index of the generic vertex attribute ("layout (location = 1)")
        n_components_2, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        0 as gl::types::GLint, // stride (byte offset between consecutive attributes)
        std::ptr::null() // offset of the first component
    );

    gl.BindBuffer(gl::ARRAY_BUFFER, 0);
    gl.BindVertexArray(0);
  }

  return vao;
}