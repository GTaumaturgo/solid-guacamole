use crate::bishop::*;
use crate::rook::*;
use crate::bitboard::chess::*;
pub fn generate_queen_moves(board: &Chessboard, color: Color) -> Vec<(usize, usize)> {
    generate_bishop_moves(board, color) | generate_rook_moves(board, color);
}
