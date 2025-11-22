// --- Standard Library ---
use std::collections::HashMap;
use std::time::{Duration, Instant};

// --- External Crates ---
// use async_recursion; // No longer needed
// use async_trait::async_trait; // May not be needed if your evaluator traits are sync
use lazy_static::__Deref;
use rocket::futures::sink::Send;

// --- Internal `crate` Imports ---
use crate::{
    chess::{
        bitboard::{BitArraySize, BitB64, BitboardMove, PlayerBitboard, SpecialMoveType},
        position::Position,
        position_cache::{CacheEntry, CachedScoreInfo, CachedScoreType},
        ChessPiece, PieceType, PlayerColor,
    },
    move_gen::{MoveGenPerspective, MovesMap},
    CACHE,
};

// --- `super` (Parent Module) Imports ---
use super::{PositionEvaluationPipeline, PositionEvaluator};
pub struct MinimaxSearchEvaluator {
    leaf_evaluator: PositionEvaluationPipeline,
    depth: u8,
}

impl MinimaxSearchEvaluator {
    pub fn new(leaf_evaluator: PositionEvaluationPipeline, depth: u8) -> Self {
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
        let hash = *position.hash();
        let original_alpha = alpha;

        // --- Transposition Table Probe (Cache Read) ---
        if let Some(entry) = CACHE.get(&hash) {
            if let Some(score_info) = entry.get_score(&CachedScoreType::Exact) {
                if score_info.depth >= remaining_depth {
                    return (score_info.score, 1);
                }
            }
            if let Some(score_info) = entry.get_score(&CachedScoreType::LowerBound) {
                if score_info.depth >= remaining_depth {
                    alpha = alpha.max(score_info.score);
                }
            }
            if let Some(score_info) = entry.get_score(&CachedScoreType::UpperBound) {
                if score_info.depth >= remaining_depth {
                    beta = beta.min(score_info.score);
                }
            }
            if alpha >= beta {
                return (alpha, 1);
            }
        }

        // --- 2. Base Case: Reached Leaf Node (Depth = 0) ---
        if remaining_depth == 0 {
            let evaluation = self.leaf_evaluator.evaluate(position);

            let mut entry = CACHE.entry(hash).or_default();
            entry.try_update_score(CachedScoreType::Exact, evaluation, 0);
            return (evaluation, 1);
        }

        // --- 3. Recursive Step: Generate and Search Moves ---
        let moving_player = position.player_to_move();
        let mut best_score = match moving_player {
            PlayerColor::White => i32::MIN,
            PlayerColor::Black => i32::MAX,
        };
        let mut total_nodes_explored = 0;

        // --- 3a. REMOVED `.await` ---
        let continuation_map = position.legal_continuations();

        // --- 3b. Handle Checkmate/Stalemate ---
        if continuation_map.is_empty() {
            // --- 3c. REMOVED `.await` ---
            let score = if position.can_king_be_captured(MoveGenPerspective::MovingPlayer) {
                // CHECKMATE
                match moving_player {
                    PlayerColor::White => i32::MIN + 1,
                    PlayerColor::Black => i32::MAX - 1,
                }
            } else {
                // STALEMATE
                0
            };

            let mut entry = CACHE.entry(hash).or_default();
            entry.try_update_score(CachedScoreType::Exact, score, remaining_depth);
            return (score, 1);
        }

        // --- 3d. Iterate Through Moves ---
        for (_from_id, piece_and_moves) in continuation_map.iter() {
            for mv in piece_and_moves.moves.iter() {
                let new_pos = position.make_move(
                    mv,
                    ChessPiece {
                        typpe: piece_and_moves.typpe,
                        color: position.player_to_move(),
                    },
                );

                // --- 3e. REMOVED `.await` from recursive call ---
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
            if alpha >= beta {
                break;
            }
        }

        // --- 4. Transposition Table Store (Cache Write) ---
        let score_type = if best_score <= original_alpha {
            CachedScoreType::UpperBound
        } else if best_score >= beta {
            CachedScoreType::LowerBound
        } else {
            CachedScoreType::Exact
        };

        let mut entry = CACHE.entry(hash).or_default();
        entry.try_update_score(score_type, best_score, remaining_depth);

        (best_score, total_nodes_explored)
    }
}

impl MinimaxSearchEvaluator {
    // --- 5. REMOVED `async` ---
    pub fn evaluate(&self, position: &Position) -> i32 {
        let start = Instant::now();

        // --- 6. REMOVED `.await` ---
        let (score, nodes_explored) = self.minimax(position, self.depth, i32::MIN, i32::MAX);

        println!("Nodes explored: {}", nodes_explored);
        println!("Score: {}", score);
        let duration = start.elapsed();
        println!("Duration: {:?}", duration);

        if nodes_explored > 0 {
            println!(
                "microsseconds per node: {}",
                duration.as_micros() / nodes_explored as u128
            );
        }

        score
    }
}
