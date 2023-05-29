mod move_search;

use bitboard::chess;

mod move_search {
    trait MoveSearch {
        pub fn do_search(&self, position: Position, usize: topk) -> Vec<ScoredMove>;
    }
}
