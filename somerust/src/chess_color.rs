#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ChessColor {
  White, Black
}


impl ChessColor {
  pub fn get_opposite(&self) -> ChessColor {
    return match self {
      Self::White => ChessColor::Black,
      Self::Black => ChessColor::White,
    }
  }
}