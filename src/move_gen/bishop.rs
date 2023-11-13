use super::{MovesMap, PieceAndMoves};
use crate::bitb;
use crate::chess::position::Position;
use crate::chess::PlayerColor;
use crate::chess::{
    bitboard::{empty_board, BitB64, BitboardMove},
    PieceType,
};
use std::collections::hash_map;
use std::collections::HashMap;

pub fn generate_moves(pos: &Position, mut bishop_set: BitB64) -> MovesMap {
    let mut result = HashMap::new();
    let mut i = 0;
    while bishop_set != 0 {
        let id = bishop_set.trailing_zeros() as u8;
        let cur_bishop = bitb!(id);
        bishop_set ^= cur_bishop;
        let mut cur_bishop_moves = empty_board;

        let mut blocking_upwards_leftwards = false;
        let mut blocking_upwards_rightwards = false;
        let mut blocking_downwards_leftwards = false;
        let mut blocking_downwards_rightwards = false;
        let enemy_pieces = pos.enemy_pieces();
        let own_pieces = pos.pieces_to_move();
        for i in 1..7 {
            if !blocking_upwards_leftwards && (id + 8 * i < 64 && id % 8 - i >= 0) {
                let sq_upwards_leftwards = bitb!(id + (8 * i) - i);
                if sq_upwards_leftwards & own_pieces.all_pieces() != 0 {
                    blocking_upwards_leftwards = true;
                }
                if sq_upwards_leftwards & enemy_pieces.all_pieces() != 0 {
                    cur_bishop_moves |= sq_upwards_leftwards;
                    blocking_upwards_leftwards = true;
                }
            }
            if !blocking_upwards_rightwards && (id + 8 * i < 64 && i <= id % 8) {
                let sq_upwards_rightwards = bitb!(id + (8 * i) + i);
                if sq_upwards_rightwards & own_pieces.all_pieces() != 0 {
                    blocking_upwards_rightwards = true;
                }
                if sq_upwards_rightwards & enemy_pieces.all_pieces() != 0 {
                    cur_bishop_moves |= sq_upwards_rightwards;
                    blocking_upwards_rightwards = true;
                }
            }
            if !blocking_downwards_leftwards && (8 * i <= id && id % 8 - i >= 0) {
                let sq_downwards_leftwards = bitb!(id - (8 * i) - i);
                if sq_downwards_leftwards & own_pieces.all_pieces() != 0 {
                    blocking_downwards_leftwards = true;
                }
                if sq_downwards_leftwards & enemy_pieces.all_pieces() != 0 {
                    cur_bishop_moves |= sq_downwards_leftwards;
                    blocking_downwards_leftwards = true;
                }
            }
            if !blocking_downwards_rightwards && i <= id % 8 && 8 * i <= id {
                let sq_downwards_rightwards = bitb!(id - (8 * i) + i);
                if sq_downwards_rightwards & own_pieces.all_pieces() != 0 {
                    blocking_downwards_rightwards = true;
                }
                if sq_downwards_rightwards & enemy_pieces.all_pieces() != 0 {
                    cur_bishop_moves |= sq_downwards_rightwards;
                    blocking_downwards_rightwards = true;
                }
            }
        }
        if cur_bishop_moves == empty_board {
            continue;
        };
        result.insert(
            id,
            PieceAndMoves {
                typpe: PieceType::Bishop,
                moves: cur_bishop_moves,
            },
        );
    }
    result
}