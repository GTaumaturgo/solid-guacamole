use super::{
    internal::test_utils::{self, *},
    king, Position,
};

use crate::chess::{
    bitboard::{
        BitArraySize, A2, A3, A4, B1, B2, B3, B4, C2, C3, C4, D2, D3, D4, E2, E3, E4, F2, F3, F4,
        G1, G2, G3, G4, H2, H3, H4,
    },
    PieceType,
};
use crate::move_gen::{
    pawn::PawnBitboardMoveGenerator, BitboardMoveGenerator, MovesMap, PieceAndMoves,
};

use std::collections::HashMap;

#[test]
fn test_initial_pos() {
    // Initial position has no moves for bishops.
    let pos = test_utils::get_initial_position();
    assert_eq!(
        HashMap::from([
            (
                A2,
                PieceAndMoves {
                    typpe: PieceType::Pawn,
                    moves: u64::nth(A3) | u64::nth(A4),
                }
            ),
            (
                B2,
                PieceAndMoves {
                    typpe: PieceType::Pawn,
                    moves: u64::nth(B3) | u64::nth(B4),
                }
            ),
            (
                C2,
                PieceAndMoves {
                    typpe: PieceType::Pawn,
                    moves: u64::nth(C3) | u64::nth(C4),
                }
            ),
            (
                D2,
                PieceAndMoves {
                    typpe: PieceType::Pawn,
                    moves: u64::nth(D3) | u64::nth(D4),
                }
            ),
            (
                E2,
                PieceAndMoves {
                    typpe: PieceType::Pawn,
                    moves: u64::nth(E3) | u64::nth(E4),
                }
            ),
            (
                F2,
                PieceAndMoves {
                    typpe: PieceType::Pawn,
                    moves: u64::nth(F3) | u64::nth(F4),
                }
            ),
            (
                G2,
                PieceAndMoves {
                    typpe: PieceType::Pawn,
                    moves: u64::nth(G3) | u64::nth(G4),
                }
            ),
            (
                H2,
                PieceAndMoves {
                    typpe: PieceType::Pawn,
                    moves: u64::nth(H3) | u64::nth(H4),
                }
            ),
        ]),
        PawnBitboardMoveGenerator::generate_moves(&pos)
    );
}
