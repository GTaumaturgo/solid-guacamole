use crate::chess::bitboard::BitB64;
use crate::chess::position;
use crate::chess::position::Position;
use crate::chess::position_cache::CacheEntry;
use crate::chess::ChessPiece;
use crate::chess::PieceType;
use crate::chess::PlayerColor;

use crate::move_gen::MovesMap;

use super::PositionEvaluationPipeline;
use super::PositionEvaluator;
use crate::chess::bitboard::{BitArraySize, BitboardMove, PlayerBitboard, SpecialMoveType};
use async_trait::async_trait;

use crate::CACHE;
use lazy_static::__Deref;
use rocket::futures::sink::Send;

use async_recursion::async_recursion;
use std::collections::HashMap;
use std::time::{Duration, Instant};

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

    #[async_recursion]
    async fn minimax(
        &self,
        position: &Position,
        remaining_depth: u8,
        mut alpha: i32,
        mut beta: i32,
    ) -> (i32, i32) {
        if remaining_depth == 0 {
            if let Some(entry) = CACHE.get(position.hash()) {
                if let Some(score) = (*entry).scores.get(&0) {
                    // println!("leeaf cache hit");
                    return (*score, 1);
                }
            }
            let evaluation = self.leaf_evaluator.evaluate(position).await;
            if !CACHE.contains_key(position.hash()) {
                CACHE.insert(
                    *position.hash(),
                    CacheEntry {
                        scores: HashMap::new(),
                        raw_attacked_squares: HashMap::new(),
                        legal_continuations: None,
                    },
                );
            };
            CACHE
                .get_mut(position.hash())
                .unwrap()
                .scores
                .insert(0, evaluation);
            return (evaluation, 1);
        }
        if let Some(entry) = CACHE.get(position.hash()) {
            if let Some(score) = (*entry).scores.get(&(remaining_depth as usize)) {
                // println!("non-leeaf cache hit");
                return (*score, 1);
            }
        }
        let moving_player = position.player_to_move();
        let mut best_score = match moving_player {
            PlayerColor::White => i32::MIN,
            PlayerColor::Black => i32::MAX,
        };
        let mut total_nodes_explored = 0;
        let continuation_map = position.legal_continuations().await;
        for (_from_id, piece_and_moves) in continuation_map.iter() {
            for mv in piece_and_moves.moves.iter() {
                let new_pos = position.make_move(
                    mv,
                    ChessPiece {
                        typpe: piece_and_moves.typpe,
                        color: position.player_to_move(),
                    },
                );
                let (score, nodes_explored) = self
                    .minimax(&new_pos, remaining_depth - 1, alpha, beta)
                    .await;
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
        if !CACHE.contains_key(position.hash()) {
            CACHE.insert(
                *position.hash(),
                CacheEntry {
                    scores: HashMap::new(),
                    raw_attacked_squares: HashMap::new(),
                    legal_continuations: None,
                },
            );
        };
        CACHE
            .get_mut(position.hash())
            .unwrap()
            .scores
            .insert(remaining_depth as usize, best_score);
        (best_score, total_nodes_explored)
    }
}

pub struct MoveScore {
    pub score: i32,
    pub depth: u8,
    // pub confidence: u16,
}

impl MinimaxSearchEvaluator {
    pub async fn evaluate(&self, position: &Position) -> i32 {
        let start = Instant::now();
        // let test = (self.minimax(position, self.depth, i32::MIN, i32::MAX)).await;
        let (score, nodes_explored) = self.minimax(position, self.depth, i32::MIN, i32::MAX).await;
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
