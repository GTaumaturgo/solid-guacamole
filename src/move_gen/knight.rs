use super::internal;
use super::{internal::bounded, BitboardMoveGenerator, MovesMap, PieceAndMoves};
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
            let id = knight_set.trailing_zeros() as i8;
            let cur_knight = u64::nth(id as u8);
            knight_set ^= cur_knight;
            let mut cur_knight_moves = EMPTY_BOARD;

            for i in [-2i8, -1i8, 1i8, 2i8].iter() {
                for j in [-2i8, -1i8, 1i8, 2i8].iter() {
                    if i * i + j * j != 5 {
                        continue;
                    }
                    let i_pos = (id >> 3) + i;
                    let j_pos = (id % 8) + j;
                    if !bounded(i_pos, 0, 7) || !bounded(j_pos, 0, 7) {
                        continue;
                    }
                    let to = (i_pos * 8 + j_pos) as u8;
                    cur_knight_moves |= u64::nth(to);
                }
            }
            // Can't move to squares that contain our pieces.
            cur_knight_moves &= u64::compl(pieces_to_move.all_pieces());

            let mut resulting_moves = internal::bitb64_to_moves_list(id as u8, cur_knight_moves);
            if resulting_moves.len() > 0 {
                result.insert(
                    id as u8,
                    PieceAndMoves {
                        typpe: PieceType::King,
                        moves: resulting_moves,
                    },
                );
            };
        }
        result
    }
}
