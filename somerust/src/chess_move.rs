use core::panic;
use std::collections::HashSet;

use crate::chess_board::{ChessBoard, copy_board, get_fen_board_part};
use crate::chess_color::ChessColor;
use crate::chess_square::{ChessSquare};
use crate::castling::Castling;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ChessMove {
  pub piece : ChessSquare,
  pub x : usize,
  pub y : usize,
  pub to_x : usize,
  pub to_y: usize,
  
  pub capture : Option<ChessSquare>,
  pub promotion : Option<ChessSquare>,
  pub castling: Option<Castling>,
  pub en_pessant : bool,
}

impl ChessMove {
  pub fn to_string(&self) -> String {
    return match self.castling.as_ref() {
      Some(c) => match c {
        &Castling::BlackLong | &Castling::WhiteLong => {String::from("O-O-O")},       
         &Castling::WhiteShort | &Castling::BlackShort => {String::from("O-O")}
      }
      _ => match &self.piece {
          &ChessSquare::WhitePawn | &ChessSquare::BlackPawn => { [
            String::from("abcdefgh".chars().nth(self.to_x).unwrap()),
            (self.to_y + 1).to_string() ].join("")
          },
          _ => { [
            self.piece.get_pgn_char(),
            String::from("abcdefgh".chars().nth(self.to_x).unwrap()),
            (self.to_y + 1).to_string() ].join("")
          }
        }
    }
  }
}

fn get_rook_directions() -> Vec<(i32, i32)> {
  return vec![
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
  ];
}

fn get_bishop_directions() -> Vec<(i32, i32)> {
  return vec![
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
  ];
}

fn get_queen_directions() -> Vec<(i32, i32)> {
  return vec![
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
  ];
}

fn get_knight_moves() -> Vec<(i32, i32)> {
  return vec![
    (-1, 2),
    (1, 2),

    (2, 1),
    (2, -1),
    
    (-1, -2),
    (1, -2),
    
    (-2, 1),
    (-2, -1),
  ];
}

fn simple_move(piece: &ChessSquare, x: usize, y: usize, to_x: usize, to_y: usize) -> ChessMove {
  return ChessMove{
    piece: piece.clone(),
    x: x,
    y: y,
    to_x: to_x,
    to_y: to_y,
    capture: None,
    promotion: None,
    castling: None,
    en_pessant: false
  }
}

fn create_pawn_moves(x: usize, y: usize, to_x: usize, to_y: usize, piece: &ChessSquare, capture: Option<ChessSquare>) -> Vec<ChessMove> {
  let mut pawn_moves: Vec<ChessMove> = Vec::new();
  if to_y == 0 || to_y == 7 {
    let promotion_pieces = match piece.get_color() {
      ChessColor::White => [ChessSquare::WhiteBishop, ChessSquare::WhiteKnight, ChessSquare::WhiteQueen, ChessSquare::WhiteRook],
      ChessColor::Black => [ChessSquare::BlackBishop, ChessSquare::BlackKnight, ChessSquare::BlackQueen, ChessSquare::BlackRook]
    };
    for p in promotion_pieces {
      pawn_moves.push(ChessMove{
        piece: piece.clone(),
        x: x,
        y: y,
        to_x: to_x,
        to_y: to_y,
        capture: capture.clone(),
        promotion: Some(p),
        castling: None,
        en_pessant: false
      });
    }
  } else {
    pawn_moves.push(ChessMove{
      piece: piece.clone(),
      x: x,
      y: y,
      to_x: to_x,
      to_y: to_y,
      capture: capture,
      promotion: None,
      castling: None,
      en_pessant: false
    });
  }
  return pawn_moves;
}

fn potential_pawn_moves(last_move: &Option<ChessMove>, x: usize, y: usize, piece: &ChessSquare, board: &[[ChessSquare; 8]; 8]) -> Vec<ChessMove> {
  let mut pawn_moves: Vec<ChessMove> = Vec::new();
  let new_y  = match piece.get_color() {
    ChessColor::White => y + 1,
    _ => y - 1
  };
  let clr = piece.get_color();
  let opp = clr.get_opposite();
  if &board[new_y][x] == &ChessSquare::Empty {
    pawn_moves.append(&mut create_pawn_moves(x, y, x, new_y, piece, None));
  }
  // move 2 spaces on first move
  if clr == ChessColor::White && y == 1 && &board[2][x] == &ChessSquare::Empty && &board[3][x] == &ChessSquare::Empty {
    pawn_moves.push(simple_move(piece, x, y, x, 3));
  } else if clr == ChessColor::Black && y == 6 && &board[5][x] == &ChessSquare::Empty && &board[4][x] == &ChessSquare::Empty {
    pawn_moves.push(simple_move(piece, x, y, x, 4));
  }
  // Captures
  if x > 0 && board[new_y][x - 1] != ChessSquare::Empty && board[new_y][x - 1].get_color() == opp { // take left
    pawn_moves.append(&mut create_pawn_moves(x, y, x - 1, new_y, piece, Some(board[new_y][x - 1].clone())));
  }
  if x < 7 && board[new_y][x + 1] != ChessSquare::Empty && board[new_y][x + 1].get_color() == opp { // take right
    pawn_moves.append(&mut create_pawn_moves(x, y, x + 1, new_y, piece, Some(board[new_y][x + 1].clone())));
  }
  // En pessant
  if last_move.is_some() {
    let last = last_move.as_ref().unwrap();
    match last.piece {
      ChessSquare::WhitePawn if clr == ChessColor::Black && new_y == 2 && last.y == 1 && last.to_y == 3 && (x as i32 - last.x as i32).abs() == 1 => {
        pawn_moves.push(ChessMove{
          piece: piece.clone(),
          x: x,
          y: y,
          to_x: last.x,
          to_y: 2,
          capture: Some(ChessSquare::WhitePawn),
          promotion: None,
          castling: None,
          en_pessant: true
        });
      },
      ChessSquare::BlackPawn => {
        let available = clr == ChessColor::White && new_y == 5 && last.y == 6 && last.to_y == 4 && (x as i32 - last.x as i32).abs() == 1;
        if available {
          pawn_moves.push(ChessMove{
            piece: piece.clone(),
            x: x,
            y: y,
            to_x: last.x,
            to_y: 5,
            capture: Some(ChessSquare::BlackPawn),
            promotion: None,
            castling: None,
            en_pessant: true
          });
        }
      },
      _ => {}
    }
  }
  return pawn_moves;
}

fn get_potential_moves_for_directional_piece(player: &ChessColor, piece: &ChessSquare, x: usize, y: usize, directions: &Vec<(i32,i32)>, board: &[[ChessSquare; 8]; 8]) -> Vec<ChessMove> {
  let mut potential_moves: Vec<ChessMove> = Vec::new();
  for d in directions {
    let mut to_x = x as i32 + d.0;
    let mut to_y = y as i32 + d.1;
    while to_x >= 0 && to_x <= 7 && to_y >= 0 && to_y <= 7 {
      let move_piece = board[to_y as usize][to_x as usize].clone();
      if move_piece == ChessSquare::Empty {
        potential_moves.push(simple_move(piece, x, y, to_x as usize, to_y as usize));
        to_x += d.0;
        to_y += d.1;
      } else {
        if move_piece.get_color() == player.get_opposite() {
          potential_moves.push(ChessMove{
            piece: piece.clone(),
            x: x,
            y: y,
            to_x: to_x as usize,
            to_y: to_y as usize,
            capture: Some(move_piece),
            promotion: None,
            castling: None,
            en_pessant: false
          });
        }
        break;
      }
    }
  }
  return potential_moves;
}

fn get_potential_moves_for_knight(player: &ChessColor, piece: &ChessSquare, x: usize, y: usize, board: &[[ChessSquare; 8]; 8]) -> Vec<ChessMove> {
  let mut potential_moves: Vec<ChessMove> = Vec::new();
  for m in get_knight_moves() {
    let to_x = x as i32 + m.0;
    let to_y = y as i32 + m.1;
    if to_x >= 0 && to_x <= 7 && to_y >= 0 && to_y <= 7 {
      let move_piece = board[to_y as usize][to_x as usize].clone();
      if move_piece == ChessSquare::Empty || move_piece.get_color() == player.get_opposite() {
        potential_moves.push(ChessMove{
          piece: piece.clone(),
          x: x,
          y: y,
          to_x: to_x as usize,
          to_y: to_y as usize,
          capture: if move_piece == ChessSquare::Empty { None } else { Some(move_piece) },
          promotion: None,
          castling: None,
          en_pessant: false
        });
      }
    }
  }
  return potential_moves;
}

fn get_potential_moves_for_king(player: &ChessColor, piece: &ChessSquare, x: usize, y: usize, board: &[[ChessSquare; 8]; 8], available_castling: &HashSet<Castling>) -> Vec<ChessMove> {
  let mut potential_moves: Vec<ChessMove> = Vec::new();
  for m in get_queen_directions() {
    let to_x = x as i32 + m.0;
    let to_y = y as i32 + m.1;
    if to_x >= 0 && to_x <= 7 && to_y >= 0 && to_y <= 7 {
      let move_piece = board[to_y as usize][to_x as usize].clone();
      if move_piece == ChessSquare::Empty || move_piece.get_color() == player.get_opposite() {
        potential_moves.push(ChessMove{
          piece: piece.clone(),
          x: x,
          y: y,
          to_x: to_x as usize,
          to_y: to_y as usize,
          capture: if move_piece == ChessSquare::Empty { None } else { Some(move_piece) },
          promotion: None,
          castling: None,
          en_pessant: false
        });
      }
    }
  }
  if piece == &ChessSquare::WhiteKing {
    if available_castling.contains(&Castling::WhiteLong) && board[0][1] == ChessSquare::Empty && board[0][2] == ChessSquare::Empty && board[0][3] == ChessSquare::Empty {
      potential_moves.push(ChessMove{
        piece: piece.clone(),
        x: x,
        y: y,
        to_x: 2,
        to_y: 0,
        capture: None,
        promotion: None,
        castling: Some(Castling::WhiteLong),
        en_pessant: false
      });
    }
    if available_castling.contains(&Castling::WhiteShort) && board[0][5] == ChessSquare::Empty && board[0][6] == ChessSquare::Empty {
      potential_moves.push(ChessMove{
        piece: piece.clone(),
        x: x,
        y: y,
        to_x: 6,
        to_y: 0,
        capture: None,
        promotion: None,
        castling: Some(Castling::WhiteShort),
        en_pessant: false
      });
    }
  } else if piece == &ChessSquare::BlackKing {
    if available_castling.contains(&Castling::BlackLong) && board[7][1] == ChessSquare::Empty && board[7][2] == ChessSquare::Empty && board[7][3] == ChessSquare::Empty {
      potential_moves.push(ChessMove{
        piece: piece.clone(),
        x: x,
        y: y,
        to_x: 2,
        to_y: 7,
        capture: None,
        promotion: None,
        castling: Some(Castling::BlackLong),
        en_pessant: false
      });
    }
    if available_castling.contains(&Castling::BlackShort) && board[7][5] == ChessSquare::Empty && board[7][6] == ChessSquare::Empty {
      potential_moves.push(ChessMove{
        piece: piece.clone(),
        x: x,
        y: y,
        to_x: 6,
        to_y: 7,
        capture: None,
        promotion: None,
        castling: Some(Castling::BlackShort),
        en_pessant: false
      });
    }
  }
  return potential_moves;
}

fn get_potential_moves(last_move: &Option<ChessMove>, player: &ChessColor, board: &[[ChessSquare; 8]; 8], available_castling: &HashSet<Castling>) -> Vec<ChessMove> {
  let mut potential_moves: Vec<ChessMove> = Vec::new();
  for y in 0..8 {
    for x in 0..8 {
      let piece = &board[y][x];
      if piece != &ChessSquare::Empty && &piece.get_color() == player {
        match piece {
          ChessSquare::WhitePawn | ChessSquare::BlackPawn => {
            potential_moves.append(&mut potential_pawn_moves(last_move, x, y, piece, board));
          },
          ChessSquare::WhiteRook | ChessSquare::BlackRook => {
            potential_moves.append(&mut get_potential_moves_for_directional_piece(player, piece, x, y, &get_rook_directions(), board));
          },
          ChessSquare::WhiteBishop | ChessSquare::BlackBishop => {
            potential_moves.append(&mut get_potential_moves_for_directional_piece(player, piece, x, y, &get_bishop_directions(), board));
          },
          ChessSquare::WhiteQueen | ChessSquare::BlackQueen => {
            potential_moves.append(&mut get_potential_moves_for_directional_piece(player, piece, x, y, &get_queen_directions(), board));
          },
          ChessSquare::WhiteKnight | ChessSquare::BlackKnight => {
            potential_moves.append(&mut get_potential_moves_for_knight(player, piece, x, y, board));
          },
          ChessSquare::WhiteKing | ChessSquare::BlackKing => {
            potential_moves.append(&mut get_potential_moves_for_king(player, piece, x, y, board, available_castling));
          }
          _ => {}
        }
      }
    }
  }
  return potential_moves;
}

pub fn do_move(chess_move: &ChessMove, board: &mut [[ChessSquare; 8]; 8]) {
  if board[chess_move.y][chess_move.x] != chess_move.piece {
    let fen = ChessBoard{
      last_move: None,
      squares: copy_board(board),
      current_player: chess_move.piece.get_color(),
      available_castling: HashSet::new(),
      move_number: 0
    }.get_forsyth_edwards_notation();
    panic!("cannot perform {} {} {} {} on board {}", chess_move.to_string(), chess_move.en_pessant, chess_move.capture.is_some(), chess_move.promotion.is_some(), fen);
  }
  match &chess_move.castling {
    Some(castling) => {
      match castling {
        Castling::WhiteLong => {
          if board[0][0] != ChessSquare::WhiteRook || board[0][4] != ChessSquare::WhiteKing {
            panic!("cannot perform move on board {} on board {}", chess_move.to_string(), get_fen_board_part(board));
          }
          board[0][0] = ChessSquare::Empty;
          board[0][2] = ChessSquare::WhiteKing;
          board[0][3] = ChessSquare::WhiteRook;
          board[0][4] = ChessSquare::Empty;
        },
        Castling::WhiteShort => {
          if board[0][7] != ChessSquare::WhiteRook || board[0][4] != ChessSquare::WhiteKing {
            panic!("cannot perform move on board");
          }
          board[0][7] = ChessSquare::Empty;
          board[0][6] = ChessSquare::WhiteKing;
          board[0][5] = ChessSquare::WhiteRook;
          board[0][4] = ChessSquare::Empty;
        },
        Castling::BlackLong => {
          if board[7][0] != ChessSquare::BlackRook || board[7][4] != ChessSquare::BlackKing {
            panic!("cannot perform move on board");
          }
          board[7][0] = ChessSquare::Empty;
          board[7][2] = ChessSquare::BlackKing;
          board[7][3] = ChessSquare::BlackRook;
          board[7][4] = ChessSquare::Empty;
        },
        Castling::BlackShort => {
          if board[7][7] != ChessSquare::BlackRook || board[7][4] != ChessSquare::BlackKing {
            panic!("cannot perform move on board");
          }
          board[7][7] = ChessSquare::Empty;
          board[7][6] = ChessSquare::BlackKing;
          board[7][5] = ChessSquare::BlackRook;
          board[7][4] = ChessSquare::Empty;
        },
      }
    },
    None => {
      board[chess_move.y][chess_move.x] = ChessSquare::Empty;
      board[chess_move.to_y][chess_move.to_x] = match &chess_move.promotion {
        Some(p) => p.clone(),
        None => chess_move.piece.clone()
      };
      if chess_move.en_pessant {
        if board[chess_move.y][chess_move.to_x] == ChessSquare::Empty {
          panic!("not a valid en-pessant");
        }
        board[chess_move.y][chess_move.to_x] = ChessSquare::Empty;
      }
    }
  }
}

pub fn undo_move(chess_move: &ChessMove, board: &mut [[ChessSquare; 8]; 8]) {
  if !chess_move.en_pessant && chess_move.promotion.is_none() && board[chess_move.to_y][chess_move.to_x] != chess_move.piece {
    let fen = ChessBoard{
      last_move: None,
      squares: copy_board(board),
      current_player: chess_move.piece.get_color(),
      available_castling: HashSet::new(),
      move_number: 0
    }.get_forsyth_edwards_notation();
    panic!("bad state {} for move {}", fen, chess_move.to_string());
  }
  match &chess_move.castling {
    Some(castling) => {
      match castling {
        Castling::WhiteLong => {
          board[0][0] = ChessSquare::WhiteRook;
          board[0][2] = ChessSquare::Empty;
          board[0][3] = ChessSquare::Empty;
          board[0][4] = ChessSquare::WhiteKing;
        },
        Castling::WhiteShort => {
          board[0][7] = ChessSquare::WhiteRook;
          board[0][6] = ChessSquare::Empty;
          board[0][5] = ChessSquare::Empty;
          board[0][4] = ChessSquare::WhiteKing;
        },
        Castling::BlackLong => {
          board[7][0] = ChessSquare::BlackRook;
          board[7][2] = ChessSquare::Empty;
          board[7][3] = ChessSquare::Empty;
          board[7][4] = ChessSquare::BlackKing;
        },
        Castling::BlackShort => {
          board[7][7] = ChessSquare::BlackRook;
          board[7][6] = ChessSquare::BlackKing;
          board[7][5] = ChessSquare::BlackRook;
          board[7][4] = ChessSquare::BlackKing;
        },
      }
    },
    None => {
      match &chess_move.capture {
        Some(cap) => {
          if chess_move.en_pessant {
            board[chess_move.to_y][chess_move.to_x] = ChessSquare::Empty;
            board[chess_move.y][chess_move.to_x] = cap.clone();
          } else {
            board[chess_move.to_y][chess_move.to_x] = cap.clone();
          }
        },
        None => {
          board[chess_move.to_y][chess_move.to_x] = ChessSquare::Empty;
        }
      };
      board[chess_move.y][chess_move.x] = chess_move.piece.clone();
    }
  }
}

// color is the color of the king who may be in check. x and y are the coords of the king.
pub fn is_check(x: usize, y: usize, board: &[[ChessSquare; 8]; 8], color: ChessColor) -> bool {
  let pawn = if color == ChessColor::White { ChessSquare::BlackPawn } else { ChessSquare::WhitePawn };
  let rook = if color == ChessColor::White { ChessSquare::BlackRook } else { ChessSquare::WhiteRook };
  let bishop = if color == ChessColor::White { ChessSquare::BlackBishop } else { ChessSquare::WhiteBishop };
  let knight = if color == ChessColor::White { ChessSquare::BlackKnight } else { ChessSquare::WhiteKnight };
  let queen = if color == ChessColor::White { ChessSquare::BlackQueen } else { ChessSquare::WhiteQueen };
  let king = if color == ChessColor::White { ChessSquare::BlackKing } else { ChessSquare::WhiteKing };

  for d in get_rook_directions() {
    let mut check_x = x as i32 + d.0;
    let mut check_y = y as i32 + d.1;
    while check_x >= 0 && check_x <= 7 && check_y >= 0 && check_y <= 7 {
      let p = &board[check_y as usize][check_x as usize];
      if p == &ChessSquare::Empty {
        check_x += d.0;
        check_y += d.1;
      } else if p == &rook || p == &queen {
        return true;
      } else {
        break;
      }
    }
  }
  for d in get_bishop_directions() {
    let mut check_x = x as i32 + d.0;
    let mut check_y = y as i32 + d.1;
    while check_x >= 0 && check_x <= 7 && check_y >= 0 && check_y <= 7 {
      let p = &board[check_y as usize][check_x as usize];
      if p == &ChessSquare::Empty {
        check_x += d.0;
        check_y += d.1;
      } else if p == &bishop || p == &queen {
        return true;
      } else {
        break;
      }
    }
  }
  for m in get_knight_moves() {
    let check_x = x as i32 + m.0;
    let check_y = y as i32 + m.1;
    if check_x >= 0 && check_x <= 7 && check_y >= 0 && check_y <= 7 && board[check_y as usize][check_x as usize] == knight {
      return true;
    }
  }
  let pawn_directions = if color == ChessColor::White { [(-1, 1), (1, 1)] } else { [(-1, -1), (1, -1)] };
  for p in pawn_directions {
    let check_x = x as i32 + p.0;
    let check_y = y as i32 + p.1;
    if check_x >= 0 && check_x <= 7 && check_y >= 0 && check_y <= 7 && board[check_y as usize][check_x as usize] == pawn {
      return true;
    }
  }
  for d in get_queen_directions() {
    let check_x = x as i32 + d.0;
    let check_y = y as i32 + d.1;
    if check_x >= 0 && check_x <= 7 && check_y >= 0 && check_y <= 7 && board[check_y as usize][check_x as usize] == king {
      return true;
    }
  }
  return false;
}

// checks if given player is in check
pub fn get_check(board: &[[ChessSquare; 8]; 8], player: &ChessColor) -> Option<ChessColor> {
  for y in 0..8 {
    for x in 0..8 {
      match board[y][x] {
        ChessSquare::WhiteKing if player == &ChessColor::White && is_check(x, y, board, ChessColor::White) => {
          return Some(ChessColor::White);
        }
        ChessSquare::BlackKing if player == &ChessColor::Black && is_check(x, y, board, ChessColor::Black) => {
          return Some(ChessColor::Black);
        }
        _ => {}
      }
    }
  }
  return None
}

pub fn get_valid_moves(last_move: &Option<ChessMove>, player: &ChessColor, board: &[[ChessSquare; 8]; 8], available_castling: &HashSet<Castling>) -> Vec<ChessMove> {
  let mut testing_board = copy_board(board);
  return get_potential_moves(last_move, player, board, available_castling).into_iter().filter(|m| {
    if board[m.y][m.x] == m.piece && testing_board[m.y][m.x] != m.piece {
      let fen1 = ChessBoard{
        last_move: None,
        squares: copy_board(board),
        current_player: player.clone(),
        available_castling: HashSet::new(),
        move_number: 0
      }.get_forsyth_edwards_notation();
      let fen2 = ChessBoard{
        last_move: None,
        squares: copy_board(&testing_board),
        current_player: player.clone(),
        available_castling: HashSet::new(),
        move_number: 0
      }.get_forsyth_edwards_notation();
      print!("Some jank shit occurred {} {}!", fen1, fen2);
    }
    return check_move_validity(player, m, &mut testing_board);
  }).collect::<Vec<ChessMove>>();
}

pub fn check_move_validity(player: &ChessColor, chess_move: &ChessMove, testing_board: &mut [[ChessSquare; 8]; 8]) -> bool {
  return match chess_move.castling.as_ref() {
    Some(c) => check_castling_validity(player, c, testing_board),
    None => {
      do_move(chess_move, testing_board);
      let check = get_check(&testing_board, player);
      undo_move(chess_move, testing_board);
      return match check {
        Some(c) => &c != player,
        _ => true
      }
    }
  }
}

pub fn check_castling_validity(player: &ChessColor, castling: &Castling, testing_board: &mut [[ChessSquare; 8]; 8])-> bool {
  match castling {
    &Castling::WhiteLong => {
      let valid_position = testing_board[0][0] == ChessSquare::WhiteRook
        && testing_board[0][1] == ChessSquare::Empty
        && testing_board[0][2] == ChessSquare::Empty
        && testing_board[0][3] == ChessSquare::Empty
        && testing_board[0][4] == ChessSquare::WhiteKing;
      if !valid_position {
        return false;
      }
      for i in 0..3 {
        testing_board[0][4] = ChessSquare::Empty;
        testing_board[0][4 - i] = ChessSquare::WhiteKing;
        let check = get_check(&testing_board, player);
        let is_player_in_check = check.is_some() && check.as_ref().unwrap() == player;
        testing_board[0][4 - i] = ChessSquare::Empty;
        testing_board[0][4] = ChessSquare::WhiteKing;
        if is_player_in_check {
          return false;
        }
      }
      return true;
    },
    &Castling::WhiteShort => {
      let valid_position = testing_board[0][7] == ChessSquare::WhiteRook
        && testing_board[0][6] == ChessSquare::Empty
        && testing_board[0][5] == ChessSquare::Empty
        && testing_board[0][4] == ChessSquare::WhiteKing;
      if !valid_position {
        return false;
      }
      for i in 0..3 {
        testing_board[0][4] = ChessSquare::Empty;
        testing_board[0][4 + i] = ChessSquare::WhiteKing;
        let check = get_check(&testing_board, player);
        let is_player_in_check = check.is_some() && check.as_ref().unwrap() == player;
        testing_board[0][4 + i] = ChessSquare::Empty;
        testing_board[0][4] = ChessSquare::WhiteKing;
        if is_player_in_check {
          return false;
        }
      }
      return true;
    },
    &Castling::BlackLong => {
      let valid_position = testing_board[7][0] == ChessSquare::BlackRook
        && testing_board[7][1] == ChessSquare::Empty
        && testing_board[7][2] == ChessSquare::Empty
        && testing_board[7][3] == ChessSquare::Empty
        && testing_board[7][4] == ChessSquare::BlackKing;
      if !valid_position {
        return false;
      }
      for i in 0..3 {
        testing_board[7][4] = ChessSquare::Empty;
        testing_board[7][4 - i] = ChessSquare::BlackKing;
        let check = get_check(&testing_board, player);
        let is_player_in_check = check.is_some() && check.as_ref().unwrap() == player;
        testing_board[7][4 - i] = ChessSquare::Empty;
        testing_board[7][4] = ChessSquare::BlackKing;
        if is_player_in_check {
          return false;
        }
      }
      return true;
    },
    &Castling::BlackShort => {
      let valid_position = testing_board[7][7] == ChessSquare::BlackRook
        && testing_board[7][6] == ChessSquare::Empty
        && testing_board[7][5] == ChessSquare::Empty
        && testing_board[7][4] == ChessSquare::BlackKing;
      if !valid_position {
        return false;
      }
      for i in 0..3 {
        testing_board[7][4] = ChessSquare::Empty;
        testing_board[7][4 + i] = ChessSquare::BlackKing;
        let check = get_check(&testing_board, player);
        let is_player_in_check = check.is_some() && check.as_ref().unwrap() == player;
        testing_board[7][4 + i] = ChessSquare::Empty;
        testing_board[7][4] = ChessSquare::BlackKing;
        if is_player_in_check {
          return false;
        }
      }
      return true;
    },
  }
}