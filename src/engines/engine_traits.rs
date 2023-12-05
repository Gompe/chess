
use crate::chess_server::chess_types::chess_board::{MoveContainer, MOVE_CONTAINER_SIZE};
use crate::chess_server::game::Player;
use crate::chess_server::chess_types::{ChessBoard, Move, chess_board};

use ordered_float::OrderedFloat;
use smallvec::SmallVec;

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


/// Monte Carlo Tree Search
pub trait Policy {
    fn get_priors(&self, chess_board: &ChessBoard, moves: &MoveContainer) -> SmallVec<[f64; MOVE_CONTAINER_SIZE]>;
}
