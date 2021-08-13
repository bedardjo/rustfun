extern crate gl;
extern crate sdl2;
extern crate image;

pub mod render_gl;
pub mod mat4;
pub mod vec3;
pub mod colored_shape;
pub mod renderable_colored_shape;
pub mod texture;

fn main() {
  print!("{}", std::env::current_dir().unwrap().display());
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
  let mut x_translation = 0.0;
  let mut y_translation = 0.0;
  let aspect_ratio = 900.0 / 700.0;
  let width = aspect_ratio * 10.0;
  let projection = mat4::orthographic(-width * 0.5, width * 0.5, -5.0, 5.0, -1.0, 1.0);

    let _gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| {
        video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
    });

    // set up shader program

    use std::ffi::CString;
    let vert_shader = render_gl::Shader::from_vert_source(
        &gl,
        &CString::new(include_str!("shaders/triangle.vert")).unwrap(),
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("shaders/triangle.frag")).unwrap(),
    ).unwrap();

    let shader_program =
        render_gl::Program::from_shaders(&gl, &[vert_shader, frag_shader]).unwrap();

    let img_vert_shader = render_gl::Shader::from_vert_source(
      &gl,
      &CString::new(include_str!("shaders/simple_image.vert")).unwrap(),
    ).unwrap();
  
    let img_frag_shader = render_gl::Shader::from_frag_source(
        &gl,
        &CString::new(include_str!("shaders/simple_image.frag")).unwrap(),
    ).unwrap();
  
    let img_shader_program =
        render_gl::Program::from_shaders(&gl, &[img_vert_shader, img_frag_shader]).unwrap();

  let black : vec3::Vec3 = [0.0, 0.0, 0.0];
  let white : vec3::Vec3 = [1.0, 1.0, 1.0];
  let renderable_colored_shape = renderable_colored_shape::create(colored_shape::equilateral_triangle(0.2), &gl);
  let black_square = renderable_colored_shape::create(colored_shape::square(1.0, &black), &gl);
  let white_square = renderable_colored_shape::create(colored_shape::square(1.0, &white), &gl);

  let black_bishop_tex = texture::create_texture("./imagery/chess_pieces/black_bishop.png", &gl);

    unsafe {
        gl.Viewport(0, 0, 900, 700);
        gl.ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // main loop

    let mvp_str = CString::new("mvp").unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown { timestamp:_, window_id:_, keycode, scancode:_, keymod:_, repeat:_ } => {
                  match keycode {
                    Some(code) => {
                      match code {
                        sdl2::keyboard::Keycode::W => {
                          y_translation += 0.5;
                        }
                        sdl2::keyboard::Keycode::A => {
                          x_translation -= 0.5;
                        }
                        sdl2::keyboard::Keycode::S => {
                          y_translation -= 0.5;
                        }
                        sdl2::keyboard::Keycode::D => {
                          x_translation += 0.5;
                        }
                        _ => {}
                      }
                    },
                    None => {}
                  }
                }
                _ => {}
            }
        }

        unsafe {
          gl.Clear(gl::COLOR_BUFFER_BIT);

          shader_program.set_used();
          for y in 0..8 {
            for x in 0..8 {
              let translation_matrix = mat4::translation(-3.5 + x as f32, -3.5 + y as f32, 0.0);
              let m = mat4::col_mul(projection, translation_matrix);
              gl.UniformMatrix4fv(gl.GetUniformLocation(shader_program.id, mvp_str.as_ptr()), 1, gl::FALSE, m.as_ptr());

              if (y + x) % 2 == 0 {
                gl.BindVertexArray(white_square.vao);
                gl.DrawArrays(gl::TRIANGLES, 0, 6);
              } else {
                gl.BindVertexArray(black_square.vao);
                gl.DrawArrays(gl::TRIANGLES, 0, 6);
              }
            }
          }
            
            let translation_matrix = mat4::translation(x_translation, y_translation, 0.0);
            let mvp = mat4::col_mul(projection, translation_matrix);

            gl.BindVertexArray(renderable_colored_shape.vao);
            gl.UniformMatrix4fv(gl.GetUniformLocation(shader_program.id, mvp_str.as_ptr()), 1, gl::FALSE, mvp.as_ptr());
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                6,             // number of indices to be rendered
            );
            
            // gl.UniformMatrix4fv(gl.GetUniformLocation(shader_program.id, mvp_str.as_ptr()), 1, gl::FALSE, projection.as_ptr());
            // 
            // gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.gl_swap_window();
    }
}