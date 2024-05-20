use super::{bishop, rook};
use super::{
    bishop::BishopBitboardMoveGenerator, rook::RookBitboardMoveGenerator, BitboardMoveGenerator,
    MovesMap, PieceAndMoves,
};
use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::{self, Position};

pub struct QueenBitboardMoveGenerator {}

impl BitboardMoveGenerator for QueenBitboardMoveGenerator {
    fn generate_moves(pos: &Position) -> MovesMap {
        let mut result = MovesMap::new();
        let queen_set = pos.pieces_to_move().queens;
        super::merge_moves_map(bishop::generate_moves_as(pos, queen_set), &mut result);
        super::merge_moves_map(rook::generate_moves_as(pos, queen_set), &mut result);
        result
    }
}
