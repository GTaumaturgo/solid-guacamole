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

pub fn generate_moves(pos: &Position, mut pawn_set: BitB64) -> MovesMap {
    let mut result = HashMap::new();
    println!("outsidee loop");
    let mut i = 0;
    while pawn_set != 0 {
        let id = pawn_set.trailing_zeros() as u8;
        let cur_pawn = bitb!(id);
        let cur_pawn_row = id >> 3;
        println!("pawn!");
        println!("pawnset: {}", id);
        println!("curpawn: {}", cur_pawn);
        println!("pawnset: {}", pawn_set);
        pawn_set ^= cur_pawn;
        println!("afterMutation: {}", pawn_set);
        let mut cur_pawn_moves = empty_board;
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
            PlayerColor::Black => 40, // 64 - (3 << 8)
            PlayerColor::White => 24, // (3 << 8)
        };
        let advance_sq_id = id as i8 + (pawn_move_direction * 8);
        let advance_square = bitb!(advance_sq_id);
        let piece_in_front = (advance_square
            & (pos.enemy_pieces().all_pieces() | pos.pieces_to_move().all_pieces()))
            != 0;
        if !piece_in_front {
            cur_pawn_moves |= advance_square;
            if (cur_pawn_row) == pawns_initial_row {
                cur_pawn_moves |= bitb!(double_advance_offset + (id % 8));
            }
        }
        if (id % 8) != 0 {
            if bitb!(advance_sq_id - 1) & pos.enemy_pieces().all_pieces() != 0 {
                cur_pawn_moves |= bitb!(advance_sq_id - 1);
            }
        }
        if (id % 8) != 7 {
            if bitb!(advance_sq_id + 1) & pos.enemy_pieces().all_pieces() != 0 {
                cur_pawn_moves |= bitb!(advance_sq_id + 1);
            }
        }
        if cur_pawn_moves == empty_board { continue;} 
        result.insert(
            id,
            PieceAndMoves {
                typpe: PieceType::Pawn,
                moves: cur_pawn_moves,
            },
        );
    }
    result
}
