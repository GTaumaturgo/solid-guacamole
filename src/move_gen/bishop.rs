use super::internal;
use super::internal::{
    get_ij_from_sq_id, intersect, is_inside_board, try_generate_move_in_direction,
};
use super::{BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::bitboard::{self, BitArraySize, PlayerBitboard};
use crate::chess::position::Position;
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD},
    PieceType,
};

use std::collections::HashMap;

fn compute_single_bishop_attacking_moves(pos: &Position, id: u8) -> BitB64 {
    let ally_pieces = pos.pieces_to_move();
    let enemy_pieces = pos.enemy_pieces();

    let mut cur_bishop_moves = EMPTY_BOARD;
    let mut dir_blockedness = vec![
        false, // upleft
        false, // upright
        false, // downleft
        false, // downright
    ];

    let (i0, j0) = get_ij_from_sq_id(id as i8);
    // Try all possible distances (1..7 in all four diagonals):
    for i in 1..7 {
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

pub fn get_attacking_moves_as_bishop(pos: &Position, real_type: PieceType) -> MovesMap {
    let mut result = HashMap::new();
    let mut piece_set = *pos.pieces_to_move().pieces(real_type);
    while piece_set != 0 {
        let id = piece_set.trailing_zeros() as i8;
        let cur_bishop = u64::nth(id as u8);
        piece_set ^= cur_bishop; // Remove bishop from the set.
        let cur_bishop_moves = compute_single_bishop_attacking_moves(pos, id as u8);
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

pub fn generate_moves_as_bishop(pos: &Position, real_type: PieceType) -> MovesMap {
    // Bishop atatacking moves are all moves it has
    get_attacking_moves_as_bishop(pos, real_type)
}

pub struct BishopBitboardMoveGenerator {}

impl BitboardMoveGenerator for BishopBitboardMoveGenerator {
    fn get_attacking_moves(pos: &Position) -> MovesMap {
        get_attacking_moves_as_bishop(pos, PieceType::Bishop)
    }

    fn generate_moves(pos: &Position) -> MovesMap {
        generate_moves_as_bishop(pos, PieceType::Bishop)
    }
}
