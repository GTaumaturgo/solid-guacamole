use super::{BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::PlayerColor;
use crate::chess::{
    bitboard::BitArraySize,
    position::{self, Position},
};
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD},
    PieceType,
};

use std::collections::HashMap;

pub struct KnightBitboardMoveGenerator {}

impl BitboardMoveGenerator for KnightBitboardMoveGenerator {
    fn generate_moves(pos: &Position) -> MovesMap {
        let mut result = HashMap::new();
        let pieces_to_move = pos.pieces_to_move();
        let mut knight_set = pieces_to_move.knights;
        while knight_set != 0 {
            let id = knight_set.trailing_zeros() as i32;
            let cur_knight = u64::nth(id as u8);
            knight_set ^= cur_knight;
            let mut cur_knight_moves = EMPTY_BOARD;

            for i in [-2, -1, 1, 2].iter() {
                for j in [-2, -1, 1, 2].iter() {
                    if i * i + j * j != 5 {
                        continue;
                    }
                    if (id >> 3) + i < 0 {
                        continue;
                    }
                    if (id >> 3) + i >= 8 {
                        continue;
                    }
                    if (id % 8) + j < 0 {
                        continue;
                    }
                    if (id % 8) + j >= 8 {
                        continue;
                    }
                    let to = (id + i * 8 + j) as u8;
                    cur_knight_moves |= u64::nth(to);
                }
            }
            // Can't move to squares that contain our pieces.
            cur_knight_moves &= u64::compl(pieces_to_move.all_pieces());
            result.insert(
                id as u8,
                PieceAndMoves {
                    typpe: PieceType::Knight,
                    moves: cur_knight_moves,
                },
            );
        }
        result
    }
}
