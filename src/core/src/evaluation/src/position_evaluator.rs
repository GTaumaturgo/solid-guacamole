use crate::bitboard::chess::*;

trait PositionEvaluator {
    fn evaluate(&self, position: Position) -> i32;
}
trait CompositeEvaluator {
    fn new(leaf_evaluators: Vec<Box<dyn PositionEvaluator>>) -> Self;
    fn evaluate(&self, position: Position) -> chess::PositionScore;
}