use crate::search::move_search;

struct MinimaxSearch {
    depth: usize,
}

impl MoveSearch for MinimaxSearch {
    fn do_search(&self, position: Position, usize: topk) -> Vec<ScoredMove> {
        let mut moves = Vec::new();
        for mv in position.legal_moves() {
            let mut child_position = position.clone();
            child_position.make_move(mv);

            let score = self.minimax_search(child_position, self.depth - 1);

            moves.push(ScoredMove { mv, score });
        }

        moves.sort_by(|a, b| a.score.cmp(&b.score));
        moves.truncate(topk);

        moves
    }

    fn minimax_search(&self, position: Position, depth: usize) -> i32 {
        if depth == 0 {
            return position.evaluate();
        }

        let mut best_score = -std::i32::MAX;

        for mv in position.legal_moves() {
            let mut child_position = position.clone();
            child_position.make_move(mv);

            let score = self.alpha_beta_search(child_position, depth - 1);
            best_score = std::cmp::max(best_score, score);
        }
        best_score
    }
}
