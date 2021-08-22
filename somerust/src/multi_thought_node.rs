use std::{cmp::{Ordering}, thread};

use crate::{chess_board::ChessBoard, chess_color::ChessColor, chess_move::{ChessMove, get_check}, chess_scoring::score_game_state, thought_node::ThoughtNode};

pub struct MultiThoughtNode {
  pub game_state: ChessBoard,
  pub children: Vec<ThoughtNode>,
  pub calculated_score: f32,
  pub thought_threads: usize,
}

impl MultiThoughtNode {
  pub fn alphabeta(&mut self, depth: i32) -> ThoughtNode {
    let valid_moves = self.game_state.get_valid_moves();
    let chunk_size = valid_moves.len() / self.thought_threads as usize;
    let move_chunks : Vec<Vec<ChessMove>> = valid_moves.chunks(chunk_size).map(|x| x.to_vec()).collect();
    let mut thread_handles = Vec::new();
    for move_chunk in move_chunks {
      let curr_state = self.game_state.clone();
      thread_handles.push(thread::spawn(move || {
        let mut thought_nodes = Vec::new();
        for chess_move in move_chunk.iter() {
          let state = curr_state.do_move(chess_move);
          let mut thought_node = ThoughtNode{
            game_state: state,
            children: Vec::new(),
            calculated_score: 0.0
          };
          thought_node.alphabeta(depth);
          thought_nodes.push(thought_node);
        }
        return thought_nodes;
      }));
    }
    let mut children : Vec<ThoughtNode> = Vec::new(); 
    for th in thread_handles {
      children.extend(th.join().unwrap());
    }
    return ThoughtNode{
      game_state: self.game_state.clone(),
      children: children,
      calculated_score: 0.0
    };
  }
}