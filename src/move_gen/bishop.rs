use super::internal::{intersect, is_inside_board, try_generate_move_in_direction};
use super::{BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::bitboard::{BitArraySize, PlayerBitboard};
use crate::chess::position::Position;
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD},
    PieceType,
};

use std::collections::HashMap;

pub fn generate_moves_as(pos: &Position, mut piece_set: BitB64) -> MovesMap {
    let mut result = HashMap::new();
    while piece_set != 0 {
        let id = piece_set.trailing_zeros() as i8;
        let cur_bishop = u64::nth(id as u8);
        piece_set ^= cur_bishop; // Remove bishop from the set.
        let mut cur_bishop_moves = EMPTY_BOARD;

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
            let id_upleft = 8 * i + id - i;
            let id_upright = 8 * i + id + i;
            let id_downleft = (id - 8 * i) - i;
            let id_downright = (id - 8 * i) + i;
            let all_dir_ids = vec![id_upleft, id_upright, id_downleft, id_downright];
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
            }
        }
        if cur_bishop_moves != EMPTY_BOARD {
            result.insert(
                id as u8,
                PieceAndMoves {
                    typpe: PieceType::Bishop,
                    moves: cur_bishop_moves,
                },
            );
        };
    }
    result
}

pub struct BishopBitboardMoveGenerator {}

impl BitboardMoveGenerator for BishopBitboardMoveGenerator {
    fn generate_moves(pos: &Position) -> MovesMap {
        generate_moves_as(pos, pos.pieces_to_move().bishops)
    }
}
