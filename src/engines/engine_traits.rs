use crate::backend::chess_board::{MoveContainer, MOVE_CONTAINER_SIZE};
use crate::backend::{ChessBoard, Move};
use crate::chess_server::game::Player;

use ordered_float::OrderedFloat;
use smallvec::SmallVec;

use std::time::Duration;

pub const EVAL_WHITE_WON: OrderedFloat<f64> = OrderedFloat(1000.);
pub const EVAL_BLACK_WON: OrderedFloat<f64> = OrderedFloat(-1000.);
pub const EVAL_DRAW: OrderedFloat<f64> = OrderedFloat(0.);

pub trait Evaluator {
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64>;

    // As method to allow for dynamic dispatch
    fn get_name(&self) -> String;
}

pub trait Searcher<E: Evaluator> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E) -> Move;
}

pub trait TimedSearcher<E: Evaluator> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E, avail_time: Duration) -> Option<Move>;
}

pub struct TimedSearcherWrapper<E: Evaluator> {
    timed_searcher: Box<dyn TimedSearcher<E>>,
    avail_time: Duration
}

impl<E: Evaluator> TimedSearcherWrapper<E> {
    pub fn new(timed_searcher: Box<dyn TimedSearcher<E>>, avail_time: Duration) -> Self {
        TimedSearcherWrapper { timed_searcher , avail_time }
    }
}

impl<E: Evaluator> Searcher<E> for TimedSearcherWrapper<E> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E) -> Move {
        self.timed_searcher.search(chess_board, evaluator, self.avail_time).unwrap()
    }
}

#[derive(Clone)]
pub struct SearcherEngine<E: Evaluator, S: Searcher<E>> {
    evaluator: E,
    searcher: S,
}

impl<E: Evaluator, S: Searcher<E>> SearcherEngine<E, S> {
    pub fn new(evaluator: E, searcher: S) -> SearcherEngine<E, S> {
        SearcherEngine {
            evaluator,
            searcher,
        }
    }
}

impl<E: Evaluator, S: Searcher<E>> Player for SearcherEngine<E, S> {
    fn select_move(&self, chess_board: &ChessBoard) -> Move {
        self.searcher.search(chess_board, &self.evaluator)
    }
}

/// Monte Carlo Tree Search
pub trait Policy {
    fn get_priors(
        &self,
        chess_board: &ChessBoard,
        moves: &MoveContainer,
    ) -> SmallVec<[f64; MOVE_CONTAINER_SIZE]>;
}
