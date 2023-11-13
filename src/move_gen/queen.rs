use crate::bitb;
use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::PlayerColor;
use std::collections::HashMap;
use std::collections::hash_map;
use super::{MovesMap, PieceAndMoves,bishop,rook};


pub fn generate_moves(pos: &Position, mut queen_set: BitB64) -> MovesMap {
    let mut result = MovesMap::new();   
    super::merge_moves_map(bishop::generate_moves(pos, queen_set), &mut result);
    super::merge_moves_map(rook::generate_moves(pos, queen_set), &mut result);
    result
}
