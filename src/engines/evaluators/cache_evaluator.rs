use crate::engines::engine_traits::*;
use std::cell::RefCell;

use crate::backend::ChessBoard;
use crate::engines::zobrist_hash::ZobristHashMap;

use ordered_float::OrderedFloat;

#[derive(Clone)]
pub struct CacheEvaluator<E: Evaluator> {
    evaluator: E,
    cache: RefCell<ZobristHashMap<OrderedFloat<f64>>>,
}

unsafe impl<E: Evaluator> Send for CacheEvaluator<E> where E: Send {}

impl<E: Evaluator> CacheEvaluator<E> {
    pub fn new(evaluator: E) -> CacheEvaluator<E> {
        CacheEvaluator {
            evaluator,
            cache: RefCell::new(ZobristHashMap::new()),
        }
    }
}

impl<E: Evaluator> Evaluator for CacheEvaluator<E> {
    fn get_name(&self) -> String {
        format!("CacheEvaluator({})", self.evaluator.get_name())
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        if let Some(&eval) = self.cache.borrow().get_key_value(chess_board) {
            return eval;
        }

        let eval = self.evaluator.evaluate(chess_board);
        self.cache.borrow_mut().insert(chess_board, eval);
        eval
    }
}
