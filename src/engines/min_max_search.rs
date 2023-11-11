use crate::chess_server::types::Color;
use crate::chess_server::types::GameStatus;
use crate::engines::engine_traits::*;
use std::marker::PhantomData;

use crate::chess_server::types::ChessBoard;
use crate::chess_server::types::Move;

use ordered_float::OrderedFloat;

pub struct MinMaxSearcher<E: Evaluator> {
    max_depth: usize,
    phantom: PhantomData<E>,
}

impl<E: Evaluator> MinMaxSearcher<E> {
    pub fn new(max_depth: usize) -> MinMaxSearcher<E> {
        if max_depth == 0 {
            panic!("Max depth must be at least 1.")
        }

        MinMaxSearcher { max_depth, phantom: PhantomData }
    }

    fn search_impl(&self, chess_board: &ChessBoard, evaluator: &E, depth: usize) -> (OrderedFloat<f64>, Option<Move>) {
        if depth == self.max_depth {
            (evaluator.evaluate(chess_board), None)
        } else {
            match chess_board.get_game_status() {
                GameStatus::Ongoing => {
                    let color = chess_board.get_turn_color();
                    let allowed_moves = chess_board.get_allowed_moves(color);
                    let evals = allowed_moves.iter().map(
                        |move_| (
                            self.search_impl(&chess_board.next_state(move_), evaluator, depth + 1).0,
                            Some(*move_)
                        )
                    );

                    match color {
                        Color::White => evals.max_by_key(|tup| tup.0).unwrap(),
                        Color::Black => evals.min_by_key(|tup| tup.0).unwrap(),
                    }
                },
                GameStatus::BlackWon => {
                    println!("Arrived at Black Won.");
                    (EVAL_BLACK_WON, None)
                }
                GameStatus::WhiteWon => {
                    println!("Arrived at White Won.");
                    (EVAL_WHITE_WON, None) 
                }
                GameStatus::Draw => {
                    println!("Arrived at Draw.");
                    (EVAL_DRAW, None)
                }
            }
        }
    }
}

impl<E: Evaluator> Searcher<E> for MinMaxSearcher<E> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E) -> Move {
        self.search_impl(chess_board, evaluator, 0).1.unwrap()
    }
}