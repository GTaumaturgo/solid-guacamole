use super::{BitboardMoveGenerator, MovesMap, PieceAndMoves};

use crate::chess::bitboard::{BitArraySize, PlayerBitboard};
use crate::chess::position::{self, Position};
use crate::chess::PlayerColor;
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD},
    PieceType,
};
use std::collections::HashMap;

pub struct PawnBitboardMoveGenerator {}

impl BitboardMoveGenerator for PawnBitboardMoveGenerator {
    fn generate_moves(pos: &Position) -> MovesMap {
        let mut result = HashMap::new();
        let pieces_to_move = pos.pieces_to_move();
        let mut pawn_set = pieces_to_move.pawns;
        while pawn_set != 0 {
            let id = pawn_set.trailing_zeros() as u8;
            let cur_pawn = u64::nth(id);
            let cur_pawn_row = id >> 3;

            pawn_set ^= cur_pawn;

            let mut cur_pawn_moves = EMPTY_BOARD;
            let player_color = pos.player_to_move();
            // Remove current piece.
            let pawns_initial_row = match player_color {
                PlayerColor::Black => 6,
                PlayerColor::White => 1,
            };
            let pawn_move_direction: i8 = match player_color {
                PlayerColor::Black => -1,
                PlayerColor::White => 1,
            };
            let double_advance_offset = match player_color {
                PlayerColor::Black => 32, // 64 - (3 << 8)
                PlayerColor::White => 24, // (3 << 8)
            };
            let advance_sq_id = id as i8 + (pawn_move_direction * 8);
            let advance_square = u64::nth(advance_sq_id as u8);
            let piece_in_front = (advance_square
                & (pos.enemy_pieces().all_pieces() | pos.pieces_to_move().all_pieces()))
                != 0;
            if !piece_in_front {
                cur_pawn_moves |= advance_square;
                if (cur_pawn_row) == pawns_initial_row {
                    cur_pawn_moves |= u64::nth(double_advance_offset + (id % 8));
                }
            }
            if (id % 8) != 0 {
                if u64::nth((advance_sq_id - 1) as u8) & pos.enemy_pieces().all_pieces() != 0 {
                    cur_pawn_moves |= u64::nth((advance_sq_id - 1) as u8);
                }
            }
            if (id % 8) != 7 {
                if u64::nth((advance_sq_id + 1) as u8) & pos.enemy_pieces().all_pieces() != 0 {
                    cur_pawn_moves |= u64::nth((advance_sq_id + 1) as u8);
                }
            }
            cur_pawn_moves &= u64::compl(pieces_to_move.all_pieces());
            if cur_pawn_moves != EMPTY_BOARD {
                result.insert(
                    id,
                    PieceAndMoves {
                        typpe: PieceType::Pawn,
                        moves: cur_pawn_moves,
                    },
                );
            }
        }
        result
    }
}
