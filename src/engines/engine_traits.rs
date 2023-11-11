
use crate::chess_server::game::Player;
use crate::chess_server::types::{ChessBoard, Move};

use ordered_float::OrderedFloat;

pub const EVAL_WHITE_WON: OrderedFloat<f64> = OrderedFloat(1000.);
pub const EVAL_BLACK_WON: OrderedFloat<f64> = OrderedFloat(-1000.);
pub const EVAL_DRAW: OrderedFloat<f64> = OrderedFloat(0.);

pub trait Evaluator {
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64>;
}

pub trait Searcher<E: Evaluator> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E) -> Move;
}

pub struct SearcherEngine<E: Evaluator, S: Searcher<E>> {
    evaluator: E,
    searcher: S
}

impl<E: Evaluator, S: Searcher<E>> SearcherEngine<E,S> {
    pub fn new(evaluator: E, searcher: S) -> SearcherEngine<E, S> {
        SearcherEngine {evaluator, searcher}
    }
}

impl<E: Evaluator, S: Searcher<E>> Player for SearcherEngine<E, S> {
    fn select_move(&self, chess_board: &ChessBoard) -> Move {
        self.searcher.search(chess_board, &self.evaluator)
    }
}