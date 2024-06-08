use super::{bishop, rook};
use super::{
    bishop::BishopBitboardMoveGenerator, rook::RookBitboardMoveGenerator, BitboardMoveGenerator,
    MovesMap, PieceAndMoves,
};
use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::{self, Position};
use crate::chess::PieceType;

pub struct QueenBitboardMoveGenerator {}

impl BitboardMoveGenerator for QueenBitboardMoveGenerator {
    fn get_attacking_moves(pos: &Position) -> MovesMap {
        let mut result = bishop::get_attacking_moves_as(pos, PieceType::Queen);
        super::merge_moves_map(
            bishop::get_attacking_moves_as(pos, PieceType::Queen),
            &mut result,
        );
        result
    }

    fn generate_moves(pos: &Position) -> MovesMap {
        let mut result = MovesMap::new();
        super::merge_moves_map(
            bishop::generate_moves_as(pos, PieceType::Queen),
            &mut result,
        );
        super::merge_moves_map(rook::generate_moves_as(pos, PieceType::Queen), &mut result);
        result
    }
}
