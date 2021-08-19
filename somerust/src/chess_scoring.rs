use std::collections::HashMap;

use crate::{chess_board::ChessBoard, chess_square::ChessSquare};

fn get_piece_value(p: &ChessSquare) -> i32 {
  match p {
    ChessSquare::Empty => 0,
  
    ChessSquare::WhitePawn => 1,
    ChessSquare::WhiteRook=> 5,
    ChessSquare::WhiteKnight=> 3,
    ChessSquare::WhiteBishop=> 3,
    ChessSquare::WhiteQueen=> 9,
    ChessSquare::WhiteKing=> 1000,
  
    ChessSquare::BlackPawn=> -1,
    ChessSquare::BlackRook=> -5,
    ChessSquare::BlackKnight=> -3,
    ChessSquare::BlackBishop=> -3,
    ChessSquare::BlackQueen=> -9,
    ChessSquare::BlackKing=> -1000,
  }
}

pub fn score_game_state(board: &ChessBoard) -> i32 {
  let mut score = 0;
  for y in 0..8 {
    for x in 0..8 {
      score += get_piece_value(&board.squares[y][x]);
    }
  }
  return score;
}