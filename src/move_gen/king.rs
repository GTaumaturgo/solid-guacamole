use rocket::futures::io::ReuniteError;

use super::{internal::bounded, BitboardMoveGenerator, MovesMap, PieceAndMoves};
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

impl BitboardMoveGenerator for KingBitboardMoveGenerator {
    fn generate_moves(pos: &Position) -> MovesMap {
        let mut result = HashMap::new();
        let king: u64 = pos.pieces_to_move().king;
        let id: i8 = king.trailing_zeros() as i8;
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

        let mut resulting_moves = internal::bitb64_to_moves_list(id as u8, cur_king_moves);
        if resulting_moves.len() > 0 {
            result.insert(
                id as u8,
                PieceAndMoves {
                    typpe: PieceType::King,
                    moves: resulting_moves,
                },
            );
        };
        result
    }
}
