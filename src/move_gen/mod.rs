pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;


use crate::chess::position::{Position};
use crate::chess::bitboard::{BitboardMove};


pub trait MoveGenerator {
    fn generate_moves(&self, position: &Position) -> Vec<BitboardMove>;
}