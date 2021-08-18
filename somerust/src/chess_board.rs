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
    // TODO figure out castling etc.
    return ChessBoard{
      last_move: Some(chess_move.clone()),
      squares: new_board,
      current_player: self.current_player.get_opposite(),
      available_castling: self.available_castling.clone(),
      move_number: self.move_number + 1
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

pub fn create_starting_board_position() -> [[ChessSquare; 8]; 8] {
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

pub fn create_empty_board() -> [[ChessSquare; 8]; 8] {
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