use super::internal::{self, bitb64_to_moves_list};
use super::internal::{get_ij_from_sq_id, try_generate_move_in_direction};
use super::{BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::bitboard;
use crate::chess::bitboard::{BitArraySize, BitB64, BitboardMove, PlayerBitboard, EMPTY_BOARD};
use crate::chess::position::Position;
use crate::chess::{PieceType, PlayerColor};
use std::collections::hash_map;
use std::collections::HashMap;

pub struct RookBitboardMoveGenerator {}

pub fn compute_single_rook_attacking_moves(pos: &Position, id: u8) -> BitB64 {
    let (i0, j0) = get_ij_from_sq_id(id as i8);

    let enemy_pieces = pos.enemy_pieces();
    let ally_pieces = pos.pieces_to_move();
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

fn compute_rook_attacking_moves(pos: &Position) -> BitB64 {
    let mut result = EMPTY_BOARD;
    let mut piece_set = pos.pieces_to_move().rooks;
    while piece_set != 0 {
        let id = piece_set.trailing_zeros() as u8;
        result |= compute_single_rook_attacking_moves(pos, id);
        piece_set ^= u64::nth(id);
    }
    result
}

pub fn get_attacking_moves_as_rook(pos: &Position, real_type: PieceType) -> MovesMap {
    let mut result = HashMap::new();
    let mut piece_set = *pos.pieces_to_move().pieces(real_type);
    while piece_set != 0 {
        let id = piece_set.trailing_zeros() as u8;
        let moves = compute_single_rook_attacking_moves(pos, id);
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

pub fn generate_moves_as_rook(pos: &Position, real_type: PieceType) -> MovesMap {
    // Rook atatacking moves are all moves it has
    get_attacking_moves_as_rook(pos, real_type)
}

impl BitboardMoveGenerator for RookBitboardMoveGenerator {
    fn get_attacking_moves(pos: &Position) -> MovesMap {
        get_attacking_moves_as_rook(pos, PieceType::Rook)
    }

    fn generate_moves(pos: &Position) -> MovesMap {
        generate_moves_as_rook(pos, PieceType::Rook)
    }
}
