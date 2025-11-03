use super::internal::test_utils::{self, *};
use crate::move_gen::{king::KingBitboardMoveGenerator, BitboardMoveGenerator};

#[test]
fn test_initial_pos() {
    // Initial position has no moves for bishops.
    let pos = test_utils::get_initial_position();
    assert_eq!(
        HashMap::new(),
        KingBitboardMoveGenerator::generate_moves(&pos)
    );
}
