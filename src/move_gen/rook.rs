use super::internal::{self, bitb64_to_moves_list};
use super::internal::{get_ij_from_sq_id, try_generate_move_in_direction};
use super::{BitboardMoveGenerator, MoveGenOpts, MoveGenPerspective, MovesMap, PieceAndMoves};
use crate::chess::bitboard;
use crate::chess::bitboard::{BitArraySize, BitB64, BitboardMove, PlayerBitboard, EMPTY_BOARD};
use crate::chess::position::Position;
use crate::chess::{PieceType, PlayerColor};
use std::collections::hash_map;
use std::collections::HashMap;

pub struct RookBitboardMoveGenerator {}

pub fn compute_single_rook_attacking_moves(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    id: u8,
) -> BitB64 {
    let (i0, j0) = get_ij_from_sq_id(id as i8);
    let mut cur_rook_moves = EMPTY_BOARD;
    let mut dir_blockedness = vec![
        false, // up
        false, // down
        false, // right
        false, // left
    ];
    // Try all possible distances (1..7 in all four diagonals):
    for i in 1..7 {
        // up, down, right, left
        let all_dir_ids = vec![(i0 + i, j0), (i0 - i, j0), (i0, j0 + i), (i0, j0 - i)];
        let mut should_keep_trying = false;
        for (dir_sq_id, mut dir_blocked) in all_dir_ids.iter().zip(dir_blockedness.iter_mut()) {
            try_generate_move_in_direction(
                *dir_sq_id,
                ally_pieces,
                enemy_pieces,
                &mut dir_blocked,
                &mut cur_rook_moves,
            );
            should_keep_trying |= !*dir_blocked;
        }
        if !should_keep_trying {
            break;
        }
    }
    cur_rook_moves
}

pub fn compute_raw_attacking_moves_as_rook(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    real_type: PieceType,
) -> BitB64 {
    let mut result = EMPTY_BOARD;
    let mut piece_set = *ally_pieces.pieces(real_type);
    while piece_set != EMPTY_BOARD {
        let id = piece_set.trailing_zeros() as u8;
        result |= compute_single_rook_attacking_moves(ally_pieces, enemy_pieces, id);
        piece_set ^= u64::nth(id);
    }
    result
}

pub fn compute_attacking_moves_as_rook(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    real_type: PieceType,
) -> MovesMap {
    let mut result = HashMap::new();
    let mut piece_set = *ally_pieces.pieces(real_type);
    while piece_set != EMPTY_BOARD {
        let id = piece_set.trailing_zeros() as u8;
        let moves = compute_single_rook_attacking_moves(ally_pieces, enemy_pieces, id);
        result.insert(
            id,
            PieceAndMoves {
                typpe: real_type,
                moves: bitb64_to_moves_list(id, moves),
            },
        );
        piece_set ^= u64::nth(id);
    }
    result
}

impl BitboardMoveGenerator for RookBitboardMoveGenerator {
    fn get_raw_attacking_moves(pos: &Position, opts: MoveGenOpts) -> BitB64 {
        let (ally_pieces, enemy_pieces) = match opts.perspective {
            MoveGenPerspective::MovingPlayer => (pos.pieces_to_move(), pos.enemy_pieces()),
            MoveGenPerspective::WaitingPlayer => (pos.enemy_pieces(), pos.pieces_to_move()),
        };
        compute_raw_attacking_moves_as_rook(ally_pieces, enemy_pieces, PieceType::Rook)
    }

    fn get_attacking_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        compute_attacking_moves_as_rook(pos.pieces_to_move(), pos.enemy_pieces(), PieceType::Rook)
    }

    fn generate_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        Self::get_attacking_moves(pos, opts)
    }
}
