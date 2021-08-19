use crate::chess_color::ChessColor;

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

impl ChessSquare {
  pub fn to_string(&self)->String {
    return match self {
      Self::Empty => String::from("Empty"),
  
      Self::WhitePawn => String::from("WhitePawn"),
      Self::WhiteRook=> String::from("WhiteRook"),
      Self::WhiteKnight=> String::from("WhiteKnight"),
      Self::WhiteBishop=> String::from("WhiteBishop"),
      Self::WhiteQueen=> String::from("WhiteQueen"),
      Self::WhiteKing=> String::from("WhiteKing"),
      
      Self::BlackPawn=> String::from("BlackPawn"),
      Self::BlackRook=> String::from("BlackRook"),
      Self::BlackKnight=> String::from("BlackKnight"),
      Self::BlackBishop=> String::from("BlackBishop"),
      Self::BlackQueen=> String::from("BlackQueen"),
      Self::BlackKing=> String::from("BlackKing"),
    }
  }
  pub fn get_char(&self)->String {
    return match self {
      Self::Empty => panic!("at the disco"),
  
      Self::WhitePawn => String::from("P"),
      Self::WhiteRook=> String::from("R"),
      Self::WhiteKnight=> String::from("N"),
      Self::WhiteBishop=> String::from("B"),
      Self::WhiteQueen=> String::from("Q"),
      Self::WhiteKing=> String::from("K"),
      
      Self::BlackPawn=> String::from("p"),
      Self::BlackRook=> String::from("r"),
      Self::BlackKnight=> String::from("n"),
      Self::BlackBishop=> String::from("b"),
      Self::BlackQueen=> String::from("q"),
      Self::BlackKing=> String::from("k"),
    }
  }
  pub fn get_color(&self)->ChessColor {
    return match self {
      Self::Empty => { panic!("empty has no color") },
  
      Self::WhitePawn => ChessColor::White,
      Self::WhiteRook=> ChessColor::White,
      Self::WhiteKnight=> ChessColor::White,
      Self::WhiteBishop=> ChessColor::White,
      Self::WhiteQueen=> ChessColor::White,
      Self::WhiteKing=> ChessColor::White,
      
      Self::BlackPawn=> ChessColor::Black,
      Self::BlackRook=> ChessColor::Black,
      Self::BlackKnight=> ChessColor::Black,
      Self::BlackBishop=> ChessColor::Black,
      Self::BlackQueen=> ChessColor::Black,
      Self::BlackKing=> ChessColor::Black,
    }
  }
}