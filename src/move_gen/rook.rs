use super::internal::try_generate_move_in_direction;
use super::{BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::bitboard::{BitArraySize, BitB64, BitboardMove, PlayerBitboard, EMPTY_BOARD};
use crate::chess::position::Position;
use crate::chess::{PieceType, PlayerColor};
use std::collections::hash_map;
use std::collections::HashMap;

pub struct RookBitboardMoveGenerator {}

pub fn generate_moves_as(pos: &Position, mut piece_set: BitB64) -> MovesMap {
    let mut result = HashMap::new();
    while piece_set != 0 {
        let id = piece_set.trailing_zeros() as i8;
        let enemy_pieces = pos.enemy_pieces();
        let ally_pieces = pos.pieces_to_move();
        let cur_rook = u64::nth(id as u8);
        piece_set ^= cur_rook;
        let mut cur_rook_moves = EMPTY_BOARD;
        // Try all possible distances (1..7 in all four diagonals):
        let mut all_dirs_blockedness = vec![
            false, // up
            false, // down
            false, // right
            false, // left
        ];
        for i in 1..7 {
            let id_i8 = id as i8;
            let id_up = 8 * i + id_i8;
            let id_down = id_i8 - 8 * i;
            let id_right = id_i8 + i;
            let id_left = id_i8 - i;
            let all_dir_ids = vec![id_up, id_down, id_right, id_left];
            for (dir_sq_id, mut dir_blocked) in
                all_dir_ids.iter().zip(all_dirs_blockedness.iter_mut())
            {
                try_generate_move_in_direction(
                    *dir_sq_id,
                    ally_pieces,
                    enemy_pieces,
                    &mut dir_blocked,
                    &mut cur_rook_moves,
                );
            }
        }
        if cur_rook_moves != EMPTY_BOARD {
            result.insert(
                id as u8,
                PieceAndMoves {
                    typpe: PieceType::Rook,
                    moves: cur_rook_moves,
                },
            );
        };
    }
    result
}

impl BitboardMoveGenerator for RookBitboardMoveGenerator {
    fn generate_moves(pos: &Position) -> MovesMap {
        generate_moves_as(pos, pos.pieces_to_move().rooks)
    }
}
