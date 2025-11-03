pub mod checkmate_evaluator;
pub mod material_evaluator;
pub mod piece_coordinate_evaluator;

pub mod search_evaluator;
use checkmate_evaluator::CheckmateEvaluator;
use material_evaluator::MaterialEvaluator;
use piece_coordinate_evaluator::PieceCoordinateEvaluator;

use crate::chess::position::Position;

pub trait PositionEvaluator {
    async fn evaluate(&self, position: &Position) -> i32;
}
pub enum PositionEvaluatorType {
    Material(MaterialEvaluator),
    PieceCoordinate(PieceCoordinateEvaluator),
    Checkmate(CheckmateEvaluator),
}

impl PositionEvaluatorType {
    async fn evaluate(&self, position: &Position) -> i32 {
        match self {
            PositionEvaluatorType::Material(evaluator) => evaluator.evaluate(position).await,
            PositionEvaluatorType::PieceCoordinate(evaluator) => evaluator.evaluate(position).await,
            PositionEvaluatorType::Checkmate(evaluator) => evaluator.evaluate(position).await,
        }
    }
}

pub struct PositionEvaluationPipeline {
    pub evaluators: Vec<PositionEvaluatorType>,
}

impl PositionEvaluationPipeline {
    pub async fn evaluate(&self, position: &Position) -> i32 {
        let mut sum = 0;
        for evaluator in &self.evaluators {
            sum += (evaluator.evaluate(position)).await;
        }
        sum
    }
}
