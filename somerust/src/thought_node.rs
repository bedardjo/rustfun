use std::cmp::{Ordering, max, min};

use crate::{chess_board::ChessBoard, chess_color::ChessColor, chess_move::ChessMove, chess_scoring::score_game_state};


const BLACK_WIN_SCORE : i32 = -9999999;
const WHITE_WIN_SCORE : i32 = 9999999;

pub struct ThoughtNode {
  pub game_state: ChessBoard,
  pub children: Vec<ThoughtNode>,
  pub calculated_score: i32
}

impl ThoughtNode {
  pub fn alphabeta(&mut self, depth: i32) -> i32 {
    return self.alpha_beta_pruning(depth, BLACK_WIN_SCORE, WHITE_WIN_SCORE);
  }

  fn alpha_beta_pruning(&mut self, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
      self.calculated_score = score_game_state(&self.game_state);
      return self.calculated_score;
    }
    if self.children.is_empty() {
      let valid_moves = self.game_state.get_valid_moves();
      for chess_move in valid_moves.iter() {
        self.children.push(ThoughtNode{
          game_state: self.game_state.do_move(chess_move),
          children: Vec::new(),
          calculated_score: 0
        });
      }
    }
    if self.game_state.current_player == ChessColor::White {
      self.calculated_score = BLACK_WIN_SCORE;
      for node in self.children.iter_mut() {
        self.calculated_score = max(self.calculated_score, node.alpha_beta_pruning(depth - 1, alpha, beta));
        alpha = max(alpha, self.calculated_score);
        if alpha > beta {
          break;
        }
      }
    } else {
      self.calculated_score = WHITE_WIN_SCORE;
      for node in self.children.iter_mut() {
        self.calculated_score = min(self.calculated_score, node.alpha_beta_pruning(depth - 1, alpha, beta));
        beta = min(beta, self.calculated_score);
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
        self.children.sort_by(|c1, c2| -> Ordering { return c2.calculated_score.cmp(&c1.calculated_score)});
      }
      ChessColor::Black => {
        self.children.sort_by(|c1, c2| -> Ordering { return c1.calculated_score.cmp(&c2.calculated_score)});
      }
    };
  }
}