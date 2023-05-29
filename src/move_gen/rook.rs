use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::PlayerColor;

fn rook_moves(from_square: usize) -> BitB64 {
    let mut moves = 0u64;

    for i in 0..8 {
        moves |= 1u64 << (from_square + i * 8);
        moves |= 1u64 << (from_square - i * 8);
    }

    for j in 0..8 {
        moves |= 1u64 << (from_square + j);
        moves |= 1u64 << (from_square - j);
    }

    moves
}

pub fn generate_rook_moves(board: &Position, color: PlayerColor) -> Vec<BitboardMove> {
    // let own_pieces = board.get_pieces(color);
    // let opponent_pieces = board.get_pieces(color.opposite());

    // let empty_squares = !(own_pieces | opponent_pieces);

    // let mut moves = vec![];

    // for from_square in 0..64 {
    //     let from_bit = 1u64 << from_square;
    //     if own_pieces & from_bit != 0 {
    //         let possible_moves = rook_moves(from_square) & empty_squares;
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
