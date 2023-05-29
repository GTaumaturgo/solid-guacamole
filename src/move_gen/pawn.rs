use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::PlayerColor;

pub fn generate_pawn_moves(board: &Position, color: PlayerColor) -> Vec<BitboardMove> {
    let (own_pieces, opponent_pieces, pawns) = match color {
        PlayerColor::White => (
            board.white.pawns
                | board.white.knights
                | board.white.bishops
                | board.white.rooks
                | board.white.queens
                | board.white.king,
            board.black.pawns
                | board.black.knights
                | board.black.bishops
                | board.black.rooks
                | board.black.queens
                | board.black.king,
            board.white.pawns,
        ),
        PlayerColor::Black => (
            board.black.pawns
                | board.black.knights
                | board.black.bishops
                | board.black.rooks
                | board.black.queens
                | board.black.king,
            board.white.pawns
                | board.white.knights
                | board.white.bishops
                | board.white.rooks
                | board.white.queens
                | board.white.king,
            board.black.pawns,
        ),
    };

    let empty_squares = !(own_pieces | opponent_pieces);
    let rank_2 = 0x0000_0000_0000_FF00u64;
    let rank_7 = 0x00FF_0000_0000_0000u64;

    let mut moves = vec![];

    // One square forward
    let one_step = match color {
        PlayerColor::White => (pawns << 8) & empty_squares,
        PlayerColor::Black => (pawns >> 8) & empty_squares,
    };

    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if pawns & from_bit != 0 {
            let to_bit = match color {
                PlayerColor::White => from_bit << 8,
                PlayerColor::Black => from_bit >> 8,
            };

            if to_bit & one_step != 0 {
                let to_square = from_square as isize
                    + match color {
                        PlayerColor::White => 8,
                        PlayerColor::Black => -8,
                    };

                moves.push((from_square, to_square as usize));

                // Check for promotions
                if (to_bit & (rank_7 | rank_2)) != 0 {
                    moves.pop();
                    moves.push((from_square, to_square as usize));
                    moves.push((from_square, to_square as usize));
                    moves.push((from_square, to_square as usize));
                    moves.push((from_square, to_square as usize));
                }
            }
        }
    }

    // Two squares forward
    let two_steps = match color {
        PlayerColor::White => (one_step & rank_2) << 8,
        PlayerColor::Black => (one_step & rank_7) >> 8,
    };

    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if pawns & from_bit != 0 {
            let to_bit = match color {
                PlayerColor::White => from_bit << 16,
                PlayerColor::Black => from_bit >> 16,
            };

            if to_bit & two_steps != 0 {
                let to_square = from_square as isize
                    + match color {
                        PlayerColor::White => 16,
                        PlayerColor::Black => -16,
                    };

                moves.push((from_square, to_square as usize));
            }
        }
    }

    // Captures
    let left_captures = match color {
        PlayerColor::White => (pawns & 0xFEFEFEFEFEFEFEFEu64) << 7,
        PlayerColor::Black => (pawns & 0xFEFEFEFEFEFEFEFEu64) >> 9,
    };
    let right_captures = match color {
        PlayerColor::White => (pawns & 0x7F7F7F7F7F7F7F7Fu64) << 9,
        PlayerColor::Black => (pawns & 0x7F7F7F7F7F7F7F7Fu64) >> 7,
    };
    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if pawns & from_bit != 0 {
            let left_to_bit = match color {
                PlayerColor::White => from_bit << 7,
                PlayerColor::Black => from_bit >> 9,
            };
            let right_to_bit = match color {
                PlayerColor::White => from_bit << 9,
                PlayerColor::Black => from_bit >> 7,
            };

            if left_to_bit & left_captures & opponent_pieces != 0 {
                let to_square = from_square as isize
                    + match color {
                        PlayerColor::White => 7,
                        PlayerColor::Black => -9,
                    };
                moves.push((from_square, to_square as usize));
            }

            if right_to_bit & right_captures & opponent_pieces != 0 {
                let to_square = from_square as isize
                    + match color {
                        PlayerColor::White => 9,
                        PlayerColor::Black => -7,
                    };
                moves.push((from_square, to_square as usize));
            }
        }
    }
    vec![]
}
