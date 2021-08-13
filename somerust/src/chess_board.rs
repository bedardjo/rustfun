use crate::chess_square::{ChessSquare};

pub struct ChessBoard {
  pub squares : [[ChessSquare; 8]; 8],
}

pub fn create_new_board() -> ChessBoard {
  return ChessBoard{
    squares: [
      [ChessSquare::BlackRook, ChessSquare::BlackKnight, ChessSquare::BlackBishop, ChessSquare::BlackQueen, ChessSquare::BlackKing, ChessSquare::BlackBishop, ChessSquare::BlackKnight, ChessSquare::BlackRook],
      [ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn, ChessSquare::BlackPawn],
      [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
      [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
      [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
      [ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty, ChessSquare::Empty],
      [ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn, ChessSquare::WhitePawn],
      [ChessSquare::WhiteRook, ChessSquare::WhiteKnight, ChessSquare::WhiteBishop, ChessSquare::WhiteQueen, ChessSquare::WhiteKing, ChessSquare::WhiteBishop, ChessSquare::WhiteKnight, ChessSquare::WhiteRook],
    ]
  };
}