pub mod knight;
pub mod bishop;
pub mod king;
pub mod queen;
pub mod pawn;
pub mod rook;


use crate::bitboard;
pub trait MoveGenerator {
    fn generate_moves(&self, position: &chess::Position) -> Vec<chess::Move>;
}