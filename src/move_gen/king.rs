use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::PlayerColor;

fn king_moves(from_square: usize) -> BitB64 {
    let mut moves = 0u64;
    // for i in [-1, 1].iter() {
    //     for j in [-1, 0, 1].iter() {
    //         if *i != 0 || *j != 0 {
    //             moves |= 1u64 << (from_square + i * 8 + j);
    //         }
    //     }
    // }
    moves
}

pub fn generate_king_moves(pos: &Position, color: PlayerColor) -> Vec<BitboardMove> {
    let white = pos.white_pieces();
    let black = pos.black_pieces();

    let (own_pieces, opponent_pieces) = match color {
        PlayerColor::White => (white, black),
        PlayerColor::Black => (black, white),
    };

    let own_union = own_pieces.king
        | own_pieces.queens
        | own_pieces.rooks
        | own_pieces.knights
        | own_pieces.bishops
        | own_pieces.pawns;
    let opponent_union = opponent_pieces.king
        | opponent_pieces.queens
        | opponent_pieces.rooks
        | opponent_pieces.knights
        | opponent_pieces.bishops
        | opponent_pieces.pawns;
    let king = own_pieces.king;
    let empty_squares = !(own_union | opponent_union);
    let mut moves = vec![];
    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if king & from_bit != 0 {
            let possible_moves = king_moves(from_square) & !(own_union);
            for to_square in 0..64 {
                let to_bit = 1u64 << to_square;
                if possible_moves & to_bit != 0 {
                    moves.push((from_square, to_square));
                }
            }
        }
    }
    vec![]
}
