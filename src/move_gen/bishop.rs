use super::{MovesMap, PieceAndMoves};
use crate::bitb;
use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::PlayerColor;
use std::collections::hash_map;
use std::collections::HashMap;

pub fn generate_moves(bishops: BitB64) -> MovesMap {
    let mut map = HashMap::new();
    map.insert(
        2,
        PieceAndMoves {
            typpe: crate::chess::PieceType::Bishop,
            moves: bitb!(10) | bitb!(18),
        },
    );
    map
}
