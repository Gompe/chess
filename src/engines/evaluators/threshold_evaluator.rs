use std::cell::RefCell;
use crate::engines::engine_traits::*;

use crate::chess_server::chess_types::ChessBoard;

use ordered_float::OrderedFloat;

const INF : OrderedFloat<f64> = OrderedFloat(1000.0);

pub struct ThresholdEvaluator<E: Evaluator> {
    evaluator: E,
    threshold: OrderedFloat<f64>
}

impl<E: Evaluator> ThresholdEvaluator<E> {
    pub fn new(evaluator: E, threshold: OrderedFloat<f64>) -> ThresholdEvaluator<E> {
        ThresholdEvaluator { 
            evaluator,
            threshold 
        }
    }
}

impl<E: Evaluator> Evaluator for ThresholdEvaluator<E> {

    fn get_name(&self) -> String {
        format!("ThresholdEvaluator({})", self.evaluator.get_name())
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {

        let eval = self.evaluator.evaluate(chess_board);

        if eval > self.threshold {
            INF
        } else if eval < -self.threshold {
            -INF
        } else {
            eval
        }
    }
}

