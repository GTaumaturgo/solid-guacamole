use crate::chess::bitboard::BitB64;
use crate::chess::position::Position;
use crate::chess::PieceType;
use crate::chess::PlayerColor;

use crate::strum::IntoEnumIterator;

use super::PositionEvaluator;
use crate::chess::bitboard::{BitArraySize, PlayerBitboard, SpecialMoveType};
use crate::move_gen::MoveGenPerspective;
use async_trait::async_trait;
use std::collections::HashMap;

pub struct CheckmateEvaluator {}

impl CheckmateEvaluator {
    pub async fn evaluate(&self, position: &Position) -> i32 {
        if !position.legal_continuations().await.is_empty() {
            let moving_player = position.player_to_move();
            if position
                .can_king_be_captured(MoveGenPerspective::MovingPlayer)
                .await
            {
                1000000
            } else {
                0 // Stalemate.
            }
        } else if position
            .can_king_be_captured(MoveGenPerspective::MovingPlayer)
            .await
        {
            // Small demotion if king is in check.
            -60
        } else {
            0
        }
    }
}
