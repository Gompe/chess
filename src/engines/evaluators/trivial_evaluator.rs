use crate::chess_server::chess_types::ChessBoard;
use crate::engines::engine_traits::*;

use ordered_float::OrderedFloat;

#[derive(Clone)]
pub struct TrivialEvaluator;

impl TrivialEvaluator {
    pub fn new() -> TrivialEvaluator {
        TrivialEvaluator {}
    }
}

unsafe impl Send for TrivialEvaluator {}
unsafe impl Sync for TrivialEvaluator {}

impl Evaluator for TrivialEvaluator {
    fn get_name(&self) -> String {
        "TrivialEvaluator".to_string()
    }

    #[inline(always)]
    fn evaluate(&self, _chess_board: &ChessBoard) -> OrderedFloat<f64> {
        OrderedFloat(0.)
    }
}
