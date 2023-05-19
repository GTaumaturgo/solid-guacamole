use crate::common::Color;

// Bitboard with information about the pieces of one color.
pub struct Bitboard {
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    king: u64,
}

// 
pub struct Position {
    white: Bitboard,
    black: Bitboard,
    // This contains information about the position, such as castling rights, en passant square, etc.
    // bits [0-7] refer to whites en passant rights
    // bits [8-15] refer to blacks en passant rights
    // bits [16-17] are whites short and long castling rights
    // bits [18-19] are blacks short and long castling rights
    // bit 20 indicates whose turn it is, 0 is white and 1 is black.
    position_info: i32,
}

const fn knight_moves(square: usize) -> u64 {
    let mut moves = 0;

    let sq_bit = 1u64 << square;
    let not_a_file = 0xFEFEFEFEFEFEFEFEu64;
    let not_ab_file = 0xFCFCFCFCFCFCFCFCu64;
    let not_h_file = 0x7F7F7F7F7F7F7F7Fu64;
    let not_gh_file = 0x3F3F3F3F3F3F3F3Fu64;

    moves |= (sq_bit & not_a_file) << 15;
    moves |= (sq_bit & not_ab_file) << 6;
    moves |= (sq_bit & not_h_file) << 17;
    moves |= (sq_bit & not_gh_file) << 10;

    moves |= (sq_bit & not_a_file) >> 17;
    moves |= (sq_bit & not_ab_file) >> 10;
    moves |= (sq_bit & not_h_file) >> 15;
    moves |= (sq_bit & not_gh_file) >> 6;

    moves
}

fn generate_rook_moves(board: &Position, color: Color) -> Vec<(usize, usize)> {
    let (own_pieces, opponent_pieces, rooks) = match color {
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
            board.white.rooks,
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
            board.black.rooks,
        ),
    };
    let empty_squares = !(own_pieces | opponent_pieces);

    let mut moves = vec![];

    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if rooks & from_bit != 0 {
            for direction in &[8, 1, -8, -1] {
                let mut to_square = from_square as isize + direction;
                while to_square >= 0 && to_square < 64 {
                    let to_bit = 1u64 << to_square;

                    // Stop sliding when hitting own piece
                    if to_bit & own_pieces != 0 {
                        break;
                    }

                    moves.push((from_square, to_square as usize));

                    // Stop sliding after capturing opponent piece
                    if to_bit & opponent_pieces != 0 {
                        break;
                    }

                    to_square += direction;
                }
            }
        }
    }
    moves
}

fn generate_bishop_moves(board: &Position, color: Color) -> Vec<(usize, usize)> {
    let (own_pieces, opponent_pieces, bishops) = match color {
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
            board.white.bishops,
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
            board.black.bishops,
        ),
    };
    let empty_squares = !(own_pieces | opponent_pieces);

    let mut moves = vec![];

    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if bishops & from_bit != 0 {
            for direction in &[7, 9, -7, -9] {
                let mut to_square = from_square as isize + direction;
                while to_square >= 0 && to_square < 64 {
                    let to_bit = 1u64 << to_square;

                    // Stop sliding when hitting own piece
                    if to_bit & own_pieces != 0 {
                        break;
                    }

                    moves.push((from_square, to_square as usize));

                    // Stop sliding after capturing opponent piece
                    if to_bit & opponent_pieces != 0 {
                        break;
                    }

                    to_square += direction;
                }
            }
        }
    }

    moves
}

fn generate_queen_moves(board: &Position, color: Color) -> Vec<(usize, usize)> {
    let mut moves = vec![];

    let rook_moves = generate_rook_moves(board, color);
    let bishop_moves = generate_bishop_moves(board, color);

    moves.extend(rook_moves);
    moves.extend(bishop_moves);

    moves
}
