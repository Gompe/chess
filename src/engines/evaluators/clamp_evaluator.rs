use crate::engines::engine_traits::*;

use crate::chess_server::chess_types::ChessBoard;

use ordered_float::OrderedFloat;

#[derive(Clone)]
pub struct ClampEvaluator<E: Evaluator> {
    evaluator: E,
    threshold: OrderedFloat<f64>,
}

unsafe impl<E: Evaluator> Send for ClampEvaluator<E> where E: Send {}

unsafe impl<E: Evaluator> Sync for ClampEvaluator<E> where E: Sync {}

impl<E: Evaluator> ClampEvaluator<E> {
    pub fn new(evaluator: E, threshold: OrderedFloat<f64>) -> ClampEvaluator<E> {
        ClampEvaluator {
            evaluator,
            threshold,
        }
    }
}

impl<E: Evaluator> Evaluator for ClampEvaluator<E> {
    fn get_name(&self) -> String {
        format!("ClampEvaluator({})", self.evaluator.get_name())
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        let eval = self.evaluator.evaluate(chess_board);

        OrderedFloat((eval / self.threshold).tanh())
    }
}
