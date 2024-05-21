use super::{
    internal::test_utils::{self, *},
    king, Position,
};

use crate::chess::{
    bitboard::{BitArraySize, A3, B1, C3, F3, G1, H3},
    PieceType,
};
use crate::move_gen::{
    knight::KnightBitboardMoveGenerator, BitboardMoveGenerator, MovesMap, PieceAndMoves,
};

use std::collections::HashMap;

#[test]
fn test_initial_pos() {
    // Initial position has no moves for bishops.
    let pos = test_utils::get_initial_position();
    assert_eq!(
        HashMap::from([
            (
                B1,
                PieceAndMoves {
                    typpe: PieceType::Knight,
                    moves: u64::nth(A3) | u64::nth(C3)
                }
            ),
            (
                G1,
                PieceAndMoves {
                    typpe: PieceType::Knight,
                    moves: u64::nth(F3) | u64::nth(H3)
                }
            ),
        ]),
        KnightBitboardMoveGenerator::generate_moves(&pos)
    );
}
