use super::{bishop, rook};
use super::{
    bishop::BishopBitboardMoveGenerator, rook::RookBitboardMoveGenerator, BitboardMoveGenerator,
    MovesMap, PieceAndMoves,
};

use crate::chess::bitboard::{BitArraySize, BitB64, BitboardMove, EMPTY_BOARD};
use crate::chess::position::{self, Position};
use crate::chess::PieceType;

pub struct QueenBitboardMoveGenerator {}

pub fn compute_single_queen_attacking_moves(pos: &Position, id: u8) -> BitB64 {
    rook::compute_single_rook_attacking_moves(pos, id)
        | bishop::compute_single_bishop_attacking_moves(pos, id)
}

pub fn compute_queen_attacking_moves(pos: &Position) -> BitB64 {
    let mut result = EMPTY_BOARD;
    let mut piece_set = pos.pieces_to_move().queens;
    while piece_set != EMPTY_BOARD {
        let id = piece_set.trailing_zeros() as u8;
        result |= compute_single_queen_attacking_moves(pos, id);
        piece_set ^= u64::nth(id);
    }
    result
}

impl BitboardMoveGenerator for QueenBitboardMoveGenerator {
    fn get_attacking_moves(pos: &Position) -> MovesMap {
        let mut result = bishop::get_attacking_moves_as_bishop(pos, PieceType::Queen);
        super::merge_moves_map(
            rook::get_attacking_moves_as_rook(pos, PieceType::Queen),
            &mut result,
        );
        result
    }

    fn generate_moves(pos: &Position) -> MovesMap {
        let mut result = MovesMap::new();
        super::merge_moves_map(
            bishop::generate_moves_as_bishop(pos, PieceType::Queen),
            &mut result,
        );
        super::merge_moves_map(
            rook::generate_moves_as_rook(pos, PieceType::Queen),
            &mut result,
        );
        result
    }
}
