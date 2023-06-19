pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use crate::chess::position::Position;
use crate::chess::{
    bitboard::{BitB64, BitboardMove},
    PieceType,
};
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct PieceAndMoves {
    pub typpe: PieceType,
    pub moves: BitB64,
}
pub type MovesMap = HashMap<u8, PieceAndMoves>;

// Merges two move maps. The second one is borrowed and freed, the first one lives.
pub fn merge_moves_map(input: MovesMap, output: &mut MovesMap) {
    for (sq_id, input_pc_and_moves) in input.iter() {
        if let Some(output_pc_and_moves) = output.get_mut(sq_id) {
            output_pc_and_moves.moves |= input_pc_and_moves.moves;
            continue; 
        }
        output.insert(*sq_id, *input_pc_and_moves);
    }
    println!("sucessfully merged move map!");
}

// pub trait MoveGenerator {
//     fn generate_moves(&self, position: &Position) -> Vec<BitboardMove>;
// }
