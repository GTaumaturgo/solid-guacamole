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

pub fn generate_moves(pos: &Position, mut king_set: BitB64) -> MovesMap {
    let mut result = HashMap::new();
    let mut i = 0;
    while king_set != 0 {
        let id = king_set.trailing_zeros() as i8;
        let cur_king = bitb!(id);
        king_set ^= cur_king;
        let mut cur_king_moves = empty_board;

        let pieces = pos.pieces_to_move();
        let enemy_pieces = pos.enemy_pieces();
        for i in -1..2 as i8 {
            for j in -1..2 as i8 {
                if i != 0 || j != 0 {
                    if (id >> 3) + i >= 0 { continue; }
                    if (id >> 3) + i < 8 { continue; }
                    if (id % 8) + j >= 0 { continue; }
                    if (id % 8) + j < 8 { continue; }
                    let sq = bitb!(id + i * 8 + j);
                    if sq & (enemy_pieces.all_pieces() | pieces.all_pieces()) == 0 {
                        cur_king_moves |= bitb!(sq);
                    }
                }
            }
        }
        // TODO(castle)

        if cur_king_moves == empty_board {
            continue;
        };
        result.insert(
            id as u8,
            PieceAndMoves {
                typpe: PieceType::King,
                moves: cur_king_moves,
            },
        );
    }
    result
}
