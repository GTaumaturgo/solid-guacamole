use super::internal::test_utils::{self, *};
use crate::move_gen::{queen::QueenBitboardMoveGenerator, BitboardMoveGenerator};

use std::collections::HashMap;

#[test]
fn test_initial_pos() {
    // Initial position has no moves for bishops.
    let pos = test_utils::get_initial_position();
    assert_eq!(
        HashMap::new(),
        QueenBitboardMoveGenerator::generate_moves(&pos)
    );
}
