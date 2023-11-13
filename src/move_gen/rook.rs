use super::{MovesMap, PieceAndMoves};
use crate::bitb;
use crate::chess::bitboard::{empty_board, BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::{PieceType, PlayerColor};
use std::collections::hash_map;
use std::collections::HashMap;

pub fn generate_moves(pos: &Position, mut rook_set: BitB64) -> MovesMap {
    let mut result = HashMap::new();
    let mut i = 0;
    while rook_set != 0 {
        let id = rook_set.trailing_zeros() as u8;
        let cur_rook = bitb!(id);
        rook_set ^= cur_rook;
        let mut cur_rook_moves = empty_board;

        let mut blocking_upwards = false;
        let mut blocking_downwards = false;
        let mut blocking_leftwards = false;
        let mut blocking_rightwards = false;
        let enemy_pieces = pos.enemy_pieces();
        let own_pieces = pos.pieces_to_move();
        for i in 1..7 {
            if !blocking_upwards && (id + 8 * i < 64) {
                let sq_upwards = bitb!(id + (8 * i));
                if sq_upwards & own_pieces.all_pieces() != 0 {
                    blocking_upwards = true;
                }
                if sq_upwards & enemy_pieces.all_pieces() != 0 {
                    cur_rook_moves |= sq_upwards;
                    blocking_upwards = true;
                }
            }
            if !blocking_downwards && (8 * i <= id) {
                let sq_downwards = bitb!(id - (8 * i));
                if sq_downwards & own_pieces.all_pieces() != 0 {
                    blocking_downwards = true;
                }
                if sq_downwards & enemy_pieces.all_pieces() != 0 {
                    cur_rook_moves |= sq_downwards;
                    blocking_downwards = true;
                }
            }
            if !blocking_rightwards && (id % 8) + i < 8 {
                let sq_rightwards = bitb!(id + i);
                if sq_rightwards & own_pieces.all_pieces() != 0 {
                    blocking_rightwards = true;
                }
                if sq_rightwards & enemy_pieces.all_pieces() != 0 {
                    cur_rook_moves |= sq_rightwards;
                    blocking_rightwards = true;
                }
            }
            if !blocking_leftwards && i <= id % 8 {
                let sq_leftwards = bitb!(id - i);
                if sq_leftwards & own_pieces.all_pieces() != 0 {
                    blocking_downwards = true;
                }
                if sq_leftwards & enemy_pieces.all_pieces() != 0 {
                    cur_rook_moves |= sq_leftwards;
                    blocking_leftwards = true;
                }
            }
        }
        if cur_rook_moves == empty_board {
            continue;
        };
        result.insert(
            id,
            PieceAndMoves {
                typpe: PieceType::Rook,
                moves: cur_rook_moves,
            },
        );
    }
    result
}
