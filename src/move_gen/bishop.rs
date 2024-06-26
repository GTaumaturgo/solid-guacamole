use super::internal::{
    get_ij_from_sq_id, intersect, is_inside_board, try_generate_move_in_direction,
};
use super::{internal, MoveGenOpts, MoveGenPerspective};
use super::{BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::bitboard::{self, BitArraySize, PlayerBitboard};
use crate::chess::position::Position;
use crate::chess::PlayerColor;
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD},
    PieceType,
};

use std::collections::HashMap;

pub fn compute_single_bishop_attacking_moves(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    id: u8,
) -> BitB64 {
    let mut cur_bishop_moves = EMPTY_BOARD;
    let mut dir_blockedness = vec![
        false, // upleft
        false, // upright
        false, // downleft
        false, // downright
    ];

    let (i0, j0) = get_ij_from_sq_id(id as i8);
    // Try all possible distances (1..7 in all four diagonals):
    for i in 1..8 {
        // upleft, upright, downleft, downright
        let all_dir_ids = vec![
            (i0 + i, j0 - i),
            (i0 + i, j0 + i),
            (i0 - i, j0 - i),
            (i0 - i, j0 + i),
        ];
        let mut should_keep_trying = false;
        for (dir_sq_id, mut dir_blocked) in all_dir_ids.iter().zip(dir_blockedness.iter_mut()) {
            try_generate_move_in_direction(
                *dir_sq_id,
                ally_pieces,
                enemy_pieces,
                &mut dir_blocked,
                &mut cur_bishop_moves,
            );
            should_keep_trying |= !*dir_blocked;
        }
        if !should_keep_trying {
            break;
        }
    }
    cur_bishop_moves
}

pub fn compute_raw_attacking_moves_as_bishop_internal(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    real_type: PieceType,
) -> BitB64 {
    let mut result = EMPTY_BOARD;
    let mut piece_set = *ally_pieces.pieces(real_type);
    while piece_set != EMPTY_BOARD {
        let id = piece_set.trailing_zeros() as u8;
        result |= compute_single_bishop_attacking_moves(ally_pieces, enemy_pieces, id);
        piece_set ^= u64::nth(id);
    }
    result
}

pub fn get_attacking_moves_as_bishop_internal(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    real_type: PieceType,
) -> MovesMap {
    let mut result = HashMap::new();
    let mut piece_set = *ally_pieces.pieces(real_type);
    while piece_set != 0 {
        let id = piece_set.trailing_zeros() as i8;
        let cur_bishop = u64::nth(id as u8);
        piece_set ^= cur_bishop; // Remove bishop from the set.
        let cur_bishop_moves =
            compute_single_bishop_attacking_moves(ally_pieces, enemy_pieces, id as u8);
        let moves = internal::bitb64_to_moves_list(id as u8, cur_bishop_moves);
        result.insert(
            id as u8,
            PieceAndMoves {
                typpe: real_type,
                moves: moves,
            },
        );
    }
    result
}

pub struct BishopBitboardMoveGenerator {}

impl BitboardMoveGenerator for BishopBitboardMoveGenerator {
    fn get_raw_attacking_moves(pos: &Position, opts: MoveGenOpts) -> BitB64 {
        let (ally_pieces, enemy_pieces) = match opts.perspective {
            MoveGenPerspective::MovingPlayer => (pos.pieces_to_move(), pos.enemy_pieces()),
            MoveGenPerspective::WaitingPlayer => (pos.enemy_pieces(), pos.pieces_to_move()),
        };
        compute_raw_attacking_moves_as_bishop_internal(ally_pieces, enemy_pieces, PieceType::Bishop)
    }

    fn get_attacking_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        let (ally_pieces, enemy_pieces) = match opts.perspective {
            MoveGenPerspective::MovingPlayer => (pos.pieces_to_move(), pos.enemy_pieces()),
            MoveGenPerspective::WaitingPlayer => (pos.enemy_pieces(), pos.pieces_to_move()),
        };
        get_attacking_moves_as_bishop_internal(
            pos.pieces_to_move(),
            pos.enemy_pieces(),
            PieceType::Bishop,
        )
    }

    fn generate_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        Self::get_attacking_moves(pos, opts)
    }
}
