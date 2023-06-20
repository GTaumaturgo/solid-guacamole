use crate::bitb;
use crate::chess::{PieceType, bitboard::{BitB64, BitboardMove, empty_board}};
use crate::chess::position::{Position, self};
use crate::chess::PlayerColor;
use std::collections::HashMap;
use std::collections::hash_map;
use super::{MovesMap, PieceAndMoves};

pub fn generate_moves(pos: &Position, mut pawn_set: BitB64) -> MovesMap {
    let mut result = HashMap::new();
    println!("outsidee loop" );
    while pawn_set != 0 {
        let id = 1 + pawn_set.trailing_zeros() as u8;
        let cur_pawn = bitb!(id);
        result.insert(id, PieceAndMoves {typpe: PieceType::Pawn, moves: empty_board });
        let cur_pawn_entry: &mut PieceAndMoves = result.get_mut(&id).unwrap();
        
        // Remove current piece.
        pawn_set ^= cur_pawn;
        let pawns_initial_row = match pos.player_to_move() {
            PlayerColor::Black => 6,
            PlayerColor::White => 1,
        };
        if (id / 8) == pawns_initial_row {
            cur_pawn_entry.moves |= (24 + (id % 8)) as BitB64; 
        }
        println!("pawn!");
    }    
    // for (sq_id, piece_n_moves) in continuations_map.iter() {
    //     let mut cur_piece_moves = piece_n_moves.moves;
    //         let zeros = cur_piece_moves.trailing_zeros() as u8;
    //     }
    // map.insert(
    //     0,
    //     PieceAndMoves {
    //         typpe: crate::chess::PieceType::King,
    //         moves: bitb!(8) | bitb!(16),
    //     },
    // );
    result
}
