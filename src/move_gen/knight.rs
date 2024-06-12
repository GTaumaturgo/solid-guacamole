use super::{internal, MoveGenOpts};
use super::{internal::bounded, BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::bitboard::PlayerBitboard;
use crate::chess::{bitboard, PlayerColor};
use crate::chess::{
    bitboard::BitArraySize,
    position::{self, Position},
};
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD},
    PieceType,
};
use crate::move_gen::MoveGenPerspective;

use std::collections::HashMap;

pub fn compute_single_knight_attacking_moves(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    id: u8,
) -> BitB64 {
    let mut result = EMPTY_BOARD;
    let (i0, j0) = internal::get_ij_from_sq_id(id as i8);
    // Try all possible distances (1..7 in all four diagonals):
    for i in [-2i8, -1i8, 1i8, 2i8].iter() {
        for j in [-2i8, -1i8, 1i8, 2i8].iter() {
            if i * i + j * j != 5 {
                continue;
            }
            let (i_pos, j_pos) = (i0 + i, j0 + j);
            if !bounded(i_pos, 0, 7) || !bounded(j_pos, 0, 7) {
                continue;
            }
            let to = (i_pos * 8 + j_pos) as u8;
            let to_sq = u64::nth(to);
            result |= to_sq;
        }
    }
    result &= u64::compl(ally_pieces.all_pieces());
    result
}

pub fn compute_raw_knight_attacking_moves_internal(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
) -> BitB64 {
    let mut result = EMPTY_BOARD;
    let mut piece_set = ally_pieces.knights;
    while piece_set != EMPTY_BOARD {
        let id = piece_set.trailing_zeros() as u8;
        result |= compute_single_knight_attacking_moves(ally_pieces, enemy_pieces, id);
        piece_set ^= u64::nth(id);
    }
    result
}

fn get_attacking_moves_internal(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    opts: MoveGenOpts,
) -> MovesMap {
    let mut result = HashMap::new();
    let mut piece_set = ally_pieces.knights;
    while piece_set != EMPTY_BOARD {
        let id: u8 = piece_set.trailing_zeros() as u8;
        let cur_knight = u64::nth(id as u8);
        piece_set ^= cur_knight;
        let resulting_moves = internal::bitb64_to_moves_list(
            id,
            compute_single_knight_attacking_moves(ally_pieces, enemy_pieces, id as u8),
        );
        if resulting_moves.len() > 0 {
            result.insert(
                id as u8,
                PieceAndMoves {
                    typpe: PieceType::Knight,
                    moves: resulting_moves,
                },
            );
        }
    }
    result
    // Can't move to squares that contain our pieces.
}

pub struct KnightBitboardMoveGenerator {}
impl BitboardMoveGenerator for KnightBitboardMoveGenerator {
    fn get_raw_attacking_moves(pos: &Position, opts: MoveGenOpts) -> BitB64 {
        let (ally_pieces, enemy_pieces) = match opts.perspective {
            MoveGenPerspective::MovingPlayer => (pos.pieces_to_move(), pos.enemy_pieces()),
            MoveGenPerspective::WaitingPlayer => (pos.enemy_pieces(), pos.pieces_to_move()),
        };
        compute_raw_knight_attacking_moves_internal(ally_pieces, enemy_pieces)
    }
    fn get_attacking_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        let (ally_pieces, enemy_pieces) = match opts.perspective {
            MoveGenPerspective::MovingPlayer => (pos.pieces_to_move(), pos.enemy_pieces()),
            MoveGenPerspective::WaitingPlayer => (pos.enemy_pieces(), pos.pieces_to_move()),
        };
        get_attacking_moves_internal(ally_pieces, enemy_pieces, opts)
    }

    fn generate_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        Self::get_attacking_moves(pos, opts)
    }
}
