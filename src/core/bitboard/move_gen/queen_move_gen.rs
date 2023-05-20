mod queen_move_gen;

use crate::bishop_move_gen;
use crate::rook_move_gen;

mod queen_move_gen {
    pub fn generate_queen_moves(board: &Chessboard, color: Color) -> Vec<(usize, usize)> {
        generate_bishop_moves(board, color) | generate_rook_moves(board, color);
    }
}