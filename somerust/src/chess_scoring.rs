use crate::{chess_board::ChessBoard, chess_color::ChessColor, chess_square::ChessSquare};

fn get_piece_value(p: &ChessSquare) -> f32 {
  match p {
    ChessSquare::Empty => 0.0,
  
    ChessSquare::WhitePawn => 1.0,
    ChessSquare::WhiteRook=> 5.0,
    ChessSquare::WhiteKnight=> 3.0,
    ChessSquare::WhiteBishop=> 3.0,
    ChessSquare::WhiteQueen=> 9.0,
    ChessSquare::WhiteKing=> 1000.0,
  
    ChessSquare::BlackPawn=> -1.0,
    ChessSquare::BlackRook=> -5.0,
    ChessSquare::BlackKnight=> -3.0,
    ChessSquare::BlackBishop=> -3.0,
    ChessSquare::BlackQueen=> -9.0,
    ChessSquare::BlackKing=> -1000.0,
  }
}

pub fn score_game_state(board: &ChessBoard) -> f32 {
  let mut score = 0.0;
  for y in 0..8 {
    for x in 0..8 {
      score += get_piece_value(&board.squares[y][x]);
    }
  }
  let valid_moves = board.get_valid_moves();
  for m in valid_moves.iter() {
    score += match m.piece.get_color() {
      ChessColor::White => 0.05,
      ChessColor::Black => -0.05,
    };
  }
  return score;
}