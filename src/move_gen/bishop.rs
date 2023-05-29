use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::PlayerColor;

fn bishop_moves(from_square: usize) -> BitB64 {
    // let mut moves = 0u64;
    // for i in 0..8 {
    //     for j in 0..8 {
    //         if i != j {
    //             moves |= 1u64 << (from_square + i * 8 + j);
    //         }
    //     }
    // }
    // Not implemented.
    0u64
}

pub fn generate_bishop_moves(pos: &Position, color: PlayerColor) -> Vec<BitboardMove> {
    // let own_pieces = pos.get_pieces(color);
    // let opponent_pieces = pos.get_pieces(color.opposite());
    // let empty_squares = !(own_pieces | opponent_pieces);
    // let mut moves = vec![];
    // for from_square in 0..64 {
    //     let from_bit = 1u64 << from_square;
    //     if own_pieces & from_bit != 0 {
    //         let possible_moves = bishop_moves(from_square) & empty_squares;
    //         for to_square in 0..64 {
    //             let to_bit = 1u64 << to_square;
    //             if possible_moves & to_bit != 0 {
    //                 moves.push((from_square, to_square));
    //             }
    //         }
    //     }
    // }
    // moves
    vec![]
}
