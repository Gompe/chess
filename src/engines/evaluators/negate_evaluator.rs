use std::cell::RefCell;
use crate::engines::engine_traits::*;

use crate::chess_server::chess_types::ChessBoard;

use ordered_float::OrderedFloat;

#[derive(Clone)]
pub struct NegateEvaluator<E: Evaluator> {
    evaluator: E,
}

unsafe impl<E: Evaluator> Send for NegateEvaluator<E> 
where E: Send {}

unsafe impl<E: Evaluator> Sync for NegateEvaluator<E> 
where E: Sync {}

impl<E: Evaluator> NegateEvaluator<E> {
    pub fn new(evaluator: E) -> NegateEvaluator<E> {
        NegateEvaluator { 
            evaluator,
        }
    }
}

impl<E: Evaluator> Evaluator for NegateEvaluator<E> {

    fn get_name(&self) -> String {
        format!("NegateEvaluator({})", self.evaluator.get_name())
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        -self.evaluator.evaluate(chess_board)
    }
}

