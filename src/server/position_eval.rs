use rocket::{figment::providers::Format, State};

use crate::{
    chess::{
        bitboard::{BitArraySize, BitB64, PlayerBitboard, SpecialMoveType, FULL_BOARD},
        position::Position,
    },
    move_gen::PieceAndMoves,
    UciRequest, UciResponse,
};

use crate::evaluation::checkmate_evaluator::CheckmateEvaluator;
use crate::evaluation::material_evaluator::MaterialEvaluator;
use crate::evaluation::piece_coordinate_evaluator::PieceCoordinateEvaluator;
use crate::evaluation::search_evaluator::MinimaxSearchEvaluator;

use crate::evaluation::{PositionEvaluationPipeline, PositionEvaluator, PositionEvaluatorType};

pub async fn handle_position_eval_request(uci_req: &UciRequest) -> UciResponse {
    let eval_pipeline = PositionEvaluationPipeline {
        evaluators: vec![
            PositionEvaluatorType::Material(MaterialEvaluator::new()),
            PositionEvaluatorType::PieceCoordinate(PieceCoordinateEvaluator::new()),
            // PositionEvaluatorType::Checkmate(CheckmateEvaluator {}),
        ],
    };

    let position = Position::from_uci(uci_req);

    let minimax_evaluator = MinimaxSearchEvaluator::new(eval_pipeline, 5);

    let score = minimax_evaluator.evaluate(&position);
    UciResponse {
        best_moves: "".to_string(),
        possible_moves: "".to_string(),
        pos_score: score.to_string(),
    }
}
