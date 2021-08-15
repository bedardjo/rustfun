use crate::chess_square::{ChessSquare};

pub struct ChessBoard {
  pub squares : [[ChessSquare; 8]; 8],
}

impl ChessBoard {
  pub fn copy_board(&self) -> [[ChessSquare; 8]; 8] {
    return copy_board(&self.squares);
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
    squares: create_starting_board_position()
  };
}