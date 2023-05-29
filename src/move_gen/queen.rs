use crate::chess::bitboard::{BitB64, BitboardMove};
use crate::chess::position::Position;
use crate::chess::PlayerColor;
use crate::move_gen::bishop::*;
use crate::move_gen::rook::*;

pub fn generate_queen_moves(board: &Position, color: PlayerColor) -> Vec<BitboardMove> {
    // let bishop_moves = generate_bishop_moves(board, color);
    // let rook_moves = generate_rook_moves(board, color);
    
    // let mut moves = bishop_moves;
    // moves.extend(rook_moves);
    // moves
    vec![]
}
