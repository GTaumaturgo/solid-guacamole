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

pub fn get_attacking_moves_as(pos: &Position, real_type: PieceType) -> MovesMap {
    let mut result = HashMap::new();
    let mut piece_set = *pos.pieces_to_move().pieces(real_type);
    while piece_set != 0 {
        let id = piece_set.trailing_zeros() as i8;
        let (i0, j0) = get_ij_from_sq_id(id);
        let cur_bishop = u64::nth(id as u8);
        piece_set ^= cur_bishop; // Remove bishop from the set.
        let mut cur_bishop_moves = bitboard::EMPTY_BOARD;

        let enemy_pieces = pos.enemy_pieces();
        let ally_pieces = pos.pieces_to_move();

        // Try all possible distances (1..7 in all four diagonals):
        let mut all_diagonals_blockedness = vec![
            false, // upleft
            false, // upright
            false, // downleft
            false, // downright
        ];
        for i in 1..7 {
            let ij_upleft = (i0 + i, j0 - i);
            let ij_upright = (i0 + i, j0 + i);
            let ij_downleft = (i0 - i, j0 - i);
            let ij_downright = (i0 - i, j0 + i);
            let all_dir_ids = vec![ij_upleft, ij_upright, ij_downleft, ij_downright];
            let mut any_non_blocked = false;
            for (dir_sq_id, mut dir_blocked) in
                all_dir_ids.iter().zip(all_diagonals_blockedness.iter_mut())
            {
                try_generate_move_in_direction(
                    *dir_sq_id,
                    ally_pieces,
                    enemy_pieces,
                    &mut dir_blocked,
                    &mut cur_bishop_moves,
                );
                any_non_blocked |= !*dir_blocked;
            }
            if !any_non_blocked {
                break;
            }
        }
        let moves = internal::bitb64_to_moves_list(id as u8, cur_bishop_moves);
        result.insert(
            id as u8,
            PieceAndMoves {
                typpe: PieceType::Rook,
                moves: moves,
            },
        );
    }
    result
}
pub fn generate_moves_as(pos: &Position, real_type: PieceType) -> MovesMap {
    // Bishop atatacking moves are all moves it has
    get_attacking_moves_as(pos, real_type)
}

pub struct BishopBitboardMoveGenerator {}

impl BitboardMoveGenerator for BishopBitboardMoveGenerator {
    fn get_attacking_moves(pos: &Position) -> MovesMap {
        get_attacking_moves_as(pos, PieceType::Bishop)
    }

    fn generate_moves(pos: &Position) -> MovesMap {
        generate_moves_as(pos, PieceType::Bishop)
    }
}
