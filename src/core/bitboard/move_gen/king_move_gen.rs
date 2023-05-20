mod king_move_gen;

mod king_move_gen {
fn king_moves(from_square: usize) -> u64 {
    let mut moves = 0u64;
    for i in [-1, 1].iter() {
        for j in [-1, 0, 1].iter() {
            if *i != 0 || *j != 0 {
                moves |= 1u64 << (from_square + i * 8 + j);
            }
        }
    }
    moves
}

pub fn generate_king_moves(board: &Chessboard, color: Color) -> Vec<(usize, usize)> {
    let (own_pieces, opponent_pieces, king) = match color {
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
            board.white.king,
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
            board.black.king,
        ),
    };
    let empty_squares = !(own_pieces | opponent_pieces);
    let mut moves = vec![];
    for from_square in 0..64 {
        let from_bit = 1u64 << from_square;
        if king & from_bit != 0 {
            let possible_moves = king_moves(from_square) & !(own_pieces);
            for to_square in 0..64 {
                let to_bit = 1u64 << to_square;
                if possible_moves & to_bit != 0 {
                    moves.push((from_square, to_square));
                }
            }
        }
    }
    moves
}
}