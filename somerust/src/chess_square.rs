#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ChessSquare {
  Empty,

  WhitePawn,
  WhiteRook,
  WhiteKnight,
  WhiteBishop,
  WhiteQueen,
  WhiteKing,
  
  BlackPawn,
  BlackRook,
  BlackKnight,
  BlackBishop,
  BlackQueen,
  BlackKing,
}