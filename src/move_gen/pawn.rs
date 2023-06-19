use crate::bitb;
use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::PlayerColor;
use std::collections::HashMap;
use std::collections::hash_map;
use super::{MovesMap, PieceAndMoves};
pub fn generate_moves(bishops: BitB64) -> MovesMap {
    let mut map = HashMap::new();
    map.insert(
        0,
        PieceAndMoves {
            typpe: crate::chess::PieceType::King,
            moves: bitb!(8) | bitb!(16),
        },
    );
    map
}
