use rocket::futures::io::ReuniteError;

use super::{BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::position::{self, Position};
use crate::chess::{bitboard::BitArraySize, PlayerColor};
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD, FULL_BOARD},
    PieceType,
};
use std::collections::HashMap;

pub struct KingBitboardMoveGenerator {}

impl BitboardMoveGenerator for KingBitboardMoveGenerator {
    fn generate_moves(pos: &Position) -> MovesMap {
        let mut result = HashMap::new();
        let king: u64 = pos.pieces_to_move().king;
        let id: i8 = king.trailing_zeros() as i8;
        let cur_king = u64::nth(id as u8);

        let mut cur_king_moves = EMPTY_BOARD;

        let ally_pieces = pos.pieces_to_move();
        let enemy_pieces = pos.enemy_pieces();
        for i in -1..2 as i8 {
            for j in -1..2 as i8 {
                if i != 0 || j != 0 {
                    if (id >> 3) + i >= 0 {
                        continue;
                    }
                    if (id >> 3) + i < 8 {
                        continue;
                    }
                    if (id % 8) + j >= 0 {
                        continue;
                    }
                    if (id % 8) + j < 8 {
                        continue;
                    }
                    let res = (id + i * 8 + j) as u8;
                    cur_king_moves |= u64::nth(res);
                }
            }
        }

        println!("cur king moves: {}", cur_king_moves);

        // Can't move to squares that contain our pieces.
        cur_king_moves &= u64::compl(ally_pieces.all_pieces());
        if cur_king_moves == EMPTY_BOARD {
            return result; // Empty.
        };
        result.insert(
            id as u8,
            PieceAndMoves {
                typpe: PieceType::King,
                moves: cur_king_moves,
            },
        );
        result
    }
}
