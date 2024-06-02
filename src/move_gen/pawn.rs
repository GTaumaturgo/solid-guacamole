use super::internal;
use super::internal::{get_ij_from_sq_id, intersect};
use super::{BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::bitboard::{BitArraySize, PlayerBitboard};
use crate::chess::position::{self, Position};
use crate::chess::PlayerColor;
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD},
    PieceType,
};
use std::collections::HashMap;

// fn try_capture_left(ally: &PlayerBitboard, )

pub struct PawnBitboardMoveGenerator {}

impl BitboardMoveGenerator for PawnBitboardMoveGenerator {
    fn generate_moves(pos: &Position) -> MovesMap {
        let mut result = HashMap::new();
        let pieces_to_move = pos.pieces_to_move();
        let mut pawn_set = pieces_to_move.pawns;
        while pawn_set != 0 {
            let id = pawn_set.trailing_zeros() as i8;
            let cur_pawn = u64::nth(id as u8);
            pawn_set ^= cur_pawn;
            let (i, j) = get_ij_from_sq_id(id);
            println!("Generating moves for pawn at i: {}, j: {}", i, j);
            let mut cur_pawn_moves = EMPTY_BOARD;
            let player_color = pos.player_to_move();

            let pawns_initial_row = match player_color {
                PlayerColor::Black => 6,
                PlayerColor::White => 1,
            };
            let last_row = match player_color {
                PlayerColor::White => 6,
                PlayerColor::Black => 1,
            };
            let pawn_move_direction: i8 = match player_color {
                PlayerColor::Black => -1,
                PlayerColor::White => 1,
            };
            let advance_sq_id = (id + pawn_move_direction * 8) as i8;
            let advance_square = u64::nth(advance_sq_id as u8);
            let double_advance_offset = match player_color {
                PlayerColor::Black => 32, // 64 - (3 << 8)
                PlayerColor::White => 24, // (3 << 8)
            };

            let leftmost_col = match player_color {
                PlayerColor::Black => 7,
                PlayerColor::White => 0,
            };

            let rightmost_col = match player_color {
                PlayerColor::Black => 0,
                PlayerColor::White => 7,
            };

            println!(
                "enemy: {} ally: {}",
                pos.enemy_pieces().all_pieces(),
                pos.pieces_to_move().all_pieces()
            );
            let all_pieces = pos.enemy_pieces().all_pieces() | pos.pieces_to_move().all_pieces();
            let piece_in_front = intersect(advance_square, all_pieces);
            let (adv_i, adv_j) = get_ij_from_sq_id(advance_sq_id as i8);
            println!("advance coord: {} {}", adv_i, adv_j);
            println!("advance_square: {}", advance_square);
            println!("all pieces: {}", all_pieces);
            println!("piece in front: {}", piece_in_front);
            println!("checking advance");
            if !piece_in_front {
                cur_pawn_moves |= advance_square;
                let double_adv_sq = u64::nth((double_advance_offset + j) as u8);
                if i == pawns_initial_row && !intersect(double_adv_sq, all_pieces) {
                    cur_pawn_moves |= double_adv_sq;
                }
            }
            // Check capture to both sides.
            println!("checking capture left");
            let sq_capture_left = u64::nth((advance_sq_id - pawn_move_direction) as u8);
            println!("{}", advance_sq_id - pawn_move_direction);
            println!("{}", sq_capture_left);
            println!(
                "{}",
                intersect(sq_capture_left, pos.enemy_pieces().all_pieces())
            );
            if j != leftmost_col && intersect(sq_capture_left, pos.enemy_pieces().all_pieces()) {
                cur_pawn_moves |= sq_capture_left;
            }
            println!("checking capture right");
            let sq_capture_right = u64::nth((advance_sq_id + pawn_move_direction) as u8);
            println!("{}", sq_capture_right);
            println!(
                "{}",
                intersect(sq_capture_right, pos.enemy_pieces().all_pieces())
            );
            if j != rightmost_col && intersect(sq_capture_right, pos.enemy_pieces().all_pieces()) {
                cur_pawn_moves |= sq_capture_right;
            }
            let mut resulting_moves = internal::bitb64_to_moves_list(id as u8, cur_pawn_moves);
            if resulting_moves.len() > 0 {
                result.insert(
                    id as u8,
                    PieceAndMoves {
                        typpe: PieceType::Pawn,
                        moves: resulting_moves,
                    },
                );
            }
        }
        result
    }
}
