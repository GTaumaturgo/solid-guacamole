use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::PlayerColor;

fn knight_moves(from_square: usize) -> BitB64 {
    let mut moves = 0u64;
    moves |= 1u64 << (from_square + 6); // 1 * 8 - 2 = 6
    moves |= 1u64 << (from_square - 6); // -1 * 8 + 2 = -6
    moves |= 1u64 << (from_square + 15); // 2 * 8 - 1 = 15
    moves |= 1u64 << (from_square - 15); // -2 * 8 + 1 = -15
    moves |= 1u64 << (from_square + 17); // 2 * 8 + 1 = 17
    moves |= 1u64 << (from_square - 17); // -2 * 8 - 1 = -17
    moves |= 1u64 << (from_square + 10); // 1 * 8 + 2 = 10
    moves |= 1u64 << (from_square - 10); // -1 * 8 - 2 = -10
    moves
}

pub fn generate_knight_moves(pos: &Position, color: PlayerColor) -> Vec<BitboardMove> {
    // let own_pieces = pos.get_pieces(color);
    // let opponent_pieces = pos.get_pieces(color.opposite());
    // let empty_squares = !(own_pieces | opponent_pieces);
    // let mut moves = vec![];
    // for from_square in 0..64 {
    //     let from_bit = 1u64 << from_square;
    //     if own_pieces & from_bit != 0 {
    //         let possible_moves = knight_moves(from_square) & empty_squares;
    //         for to_square in 0..64 {
    //             let to_bit = 1u64 << to_square;
    //             if possible_moves & to_bit != 0 {
    //                 moves.push((from_square, to_square));
    //             }
    //         }
    //     }
    // }
    vec![]
}
