use crate::search::move_search;

mod alphabeta_minimax {
     struct AlphaBetaMinimaxSearch {
        depth: usize,
    }

    impl MoveSearch for AlphaBetaMinimaxSearch {
        fn do_search(&self, position: Position, usize: topk) -> Vec<ScoredMove> {
            let mut moves = Vec::new();
            for mv in position.legal_moves() {
                let mut child_position = position.clone();
                child_position.make_move(mv);
                
                let score = self.alpha_beta_search(child_position, self.depth - 1, -std::i32::MAX, std::i32::MAX);

                moves.push(ScoredMove {
                    mv,
                    score,
                });
            }

            moves.sort_by(|a, b| a.score.cmp(&b.score));
            moves.truncate(topk);

            moves
        }

        fn alpha_beta_search(&self, position: Position, depth: usize, alpha: i32, beta: i32) -> i32 {
            if depth == 0 {
                return position.evaluate();
            }

            let mut best_score = -std::i32::MAX;

            for mv in position.legal_moves() {
                let mut child_position = position.clone();
                child_position.make_move(mv);

                let score = self.alpha_beta_search(child_position, depth - 1, alpha, beta);

                best_score = std::cmp::max(best_score, score);

                if best_score >= beta {
                    return best_score;
                }

                alpha = std::cmp::max(alpha, best_score);
            }

            best_score
        }
    }   
}