extern crate gl;
extern crate sdl2;
extern crate image;

pub mod render_gl;
pub mod mat4;
pub mod vec3;
pub mod colored_shape;
pub mod renderable_colored_shape;
pub mod texture;
pub mod gl_buffers;
pub mod sprite;
pub mod chess_square;
pub mod chess_board;
pub mod chess_move;
pub mod castling;
pub mod chess_color;
pub mod thought_node;
pub mod chess_scoring;
pub mod multi_thought_node;

use chess_square::{ChessSquare};
use rand::Rng;
use std::{collections::{HashMap}};
use thought_node::ThoughtNode;
use multi_thought_node::MultiThoughtNode;
use std::time;

use crate::{castling::Castling, chess_board::{ChessBoard, copy_board}, chess_color::ChessColor, chess_move::{ChessMove, get_check, undo_move}};

fn main() {
  let mut rng = rand::thread_rng();
  let sdl = sdl2::init().unwrap();
  let video_subsystem = sdl.video().unwrap();
  let gl_attr = video_subsystem.gl_attr();
    
  gl_attr.set_multisample_buffers(1);
  gl_attr.set_multisample_samples(8);

  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(4, 1);

  let screen_width = 1200;
  let screen_height = 1000;
  let pixels_per_unit = 120.0;
  let screen_unit_width = screen_width as f32 / pixels_per_unit;
  let screen_unit_height = screen_height as f32 / pixels_per_unit;

  let window = video_subsystem
      .window("Game", screen_width, screen_height)
      .opengl()
      .resizable()
      .build()
      .unwrap();
  let projection = mat4::orthographic(
    -screen_unit_width * 0.5,
    screen_unit_width * 0.5,
    -screen_unit_height * 0.5,
    screen_unit_height * 0.5,
    -1.0, 1.0);

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

  let mut active_computer = true;
  let mut computer_can_move_instant = time::Instant::now();
  
  let img_shader_program =
      render_gl::Program::from_shaders(&gl, &[img_vert_shader, img_frag_shader]).unwrap();

  let black : vec3::Vec3 = [46.0 / 100.0, 58.0 / 100.0, 33.0 / 100.0];
  let white : vec3::Vec3 = [93.0 / 100.0, 93.0 / 100.0, 82.0 / 100.0];
  let black_square = renderable_colored_shape::create(colored_shape::square(0.5, &black), &gl);
  let white_square = renderable_colored_shape::create(colored_shape::square(0.5, &white), &gl);

  let mut sprites : HashMap<ChessSquare, sprite::Sprite> = HashMap::new();
  sprites.insert(ChessSquare::WhitePawn, sprite::create_sprite("./imagery/chess_pieces/white_pawn_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::WhiteBishop, sprite::create_sprite("./imagery/chess_pieces/white_bishop_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::WhiteKing, sprite::create_sprite("./imagery/chess_pieces/white_king_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::WhiteKnight, sprite::create_sprite("./imagery/chess_pieces/white_knight_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::WhiteQueen, sprite::create_sprite("./imagery/chess_pieces/white_queen_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::WhiteRook, sprite::create_sprite("./imagery/chess_pieces/white_rook_120.png", pixels_per_unit, &gl));
  
  sprites.insert(ChessSquare::BlackPawn, sprite::create_sprite("./imagery/chess_pieces/black_pawn_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::BlackBishop, sprite::create_sprite("./imagery/chess_pieces/black_bishop_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::BlackKing, sprite::create_sprite("./imagery/chess_pieces/black_king_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::BlackKnight, sprite::create_sprite("./imagery/chess_pieces/black_knight_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::BlackQueen, sprite::create_sprite("./imagery/chess_pieces/black_queen_120.png", pixels_per_unit, &gl));
  sprites.insert(ChessSquare::BlackRook, sprite::create_sprite("./imagery/chess_pieces/black_rook_120.png", pixels_per_unit, &gl));

  let _fen1 = String::from("RNBQK1NR/PPPP1PPP/8/4P3/8/B7/1ppppppp/rnbqkbnr b KQkq - 0 3");
  // let mut chess_board = chess_board::from_forsyth_edwards_notation(&fen1);
  let mut chess_board = chess_board::create_new_board();

  let mut game_moves: Vec<ChessMove> = Vec::new();
  let mut move_start_coords : Option<(usize, usize)> = None;

  unsafe {
      gl.Viewport(0, 0, screen_width as i32, screen_height as i32);
      gl.ClearColor(0.3, 0.3, 0.5, 1.0);
      gl.Enable(gl::MULTISAMPLE);
  }

    let mvp_str = CString::new("mvp").unwrap();
    let tex_str = CString::new("tex").unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseButtonDown { timestamp:_, window_id:_, which:_, mouse_btn:_, clicks:_, x, y} => {
                  let (ux, uy) = get_unit_coords(x, y, screen_width, screen_height, pixels_per_unit);
                  if -4.0 < ux && ux < 4.0 && -4.0 < uy && uy < 4.0 {
                    let (bx, by) = get_board_coords((ux, uy));
                    let square = &chess_board.squares[by][bx];
                    if square != &ChessSquare::Empty && square.get_color() == chess_board.current_player {
                      move_start_coords = Some((bx, by));
                    }
                  }
                },
                sdl2::event::Event::MouseButtonUp { timestamp:_, window_id:_, which:_, mouse_btn:_, clicks:_, x, y} => {
                  let (ux, uy) = get_unit_coords(x, y, screen_width, screen_height, pixels_per_unit);
                  if -4.0 < ux && ux < 4.0 && -4.0 < uy && uy < 4.0 {
                    let (bx, by) = get_board_coords((ux, uy));
                    match move_start_coords {
                      Some(c) => {
                        let potential_capture = &chess_board.squares[by][bx];
                        if potential_capture == &ChessSquare::Empty || potential_capture.get_color() == chess_board.current_player.get_opposite() {
                          let potential_move: ChessMove = if &chess_board.squares[c.1][c.0] == &ChessSquare::WhiteKing && (bx as i32 - c.0 as i32).abs() == 2 {
                            ChessMove{
                              piece: chess_board.squares[c.1][c.0].clone(),
                              x: c.0,
                              y: c.1,
                              to_x: bx,
                              to_y: by,
                              capture: if potential_capture == &ChessSquare::Empty { None } else { Some(potential_capture.clone()) },
                              promotion: None,
                              castling: Some(if bx > c.0 { Castling::WhiteShort } else { Castling::WhiteLong }),
                              en_pessant: false,
                            }
                          } else {
                            ChessMove{
                              piece: chess_board.squares[c.1][c.0].clone(),
                              x: c.0,
                              y: c.1,
                              to_x: bx,
                              to_y: by,
                              capture: if potential_capture == &ChessSquare::Empty { None } else { Some(potential_capture.clone()) },
                              promotion: if chess_board.squares[c.1][c.0] == ChessSquare::WhitePawn && by == 7 { Some(ChessSquare::WhiteQueen) } else { None },
                              castling: None,
                              en_pessant: false,
                            }
                          };
                          
                          let valid_moves = chess_board.get_valid_moves();
                          if valid_moves.contains(&potential_move) {
                            chess_board = chess_board.do_move(&potential_move);
                            print!("{}. {} ", (chess_board.move_number / 2 + 1), potential_move.to_string());
                            computer_can_move_instant = time::Instant::now() + time::Duration::from_millis(200);
                            game_moves.push(potential_move.clone());
                          } else {
                            print!("{} was not valid {} {}! Trying en pessant.", potential_move.to_string(), potential_move.capture.is_some(), potential_move.promotion.is_some());
                            let test_en_pessant = ChessMove{
                              piece: chess_board.squares[c.1][c.0].clone(),
                              x: c.0,
                              y: c.1,
                              to_x: bx,
                              to_y: by,
                              capture: Some(ChessSquare::BlackPawn),
                              promotion: None,
                              castling: None,
                              en_pessant: true,
                            };
                            if valid_moves.contains(&test_en_pessant) {
                              chess_board = chess_board.do_move(&test_en_pessant);
                              print!("{}. {} ", (chess_board.move_number / 2 + 1), test_en_pessant.to_string());
                              computer_can_move_instant = time::Instant::now() + time::Duration::from_millis(200);
                              game_moves.push(test_en_pessant.clone());
                            }
                          }
                        }
                      }
                      None => {}
                    }
                  }
                  move_start_coords = None;
                },
                sdl2::event::Event::KeyDown { timestamp:_, window_id:_, keycode, scancode:_, keymod:_, repeat:_ } => {
                  match keycode {
                    Some(code) => {
                      match code {
                        sdl2::keyboard::Keycode::M => {
                          print!("====valid moves====\n");
                          for m in chess_board.get_valid_moves().iter() {
                            print!("{}\n", m.to_string());
                          }
                          print!("===================\n");
                        }
                        sdl2::keyboard::Keycode::G => {
                          match get_check(&chess_board.squares, &ChessColor::Black).or(get_check(&chess_board.squares, &ChessColor::White)) {
                            Some(c) => {
                              match c {
                                ChessColor::Black => { print!("black is in check\n") }
                                ChessColor::White => { print!("white is in check\n") }
                              }
                            }
                            None => {}
                          }
                        }
                        sdl2::keyboard::Keycode::F => {
                          print!("{}\n", chess_board.get_forsyth_edwards_notation());
                        }
                        sdl2::keyboard::Keycode::Escape => {
                          break 'main;
                        }
                        sdl2::keyboard::Keycode::Right => {
                          let mut possible_moves = chess_board.get_valid_moves();
                          let chosen_move = possible_moves.swap_remove(rng.gen_range(0..possible_moves.len()));
                          chess_board = chess_board.do_move(&chosen_move);
                          game_moves.push(chosen_move);
                        }
                        sdl2::keyboard::Keycode::Left => {
                          let last_move = game_moves.pop();
                          match last_move {
                            Some(m) => {
                              let mut board_copy = copy_board(&chess_board.squares);
                              undo_move(&m, &mut board_copy);
                              chess_board = ChessBoard{
                                last_move: match game_moves.last() {
                                  Some(m) => Some(m.clone()),
                                  None => None,
                                },
                                squares: board_copy,
                                current_player: chess_board.current_player.get_opposite(),
                                available_castling: chess_board.available_castling,
                                move_number: chess_board.move_number - 1
                              };
                            },
                            _ => {}
                          }
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
        
        if active_computer && chess_board.current_player == ChessColor::Black && time::Instant::now() > computer_can_move_instant {
          let start = time::Instant::now();
          // let mut multi_thought_node = MultiThoughtNode{
          //   game_state: chess_board.clone(),
          //   children: Vec::new(),
          //   calculated_score: 0.0,
          //   thought_threads: 30
          // };
          // let mut thought_node = multi_thought_node.alphabeta(4);
          let mut thought_node = ThoughtNode{
            game_state: chess_board.clone(),
            children: Vec::new(),
            calculated_score: 0.0,
          };
          thought_node.alphabeta(5);
          if thought_node.children.is_empty() {
            active_computer = false;
          } else {
            let best_ai_move = thought_node.get_best_move();
            let stop = time::Instant::now();
            chess_board = chess_board.do_move(&best_ai_move);
            print!("{}\n", best_ai_move.to_string());
            let thought_duration = stop - start;
            print!("thought for {}\n", thought_duration.as_millis());
            game_moves.push(best_ai_move.clone());
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

          for y in 0..8 {
            for x in 0..8 {
              if chess_board.squares[7 - y][x] != ChessSquare::Empty {
                img_shader_program.set_used();
                gl.ActiveTexture(gl::TEXTURE0 + 0);
                gl.BindTexture(gl::TEXTURE_2D, sprites.get(&chess_board.squares[7 - y][x]).unwrap().tex.id);
                gl.Uniform1i(gl.GetUniformLocation(img_shader_program.id, tex_str.as_ptr()), 0);
                gl.BindVertexArray(sprites.get(&chess_board.squares[7 - y][x]).unwrap().vao);
                let translation_matrix = mat4::translation(-3.5 + x as f32, -3.5 + (7 - y) as f32, 0.0);
                let m = mat4::col_mul(projection, translation_matrix);
                gl.UniformMatrix4fv(gl.GetUniformLocation(img_shader_program.id, mvp_str.as_ptr()), 1, gl::FALSE, m.as_ptr());
                gl.DrawArrays(gl::TRIANGLES, 0, 6);
                gl.BindBuffer(gl::ARRAY_BUFFER, 0);
              }
            }
          }
        }

        window.gl_swap_window();
    }
}

fn get_unit_coords(screen_x: i32, screen_y: i32, screen_width: u32, screen_height: u32, unit_pixel_size: f32) -> (f32, f32) {
  let x_norm = screen_x as f32 / screen_width as f32;
  let y_norm = screen_y as f32 / screen_height as f32;
  let ux = (-0.5 + x_norm as f32) * (screen_width as f32 / unit_pixel_size);
  let uy = (0.5 - y_norm as f32) * (screen_height as f32 / unit_pixel_size);
  return (ux, uy);
}

fn get_board_coords(unit_coords: (f32, f32)) -> (usize, usize) {
  let bx = (unit_coords.0 + 3.5).round() as usize;
  let by = (unit_coords.1 + 3.5).round() as usize;
  return (bx, by);
}