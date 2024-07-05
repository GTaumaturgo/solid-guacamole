pub mod material_evaluator;
pub mod piece_coordinate_evaluator;
pub mod checkmate_evaluator;



pub mod search_evaluator;
use crate::chess::position::Position;

pub trait PositionEvaluator {
    fn evaluate(&self, position: &Position) -> i32;
}

pub struct PositionEvaluationPipeline {
    pub evaluators: Vec<Box<dyn PositionEvaluator>>,
}

impl PositionEvaluator for PositionEvaluationPipeline {
    fn evaluate(&self, position: &Position) -> i32 {
        let mut sum = 0;
        for evaluator in &self.evaluators {
            sum += evaluator.evaluate(position);
        }
        sum
    }
}
