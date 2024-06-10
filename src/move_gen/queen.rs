use super::{bishop, rook, MoveGenOpts, MoveGenPerspective};
use super::{
    bishop::BishopBitboardMoveGenerator, rook::RookBitboardMoveGenerator, BitboardMoveGenerator,
    MovesMap, PieceAndMoves,
};

use crate::chess::bitboard::{BitArraySize, BitB64, BitboardMove, EMPTY_BOARD};
use crate::chess::position::{self, Position};
use crate::chess::{PieceType, PlayerColor};

pub struct QueenBitboardMoveGenerator {}

pub fn compute_queen_attacking_moves(pos: &Position, opts: MoveGenOpts) -> BitB64 {
    let (ally_pieces, enemy_pieces) = match opts.perspective {
        MoveGenPerspective::MovingPlayer => (pos.pieces_to_move(), pos.enemy_pieces()),
        MoveGenPerspective::WaitingPlayer => (pos.enemy_pieces(), pos.pieces_to_move()),
    };
    bishop::compute_raw_attacking_moves_as_bishop(ally_pieces, enemy_pieces, PieceType::Queen)
        | rook::compute_raw_attacking_moves_as_rook(ally_pieces, enemy_pieces, PieceType::Queen)
}

impl BitboardMoveGenerator for QueenBitboardMoveGenerator {
    fn get_attacking_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        let (ally_pieces, enemy_pieces) = match opts.perspective {
            MoveGenPerspective::MovingPlayer => (pos.pieces_to_move(), pos.enemy_pieces()),
            MoveGenPerspective::WaitingPlayer => (pos.enemy_pieces(), pos.pieces_to_move()),
        };
        let mut result =
            bishop::get_attacking_moves_as_bishop(ally_pieces, enemy_pieces, PieceType::Queen);
        super::merge_moves_map(
            rook::compute_attacking_moves_as_rook(ally_pieces, enemy_pieces, PieceType::Queen),
            &mut result,
        );
        result
    }

    fn generate_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        Self::get_attacking_moves(pos, opts)
    }
}
