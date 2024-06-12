mod internal;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

mod bishop_test;
mod king_test;
mod knight_test;
mod pawn_test;
mod queen_test;
mod rook_test;

use crate::chess::bitboard::{BitArraySize, PlayerBitboard, SpecialMoveType};
use crate::chess::position::Position;
use crate::chess::{
    bitboard::{BitB64, BitboardMove},
    PieceType,
};

use std::cmp::Eq;

use internal::{intersect, is_inside_board};
use std::collections::HashMap;

// Public exports below.
// internal utilities on internal file
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PieceAndMoves {
    pub typpe: PieceType,
    pub moves: Vec<BitboardMove>,
}
pub type MovesMap = HashMap<u8, PieceAndMoves>;

pub enum MoveGenPerspective {
    MovingPlayer,
    WaitingPlayer,
}

pub struct MoveGenOpts {
    pub perspective: MoveGenPerspective,
}

pub trait BitboardMoveGenerator {
    fn get_raw_attacking_moves(pos: &Position, opts: MoveGenOpts) -> BitB64;

    fn generate_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap;

    fn get_attacking_moves(pos: &Position, opts: MoveGenOpts) -> MovesMap;
}

// Merges two move maps. The second one is borrowed and freed, the first one lives.
pub fn merge_moves_map(input: MovesMap, output: &mut MovesMap) {
    for (sq_id, input_pc_and_moves) in input.iter() {
        // Maybe extend an existing entry.
        if let Some(output_pc_and_moves) = output.get_mut(sq_id) {
            output_pc_and_moves
                .moves
                .extend(input_pc_and_moves.moves.iter());
        } else {
            // Create a new entry.
            output.insert(*sq_id, input_pc_and_moves.clone());
        }
    }
}

pub fn try_generate_move_in_direction(
    sq_id: i8,
    ally_pieces: &PlayerBitboard,
    enemy_pieces: &PlayerBitboard,
    is_direction_blocked: &mut bool,
    cur_bishop_moves: &mut u64,
) -> () {
    if !(*is_direction_blocked)
        && is_inside_board(
            internal::get_i_from_sq_id(sq_id),
            internal::get_j_from_sq_id(sq_id),
        )
    {
        let sq_in_direction = u64::nth(sq_id as u8);
        if intersect(sq_in_direction, ally_pieces.all_pieces()) {
            *is_direction_blocked = true;
        }
        if intersect(sq_in_direction, enemy_pieces.all_pieces()) {
            *cur_bishop_moves |= sq_in_direction;
            *is_direction_blocked = true;
        }
    }
    ()
}
