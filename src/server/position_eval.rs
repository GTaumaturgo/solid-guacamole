use rocket::figment::providers::Format;

use crate::{
    chess::{
        bitboard::{BitArraySize, BitB64, PlayerBitboard, SpecialMoveType, FULL_BOARD},
        position::Position,
    },
    move_gen::PieceAndMoves,
    UciRequest, UciResponse,
};

use crate::evaluation::material_evaluator::MaterialEvaluator;
use crate::evaluation::piece_coordinate_evaluator::PieceCoordinateEvaluator;

use crate::evaluation::{
    PositionEvaluationPipeline, PositionEvaluationPipelineInterface, PositionEvaluator,
};

pub fn handle_position_eval_request(uci_req: &UciRequest) -> UciResponse {
    let eval_pipeline = PositionEvaluationPipeline {
        evaluators: vec![
            Box::new(MaterialEvaluator::new()),
            Box::new(PieceCoordinateEvaluator::new()),
        ],
    };

    let mut position = Position::from_uci(uci_req);

    let score = eval_pipeline.evaluate(&position);
    UciResponse {
        best_moves: "".to_string(),
        possible_moves: "".to_string(),
        pos_score: score.to_string(),
    }
}
