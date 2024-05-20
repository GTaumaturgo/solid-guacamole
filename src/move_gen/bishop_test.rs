use super::{
    bishop,
    internal::test_utils::{self, *},
    Position,
};
use crate::move_gen::{bishop::BishopBitboardMoveGenerator, BitboardMoveGenerator};

use std::collections::HashMap;

#[test]
fn test_initial_pos() {
    // Initial position has no moves for bishops.
    let pos = test_utils::get_initial_position();
    assert_eq!(
        HashMap::new(),
        BishopBitboardMoveGenerator::generate_moves(&pos)
    );
}
