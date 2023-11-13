use super::{MovesMap, PieceAndMoves};
use crate::bitb;
use crate::chess::position::{self, Position};
use crate::chess::PlayerColor;
use crate::chess::{
    bitboard::{empty_board, BitB64, BitboardMove},
    PieceType,
};
use std::collections::hash_map;
use std::collections::HashMap;

pub fn generate_moves(pos: &Position, mut knight_set: BitB64) -> MovesMap {
    let mut result = HashMap::new();
    let mut i = 0;
    while knight_set != 0 {
        let id = knight_set.trailing_zeros() as u8;
        let cur_knight = bitb!(id);
        knight_set ^= cur_knight;
        let mut cur_knight_moves = empty_board;

        for i in -2..3 as i8{
            for j in -2..3 as i8{
                if i != 0 && j != 0 && i * i + j * j == 5 {
                    let to = id as i8 + i * 8 + j;
                    if to < 64 && to >= 0 {
                        cur_knight_moves |= bitb!(to);
                    }
                }
            }
        }

        if cur_knight_moves == empty_board {
            continue;
        };
        result.insert(
            id,
            PieceAndMoves {
                typpe: PieceType::Knight,
                moves: cur_knight_moves,
            },
        );
    }
    result
}