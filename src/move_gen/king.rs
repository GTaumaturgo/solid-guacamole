use rocket::futures::io::ReuniteError;

use super::{internal::bounded, BitboardMoveGenerator, MovesMap, PieceAndMoves};
use crate::chess::bitboard;
use crate::chess::position::{self, Position};
use crate::chess::{
    bitboard::{BitArraySize, SpecialMoveType},
    PlayerColor,
};
use crate::chess::{
    bitboard::{BitB64, BitboardMove, EMPTY_BOARD, FULL_BOARD},
    PieceType,
};
use crate::move_gen::internal::{self, get_ij_from_sq_id, get_sq_id_from_ij};
use std::collections::HashMap;

pub struct KingBitboardMoveGenerator {}

fn get_long_castle_sq_id(pos: &Position) -> u8 {
    match pos.player_to_move() {
        PlayerColor::White => bitboard::G1,
        PlayerColor::Black => bitboard::G8,
    }
}

fn get_short_castle_sq_id(pos: &Position) -> u8 {
    match pos.player_to_move() {
        PlayerColor::White => bitboard::B1,
        PlayerColor::Black => bitboard::B8,
    }
}

fn short_castle_valid(pos: &Position) -> bool {
    if !pos
        .position_info
        .short_castling_allowed(pos.player_to_move())
    {
        return false;
    }

    false
}

fn long_castle_valid(pos: &Position) -> bool {
    if !pos
        .position_info
        .long_castling_allowed(pos.player_to_move())
    {
        return false;
    }
    false
}

impl BitboardMoveGenerator for KingBitboardMoveGenerator {
    fn get_attacking_moves(pos: &Position) -> MovesMap {
        let mut result = HashMap::new();
        let king: u64 = pos.pieces_to_move().king;
        let id: i8 = king.trailing_zeros() as i8;
        if !(bounded(id, 0, 63)) {
            println!("Attemped to generate moves for king but there is no king!");
            return result;
        }
        let (i0, j0) = get_ij_from_sq_id(id);

        let mut cur_king_moves = EMPTY_BOARD;

        let ally_pieces = pos.pieces_to_move();
        for i in [-1i8, 0i8, 1i8] {
            for j in [-1i8, 0i8, 1i8] {
                if i == 0 && j == 0 {
                    continue;
                }
                let (i1, j1) = (i0 + i, j0 + j);
                if !internal::is_inside_board(i1, j1) {
                    continue;
                }
                cur_king_moves |= u64::nth(get_sq_id_from_ij(i1, j1) as u8);
            }
        }
        println!("cur king moves: {}", cur_king_moves);

        // Can't move to squares that contain our pieces.
        cur_king_moves &= u64::compl(ally_pieces.all_pieces());
        let moves = internal::bitb64_to_moves_list(id as u8, cur_king_moves);
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

    fn generate_moves(pos: &Position) -> MovesMap {
        let king: u64 = pos.pieces_to_move().king;
        let id = king.trailing_zeros() as u8;

        let mut result = Self::get_attacking_moves(pos);

        if short_castle_valid(pos) {
            if let Some(x) = result.get_mut(&id) {
                x.moves.push(BitboardMove {
                    from: id,
                    to: get_short_castle_sq_id(pos),
                    sp_move_type: SpecialMoveType::ShortCastle,
                });
            }
        }
        if long_castle_valid(pos) {
            if let Some(x) = result.get_mut(&id) {
                x.moves.push(BitboardMove {
                    from: id,
                    to: get_short_castle_sq_id(pos),
                    sp_move_type: SpecialMoveType::LongCastle,
                });
            }
        }
        result
    }
}
