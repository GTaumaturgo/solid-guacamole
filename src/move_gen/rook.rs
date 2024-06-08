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

pub fn get_attacking_moves_as(pos: &Position, real_type: PieceType) -> MovesMap {
    let mut result = HashMap::new();
    let mut piece_set = *pos.pieces_to_move().pieces(real_type);
    while piece_set != 0 {
        let id = piece_set.trailing_zeros() as i8;
        let (i0, j0) = get_ij_from_sq_id(id);

        let enemy_pieces = pos.enemy_pieces();
        let ally_pieces = pos.pieces_to_move();
        let cur_rook = u64::nth(id as u8);
        piece_set ^= cur_rook;
        let mut cur_rook_moves = EMPTY_BOARD;
        let mut all_dirs_blockedness = vec![
            false, // up
            false, // down
            false, // right
            false, // left
        ];
        // Try all possible distances (1..7 in all four diagonals):
        for i in 1..7 {
            // up, down, right, left
            let all_dir_ids = vec![(i0 + i, j0), (i0 - i, j0), (i0, j0 + i), (i0, j0 - i)];
            let mut any_non_blocked = false;
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
                any_non_blocked |= !*dir_blocked;
            }
            if !any_non_blocked {
                break;
            }
        }
        let moves = bitb64_to_moves_list(id as u8, cur_rook_moves);
        result.insert(
            id as u8,
            PieceAndMoves {
                typpe: PieceType::Bishop,
                moves: moves,
            },
        );
    }
    result
}

pub fn generate_moves_as(pos: &Position, real_type: PieceType) -> MovesMap {
    // Rook atatacking moves are all moves it has
    get_attacking_moves_as(pos, real_type)
}

impl BitboardMoveGenerator for RookBitboardMoveGenerator {
    fn get_attacking_moves(pos: &Position) -> MovesMap {
        get_attacking_moves_as(pos, PieceType::Rook)
    }

    fn generate_moves(pos: &Position) -> MovesMap {
        generate_moves_as(pos, PieceType::Rook)
    }
}
