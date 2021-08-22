use core::panic;
use std::collections::HashSet;

use crate::{castling::{Castling, all_castling}, chess_color::ChessColor, chess_move::{ChessMove, do_move, get_valid_moves}, chess_square::{ChessSquare}};

pub struct ChessBoard {
  pub last_move: Option<ChessMove>,
  pub squares : [[ChessSquare; 8]; 8],
  pub current_player: ChessColor,
  pub available_castling: HashSet<Castling>,
  pub move_number: u32,
}

impl ChessBoard {
  pub fn get_valid_moves(&self) -> Vec<ChessMove> {
    return get_valid_moves(&self.last_move, &self.current_player, &self.squares, &self.available_castling);
  }

  pub fn do_move(&self, chess_move: &ChessMove) -> ChessBoard {
    let mut new_board = copy_board(&self.squares);
    do_move(chess_move, &mut new_board);
    let mut new_castling = self.available_castling.clone();
    match chess_move.piece {
      ChessSquare::BlackKing => {
        new_castling.remove(&Castling::BlackLong);
        new_castling.remove(&Castling::BlackShort);
      }
      ChessSquare::WhiteKing => {
        new_castling.remove(&Castling::WhiteLong);
        new_castling.remove(&Castling::WhiteShort);
      }
      ChessSquare::BlackRook => {
        if chess_move.y == 7 && chess_move.x == 0 {
          new_castling.remove(&Castling::BlackLong);
        } else if chess_move.y == 7 && chess_move.x == 7 {
          new_castling.remove(&Castling::BlackShort);
        }
      }
      ChessSquare::WhiteRook => {
        if chess_move.y == 0 && chess_move.x == 0 {
          new_castling.remove(&Castling::WhiteLong);
        } else if chess_move.y == 0 && chess_move.x == 7 {
          new_castling.remove(&Castling::WhiteShort);
        }
      }
      _ => { }
    }
    match chess_move.capture.as_ref() {
      Some(c) => {
        match c {
          &ChessSquare::WhiteRook => {
            if chess_move.to_y == 0 && chess_move.to_x == 0 {
              new_castling.remove(&Castling::WhiteLong);
            } else if chess_move.to_y == 0 && chess_move.to_x == 7 {
              new_castling.remove(&Castling::WhiteShort);
            }
          },
          &ChessSquare::BlackRook => {
            if chess_move.to_y == 7 && chess_move.to_x == 0 {
              new_castling.remove(&Castling::BlackLong);
            } else if chess_move.to_y == 7 && chess_move.to_x == 7 {
              new_castling.remove(&Castling::BlackShort);
            }
          },
          _ => {}
        }
      }
      None => {}
    }
    return ChessBoard{
      last_move: Some(chess_move.clone()),
      squares: new_board,
      current_player: self.current_player.get_opposite(),
      available_castling: new_castling,
      move_number: self.move_number + 1
    }
  }

  pub fn get_forsyth_edwards_notation(&self) -> String {
    let mut fen = String::new();
    fen.push_str(&get_fen_board_part(&self.squares));

    fen.push_str(" ");
    if self.current_player == ChessColor::White {
      fen.push_str("w");
    } else {
      fen.push_str("b");
    }

    fen.push_str(" ");
    if self.available_castling.is_empty() {
      fen.push_str("-");
    } else {
      if self.available_castling.contains(&Castling::WhiteShort) {
        fen.push_str("K");
      }
      if self.available_castling.contains(&Castling::WhiteLong) {
        fen.push_str("Q");
      }
      if self.available_castling.contains(&Castling::BlackShort) {
        fen.push_str("k");
      }
      if self.available_castling.contains(&Castling::BlackLong) {
        fen.push_str("q");
      }
    }
    fen.push_str(" ");

    // TODO available en pessant
    fen.push_str("-");
    fen.push_str(" ");
    // half move clock?
    fen.push_str("0");
    fen.push_str(" ");

    fen.push_str(self.move_number.to_string().as_str());

    return fen;
  }

  pub fn clone(&self) -> ChessBoard {
    return ChessBoard{
      last_move: self.last_move.clone(),
      squares: copy_board(&self.squares),
      current_player: self.current_player.clone(),
      available_castling: self.available_castling.clone(),
      move_number: self.move_number
    }
  }
}

pub fn copy_board(board: &[[ChessSquare; 8]; 8]) -> [[ChessSquare; 8]; 8] {
  let mut copy = create_empty_board();
  for y in 0..8 {
    for x in 0..8 {
      if board[y][x] != ChessSquare::Empty {
        copy[y][x] = board[y][x].clone();
      }
    }
  }
  return copy;
}

fn create_starting_board_position() -> [[ChessSquare; 8]; 8] {
  return [
    [ChessSquare::WhiteRook, ChessSquare::WhiteKnight, ChessSquare::WhiteBishop, ChessSquare::WhiteQueen, ChessSquare::WhiteKing, ChessSquare::WhiteBishop, ChessSquare::WhiteKnight, ChessSquare::WhiteRook],
    [ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn],
    [ChessSquare::BlackRook, ChessSquare::BlackKnight, ChessSquare::BlackBishop, ChessSquare::BlackQueen, ChessSquare::BlackKing, ChessSquare::BlackBishop, ChessSquare::BlackKnight, ChessSquare::BlackRook],
  ];
}

fn create_empty_board() -> [[ChessSquare; 8]; 8] {
  return [
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
    [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
  ];
}


pub fn create_new_board() -> ChessBoard {
  return ChessBoard{
    last_move: None,
    squares: create_starting_board_position(),
    current_player: ChessColor::White,
    available_castling: all_castling(),
    move_number: 0
  };
}

pub fn from_forsyth_edwards_notation(fen: &String) -> ChessBoard {
  let mut squares = create_empty_board();

  let fen_parts : Vec<&str> = fen.split(' ').collect();

  let mut row = 0;
  let mut col = 0;
  for c in fen_parts.get(0).unwrap().chars() {
    match c {
      'r' => {
        squares[row][col] = ChessSquare::BlackRook;
        col += 1;
      },
      'b' => {
        squares[row][col] = ChessSquare::BlackBishop;
        col += 1;
      },
      'n' => {
        squares[row][col] = ChessSquare::BlackKnight;
        col += 1;
      },
      'q' => {
        squares[row][col] = ChessSquare::BlackQueen;
        col += 1;
      },
      'k' => {
        squares[row][col] = ChessSquare::BlackKing;
        col += 1;
      },
      'p' => {
        squares[row][col] = ChessSquare::BlackPawn;
        col += 1;
      },
      
      'R' => {
        squares[row][col] = ChessSquare::WhiteRook;
        col += 1;
      },
      'B' => {
        squares[row][col] = ChessSquare::WhiteBishop;
        col += 1;
      },
      'N' => {
        squares[row][col] = ChessSquare::WhiteKnight;
        col += 1;
      },
      'Q' => {
        squares[row][col] = ChessSquare::WhiteQueen;
        col += 1;
      },
      'K' => {
        squares[row][col] = ChessSquare::WhiteKing;
        col += 1;
      },
      'P' => {
        squares[row][col] = ChessSquare::WhitePawn;
        col += 1;
      },

      '/' => {
        row += 1;
        if col != 8 {
          panic!("wtf");
        }
        col = 0;
      },
      
      _ => {
        col += c.to_digit(10).unwrap() as usize;
      }
    }
  }

  print!("{}", fen_parts.get(1).unwrap());
  let current_player = if fen_parts[1] == "w" { ChessColor::White } else { ChessColor::Black };

  let mut castling = HashSet::new();
  for c in fen_parts[2].chars() {
    match c {
      'K' => {
        castling.insert(Castling::WhiteShort);
      },
      'Q' => {
        castling.insert(Castling::WhiteLong);
      },
      'k' => {
        castling.insert(Castling::BlackShort);
      },
      'q' => {
        castling.insert(Castling::BlackLong);
      },
      '-' => {}
      _ => panic!("invalid castle char")
    }
  }

  return ChessBoard{
    last_move: None,
    squares: squares,
    current_player: current_player,
    available_castling: castling,
    move_number: fen_parts[5].parse::<u32>().unwrap()
  };
}

pub fn get_fen_board_part(board: &[[ChessSquare; 8]; 8]) -> String {
  let mut fen = String::new();
  for y in 0..8 {
    let mut empty_squares = 0;
    for x in 0..8 {
      match board[7 - y][x] {
        ChessSquare::Empty => {
          empty_squares += 1;
        },
        _ => {
          if empty_squares > 0 {
            fen.push_str(empty_squares.to_string().as_str());
          }
          fen.push_str(board[7 - y][x].get_char().as_str());
          empty_squares = 0;
        }
      }
    }
    if empty_squares > 0 {
      fen.push_str(empty_squares.to_string().as_str());
    }
    if y < 7 {
      fen.push_str("/");
    }
  }
  return fen;
}