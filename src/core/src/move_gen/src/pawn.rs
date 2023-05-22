use crate::chessboard::Chessboard;
use crate::color::Color;

pub fn generate_pawn_moves(board: &Chessboard, color: Color) -> Vec<(usize, usize)> {
    let (own_pieces, opponent_pieces, pawns) = match color {
        Color::White => (
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
        Color::Black => (
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
        Color::White => (pawns << 8) & empty_squares,
        Color::Black => (pawns >> 8) & empty_squares,
    };

    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if pawns & from_bit != 0 {
            let to_bit = match color {
                Color::White => from_bit << 8,
                Color::Black => from_bit >> 8,
            };

            if to_bit & one_step != 0 {
                let to_square = from_square as isize
                    + match color {
                        Color::White => 8,
                        Color::Black => -8,
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
        Color::White => (one_step & rank_2) << 8,
        Color::Black => (one_step & rank_7) >> 8,
    };

    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if pawns & from_bit != 0 {
            let to_bit = match color {
                Color::White => from_bit << 16,
                Color::Black => from_bit >> 16,
            };

            if to_bit & two_steps != 0 {
                let to_square = from_square as isize
                    + match color {
                        Color::White => 16,
                        Color::Black => -16,
                    };

                moves.push((from_square, to_square as usize));
            }
        }
    }

    // Captures
    let left_captures = match color {
        Color::White => (pawns & 0xFEFEFEFEFEFEFEFEu64) << 7,
        Color::Black => (pawns & 0xFEFEFEFEFEFEFEFEu64) >> 9,
    };
    let right_captures = match color {
        Color::White => (pawns & 0x7F7F7F7F7F7F7F7Fu64) << 9,
        Color::Black => (pawns & 0x7F7F7F7F7F7F7F7Fu64) >> 7,
    };
    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if pawns & from_bit != 0 {
            let left_to_bit = match color {
                Color::White => from_bit << 7,
                Color::Black => from_bit >> 9,
            };
            let right_to_bit = match color {
                Color::White => from_bit << 9,
                Color::Black => from_bit >> 7,
            };

            if left_to_bit & left_captures & opponent_pieces != 0 {
                let to_square = from_square as isize
                    + match color {
                        Color::White => 7,
                        Color::Black => -9,
                    };
                moves.push((from_square, to_square as usize));
            }

            if right_to_bit & right_captures & opponent_pieces != 0 {
                let to_square = from_square as isize
                    + match color {
                        Color::White => 9,
                        Color::Black => -7,
                    };
                moves.push((from_square, to_square as usize));
            }
        }
    }
    moves
}
