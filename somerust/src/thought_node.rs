use std::cmp::{Ordering};

use crate::{chess_board::ChessBoard, chess_color::ChessColor, chess_move::{ChessMove, get_check}, chess_scoring::score_game_state};


const BLACK_WIN_SCORE : f32 = -9999999.0;
const WHITE_WIN_SCORE : f32 = 9999999.0;

pub struct ThoughtNode {
  pub game_state: ChessBoard,
  pub children: Vec<ThoughtNode>,
  pub calculated_score: f32,
}

impl ThoughtNode {
  pub fn alphabeta(&mut self, depth: i32) -> f32 {
    return self.alpha_beta_pruning(depth, BLACK_WIN_SCORE, WHITE_WIN_SCORE);
  }

  fn alpha_beta_pruning(&mut self, depth: i32, mut alpha: f32, mut beta: f32) -> f32 {
    if depth == 0 {
      self.calculated_score = score_game_state(&self.game_state);
      return self.calculated_score;
    }
    if self.children.is_empty() {
      let valid_moves = self.game_state.get_valid_moves();
      if valid_moves.is_empty() {
        let check = get_check(&self.game_state.squares, &ChessColor::White).or(get_check(&self.game_state.squares, &ChessColor::Black));
        return match check {
          Some(c) => {
            match c {
              ChessColor::Black => WHITE_WIN_SCORE,
              ChessColor::White => BLACK_WIN_SCORE
            }
          }
          None => 0.0
        }
      }
      for chess_move in valid_moves.iter() {
        self.children.push(ThoughtNode{
          game_state: self.game_state.do_move(chess_move),
          children: Vec::new(),
          calculated_score: 0.0,
        });
      }
    }
    if self.game_state.current_player == ChessColor::White {
      self.calculated_score = BLACK_WIN_SCORE;
      for node in self.children.iter_mut() {
        self.calculated_score = f32::max(self.calculated_score, node.alpha_beta_pruning(depth - 1, alpha, beta));
        alpha = f32::max(alpha, self.calculated_score);
        if alpha > beta {
          break;
        }
      }
    } else {
      self.calculated_score = WHITE_WIN_SCORE;
      for node in self.children.iter_mut() {
        self.calculated_score = f32::min(self.calculated_score, node.alpha_beta_pruning(depth - 1, alpha, beta));
        beta = f32::min(beta, self.calculated_score);
        if beta < alpha {
          break;
        }
      }
    }
    return self.calculated_score;
  }

  pub fn get_best_move(&mut self) -> ChessMove {
    if self.children.is_empty() {
      panic!("haven't thought yet");
    }
    self.sort_children();
    return self.children.get(0).as_ref().unwrap().game_state.last_move.as_ref().unwrap().clone();
  }

  fn sort_children(&mut self) {
    match self.game_state.current_player {
      ChessColor::White => {
        self.children.sort_by(|c1, c2| -> Ordering { return c2.calculated_score.partial_cmp(&c1.calculated_score).unwrap_or(Ordering::Equal)});
      }
      ChessColor::Black => {
        self.children.sort_by(|c1, c2| -> Ordering { return c1.calculated_score.partial_cmp(&c2.calculated_score).unwrap_or(Ordering::Equal)});
      }
    };
  }
}