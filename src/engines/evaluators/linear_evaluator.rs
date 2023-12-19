use crate::engines::engine_traits::*;

use crate::chess_server::chess_types::ChessBoard;
use ordered_float::OrderedFloat;

#[derive(Clone)]
pub struct LinearEvaluator<E1: Evaluator, E2: Evaluator> {
    evaluator_1: E1,
    evaluator_2: E2,

    coef: [f64; 2],
}

unsafe impl<E1: Evaluator, E2: Evaluator> Send for LinearEvaluator<E1, E2>
where
    E1: Send,
    E2: Send,
{
}

unsafe impl<E1: Evaluator, E2: Evaluator> Sync for LinearEvaluator<E1, E2>
where
    E1: Sync,
    E2: Sync,
{
}

impl<E1: Evaluator, E2: Evaluator> LinearEvaluator<E1, E2> {
    pub fn new(evaluator_1: E1, evaluator_2: E2, coef: [f64; 2]) -> LinearEvaluator<E1, E2> {
        LinearEvaluator {
            evaluator_1,
            evaluator_2,
            coef,
        }
    }
}

impl<E1: Evaluator, E2: Evaluator> Evaluator for LinearEvaluator<E1, E2> {
    fn get_name(&self) -> String {
        format!(
            "LinearEvaluator({}, {})",
            self.evaluator_1.get_name(),
            self.evaluator_2.get_name()
        )
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        let eval_1 = self.evaluator_1.evaluate(chess_board);
        let eval_2 = self.evaluator_2.evaluate(chess_board);

        OrderedFloat(self.coef[0]) * eval_1 + OrderedFloat(self.coef[1]) * eval_2
    }
}
