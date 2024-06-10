use rocket::futures::io::ReuniteError;

use super::internal::intersect;
use super::{internal::bounded, BitboardMoveGenerator, MovesMap, PieceAndMoves};
use super::{MoveGenOpts, MoveGenPerspective};
use crate::chess::bitboard::{self, PlayerBitboard};
use crate::chess::position::{self, Position, PositionInfo};
use crate::chess::{
    bitboard::{BitArraySize, SpecialMoveType},
    PlayerColor,
};
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD, FULL_BOARD},
    PieceType,
};
use crate::move_gen::internal::{self, bitb64_to_moves_list, get_ij_from_sq_id, get_sq_id_from_ij};
use std::collections::HashMap;

pub struct KingBitboardMoveGenerator {}

fn long_castle_not_blocked_by_pieces(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    p_to_move: PlayerColor,
) -> bool {
    let mask = match p_to_move {
        PlayerColor::White => u64::nth(bitboard::B1) | u64::nth(bitboard::C1),
        PlayerColor::Black => u64::nth(bitboard::B8) | u64::nth(bitboard::C8),
    };
    intersect(ally_pieces.all_pieces() | enemy_pieces.all_pieces(), mask)
}

fn short_castle_not_blocked_by_pieces(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    p_to_move: PlayerColor,
) -> bool {
    let mask = match p_to_move {
        PlayerColor::White => u64::nth(bitboard::F1) | u64::nth(bitboard::G1),
        PlayerColor::Black => u64::nth(bitboard::F8) | u64::nth(bitboard::G8),
    };
    intersect(ally_pieces.all_pieces() | enemy_pieces.all_pieces(), mask)
}

fn get_short_castle_sq_id(p_to_move: PlayerColor) -> u8 {
    match p_to_move {
        PlayerColor::White => bitboard::G1,
        PlayerColor::Black => bitboard::G8,
    }
}

fn get_long_castle_sq_id(p_to_move: PlayerColor) -> u8 {
    match p_to_move {
        PlayerColor::White => bitboard::C1,
        PlayerColor::Black => bitboard::C8,
    }
}

fn squares_to_check_for_long_castle(p_to_move: PlayerColor) -> BitB64 {
    match p_to_move {
        PlayerColor::White => {
            u64::nth(bitboard::C1) | u64::nth(bitboard::D1) | u64::nth(bitboard::E1)
        }
        PlayerColor::Black => {
            u64::nth(bitboard::C8) | u64::nth(bitboard::D8) | u64::nth(bitboard::E8)
        }
    }
}

fn squares_to_check_for_short_castle(p_to_move: PlayerColor) -> BitB64 {
    match p_to_move {
        PlayerColor::White => {
            u64::nth(bitboard::E1) | u64::nth(bitboard::F1) | u64::nth(bitboard::G1)
        }
        PlayerColor::Black => {
            u64::nth(bitboard::E8) | u64::nth(bitboard::F8) | u64::nth(bitboard::G8)
        }
    }
}

fn short_castle_valid(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    pos_info: &PositionInfo,
    enemy_attacked_squares: BitB64,
) -> bool {
    let p_to_move = pos_info.player_to_move();
    if !pos_info.has_short_castling_rights(p_to_move) {
        false
    } else if intersect(
        enemy_attacked_squares,
        squares_to_check_for_short_castle(p_to_move),
    ) {
        false
    } else if short_castle_not_blocked_by_pieces(ally_pieces, enemy_pieces, p_to_move) {
        false
    } else {
        true
    }
}

fn long_castle_valid(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    pos_info: &PositionInfo,
    enemy_attacked_squares: BitB64,
) -> bool {
    if !pos_info.has_long_castling_rights(pos_info.player_to_move()) {
        false
    } else if intersect(
        enemy_attacked_squares,
        squares_to_check_for_long_castle(pos_info.player_to_move()),
    ) {
        false
    } else if long_castle_not_blocked_by_pieces(
        ally_pieces,
        enemy_pieces,
        pos_info.player_to_move(),
    ) {
        false
    } else {
        true
    }
}

fn compute_king_attacking_moves(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
) -> BitB64 {
    let king: u64 = ally_pieces.king;
    let id: i8 = king.trailing_zeros() as i8;
    let mut result = EMPTY_BOARD;
    let (i0, j0) = get_ij_from_sq_id(id);
    for i in [-1i8, 0i8, 1i8] {
        for j in [-1i8, 0i8, 1i8] {
            if i == 0 && j == 0 {
                continue;
            }
            let (i1, j1) = (i0 + i, j0 + j);
            if !internal::is_inside_board(i1, j1) {
                continue;
            }
            result |= u64::nth(get_sq_id_from_ij(i1, j1) as u8);
        }
    }
    result &= u64::compl(ally_pieces.all_pieces());
    result
}

fn get_attacking_moves_internal(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
) -> MovesMap {
    let king: u64 = ally_pieces.king;
    if king == EMPTY_BOARD {
        println!("Attemped to generate moves for king but there is no king!");
        return HashMap::new();
    }

    let mut result = HashMap::new();
    let id: i8 = king.trailing_zeros() as i8;

    let moves = internal::bitb64_to_moves_list(
        id as u8,
        compute_king_attacking_moves(ally_pieces, enemy_pieces),
    );
    if !moves.is_empty() {
        result.insert(
            id as u8,
            PieceAndMoves {
                typpe: PieceType::King,
                moves,
            },
        );
    }
    result
}

fn generate_moves_internal(
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    pos_info: &PositionInfo,
    enemy_attacked_squares: BitB64,
) -> MovesMap {
    let king: u64 = ally_pieces.king;
    let id = king.trailing_zeros() as u8;

    let mut result = get_attacking_moves_internal(ally_pieces, enemy_pieces);

    if short_castle_valid(ally_pieces, enemy_pieces, pos_info, enemy_attacked_squares) {
        if let Some(x) = result.get_mut(&id) {
            x.moves.push(BitboardMove {
                from: id,
                to: get_short_castle_sq_id(pos_info.player_to_move()),
                sp_move_type: SpecialMoveType::ShortCastle,
            });
        }
    }
    if long_castle_valid(ally_pieces, enemy_pieces, pos_info, enemy_attacked_squares) {
        if let Some(x) = result.get_mut(&id) {
            x.moves.push(BitboardMove {
                from: id,
                to: get_short_castle_sq_id(pos_info.player_to_move()),
                sp_move_type: SpecialMoveType::LongCastle,
            });
        }
    }
    result
}

impl BitboardMoveGenerator for KingBitboardMoveGenerator {
    fn get_attacking_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        match opts.perspective {
            MoveGenPerspective::MovingPlayer => {
                get_attacking_moves_internal(pos.pieces_to_move(), pos.enemy_pieces())
            }
            MoveGenPerspective::WaitingPlayer => {
                get_attacking_moves_internal(pos.enemy_pieces(), pos.pieces_to_move())
            }
        }
    }

    fn generate_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap {
        match opts.perspective {
            MoveGenPerspective::MovingPlayer => generate_moves_internal(
                pos.pieces_to_move(),
                pos.enemy_pieces(),
                &pos.position_info,
                pos.get_raw_attacked_squares_for_waiting_player(),
            ),
            MoveGenPerspective::WaitingPlayer => generate_moves_internal(
                pos.enemy_pieces(),
                pos.pieces_to_move(),
                &pos.position_info,
                pos.get_raw_attacked_squares_for_moving_player(),
            ),
        }
    }
}
