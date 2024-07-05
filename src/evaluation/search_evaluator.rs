use crate::chess::bitboard::BitB64;
use crate::chess::position;
use crate::chess::position::Position;
use crate::chess::ChessPiece;
use crate::chess::PieceType;
use crate::chess::PlayerColor;

use crate::move_gen::MovesMap;

use super::PositionEvaluator;
use crate::chess::bitboard::{BitArraySize, BitboardMove, PlayerBitboard, SpecialMoveType};
use std::time::{Duration, Instant};

pub struct MinimaxSearchEvaluator {
    leaf_evaluator: Box<dyn PositionEvaluator>,
    depth: u8,
}

impl MinimaxSearchEvaluator {
    pub fn new(leaf_evaluator: Box<dyn PositionEvaluator>, depth: u8) -> Self {
        Self {
            leaf_evaluator,
            depth,
        }
    }

    fn minimax(
        &self,
        position: &Position,
        remaining_depth: u8,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, i32) {
        if remaining_depth == 0 {
            return (self.leaf_evaluator.evaluate(position), 1);
        }
        let moving_player = position.player_to_move();
        let mut best_score = match moving_player {
            PlayerColor::White => i32::MIN,
            PlayerColor::Black => i32::MAX,
        };
        let mut total_nodes_explored = 0;
        let continuation_map = position.legal_continuations();
        for (_from_id, piece_and_moves) in continuation_map.iter() {
            for mv in piece_and_moves.moves.iter() {
                let new_pos = position.make_move(
                    mv,
                    ChessPiece {
                        typpe: piece_and_moves.typpe,
                        color: position.player_to_move(),
                    },
                );
                let (score, nodes_explored) =
                    self.minimax(&new_pos, remaining_depth - 1, alpha, beta);
                total_nodes_explored += nodes_explored;
                match moving_player {
                    PlayerColor::White => {
                        best_score = best_score.max(score);
                        alpha = alpha.max(best_score);
                    }
                    PlayerColor::Black => {
                        best_score = best_score.min(score);
                        beta = beta.min(best_score);
                    }
                }
                if alpha >= beta {
                    break;
                }
            }
        }
        (best_score, total_nodes_explored)
    }
}

pub struct MoveScore {
    pub score: i32,
    pub depth: u8,
    // pub confidence: u16,
}

impl PositionEvaluator for MinimaxSearchEvaluator {
    fn evaluate(&self, position: &Position) -> i32 {
        let start = Instant::now();
        let (score, nodes_explored) = self.minimax(position, self.depth, i32::MIN, i32::MAX);
        println!("Nodes explored: {}", nodes_explored);
        println!("Score: {}", score);
        let duration = start.elapsed();
        println!("Duration: {:?}", duration);
        println!(
            "microsseconds per node: {}",
            duration.as_micros() / nodes_explored as u128
        );
        score
    }
}
