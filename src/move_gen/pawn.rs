use rocket::catcher::Result;

use super::internal::{self, bitb64_to_moves_list};
use super::internal::{get_ij_from_sq_id, intersect};
use super::{BitboardMoveGenerator, MoveGenOpts, MovesMap, PieceAndMoves};
use crate::chess::bitboard::{BitArraySize, PlayerBitboard, SpecialMoveType};
use crate::chess::position::{self, Position, PositionInfo};
use crate::chess::PlayerColor;
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD},
    PieceType,
};
use crate::move_gen::MoveGenPerspective;
use std::collections::HashMap;

pub struct PawnBitboardMoveGenerator {}

fn compute_raw_single_pawn_attacking_moves(
    enemy_pieces: &PlayerBitboard,
    p_to_move: PlayerColor,
    id: i8,
) -> BitB64 {
    let (_, j) = get_ij_from_sq_id(id);
    let mut cur_pawn_moves = EMPTY_BOARD;

    let pawn_move_direction: i8 = match p_to_move {
        PlayerColor::Black => -1,
        PlayerColor::White => 1,
    };

    let leftmost_col = match p_to_move {
        PlayerColor::Black => 7,
        PlayerColor::White => 0,
    };

    let rightmost_col = match p_to_move {
        PlayerColor::Black => 0,
        PlayerColor::White => 7,
    };

    let advance_sq_id = (id + pawn_move_direction * 8) as i8;
    let capture_left_sq_id = advance_sq_id - pawn_move_direction;
    if internal::bounded(capture_left_sq_id, 0, 63) {
        let sq_capture_left = u64::nth((capture_left_sq_id) as u8);
        if j != leftmost_col && intersect(sq_capture_left, enemy_pieces.all_pieces()) {
            cur_pawn_moves |= sq_capture_left;
        }
    }

    let capture_right_sq_id = advance_sq_id + pawn_move_direction;
    if internal::bounded(capture_right_sq_id, 0, 63) {
        let sq_capture_right = u64::nth((capture_right_sq_id) as u8);
        if j != rightmost_col && intersect(sq_capture_right, enemy_pieces.all_pieces()) {
            cur_pawn_moves |= sq_capture_right;
        }
    }
    cur_pawn_moves
}

pub fn compute_pawn_attacking_moves_internal(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    p_to_move: PlayerColor,
) -> BitB64 {
    let mut result = EMPTY_BOARD;
    let mut piece_set = ally_pieces.pawns;
    while piece_set != 0 {
        let id = piece_set.trailing_zeros() as u8;
        result |= compute_raw_single_pawn_attacking_moves(enemy_pieces, p_to_move, id as i8);
        piece_set ^= u64::nth(id);
    }
    result
}

fn get_attacking_moves_internal(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    p_to_move: PlayerColor,
) -> MovesMap {
    let mut result = HashMap::new();
    let mut pawn_set = ally_pieces.pawns;
    while pawn_set != EMPTY_BOARD {
        let id = pawn_set.trailing_zeros() as i8;
        let cur_pawn: u64 = u64::nth(id as u8);
        pawn_set ^= cur_pawn;
        let resulting_moves = internal::bitb64_to_moves_list(
            id as u8,
            compute_raw_single_pawn_attacking_moves(enemy_pieces, p_to_move, id),
        );

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

fn add_promotion_moves(moves_list: Vec<BitboardMove>) -> Vec<BitboardMove> {
    let promotion_types = vec![
        SpecialMoveType::PromotionToBishop,
        SpecialMoveType::PromotionToKnight,
        SpecialMoveType::PromotionToRook,
        SpecialMoveType::PromotionToQueen,
    ];
    let mut promotion_resulting_moves = vec![];
    for mv in moves_list.iter() {
        for promotion_type in promotion_types.iter() {
            promotion_resulting_moves.push(BitboardMove {
                from: mv.from,
                to: mv.to,
                sp_move_type: *promotion_type,
            })
        }
    }
    promotion_resulting_moves
}

fn generate_moves_internal(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    p_to_move: PlayerColor,
) -> MovesMap {
    let mut result = HashMap::new();
    let mut pawn_set = ally_pieces.pawns;
    while pawn_set != EMPTY_BOARD {
        let id = pawn_set.trailing_zeros() as i8;
        let cur_pawn = u64::nth(id as u8);
        pawn_set ^= cur_pawn;
        let (i, j) = get_ij_from_sq_id(id);
        println!("Generating moves for pawn at i: {}, j: {}", i, j);
        let mut cur_pawn_moves = EMPTY_BOARD;
        let is_last_row_b4_promotion = match p_to_move {
            PlayerColor::White => 6i8 == i,
            PlayerColor::Black => 1i8 == i,
        };
        let pawns_initial_row = match p_to_move {
            PlayerColor::Black => 6,
            PlayerColor::White => 1,
        };
        let pawn_move_direction: i8 = match p_to_move {
            PlayerColor::Black => -1,
            PlayerColor::White => 1,
        };
        let advance_sq_id = (id + pawn_move_direction * 8) as i8;
        let advance_square = u64::nth(advance_sq_id as u8);
        let double_advance_offset = match p_to_move {
            PlayerColor::Black => 32, // Row 5
            PlayerColor::White => 24, // Row 4
        };
        let all_pieces = enemy_pieces.all_pieces() | ally_pieces.all_pieces();
        let piece_in_front = intersect(advance_square, all_pieces);
        if !piece_in_front {
            cur_pawn_moves |= advance_square;
            let double_adv_sq = u64::nth((double_advance_offset + j) as u8);
            if i == pawns_initial_row && !intersect(double_adv_sq, all_pieces) {
                cur_pawn_moves |= double_adv_sq;
            }
        }
        cur_pawn_moves |= compute_raw_single_pawn_attacking_moves(enemy_pieces, p_to_move, id);
        let resulting_moves = internal::bitb64_to_moves_list(id as u8, cur_pawn_moves);
        if resulting_moves.len() > 0 {
            result.insert(
                id as u8,
                PieceAndMoves {
                    typpe: PieceType::Pawn,
                    moves: match is_last_row_b4_promotion {
                        true => add_promotion_moves(resulting_moves),
                        false => resulting_moves,
                    },
                },
            );
        }
    }
    result
}

impl BitboardMoveGenerator for PawnBitboardMoveGenerator {
    fn get_raw_attacking_moves(pos: &Position, opts: MoveGenOpts) -> BitB64 {
        let (ally_pieces, enemy_pieces, p_to_move) = match opts.perspective {
            MoveGenPerspective::MovingPlayer => (
                pos.pieces_to_move(),
                pos.enemy_pieces(),
                pos.player_to_move(),
            ),
            MoveGenPerspective::WaitingPlayer => (
                pos.enemy_pieces(),
                pos.pieces_to_move(),
                pos.waiting_player(),
            ),
        };
        compute_pawn_attacking_moves_internal(ally_pieces, enemy_pieces, p_to_move)
    }

    fn get_attacking_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        let (ally_pieces, enemy_pieces, p_to_move) = match opts.perspective {
            MoveGenPerspective::MovingPlayer => (
                pos.pieces_to_move(),
                pos.enemy_pieces(),
                pos.player_to_move(),
            ),
            MoveGenPerspective::WaitingPlayer => (
                pos.enemy_pieces(),
                pos.pieces_to_move(),
                pos.waiting_player(),
            ),
        };
        get_attacking_moves_internal(ally_pieces, enemy_pieces, p_to_move)
    }

    fn generate_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        let (ally_pieces, enemy_pieces, p_to_move) = match opts.perspective {
            MoveGenPerspective::MovingPlayer => (
                pos.pieces_to_move(),
                pos.enemy_pieces(),
                pos.player_to_move(),
            ),
            MoveGenPerspective::WaitingPlayer => (
                pos.enemy_pieces(),
                pos.pieces_to_move(),
                pos.waiting_player(),
            ),
        };
        generate_moves_internal(ally_pieces, enemy_pieces, p_to_move)
    }
}
