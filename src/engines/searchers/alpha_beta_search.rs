use crate::chess_server::types::Color;
use crate::chess_server::types::GameStatus;
use crate::engines::engine_traits::*;
use std::cmp::max;
use std::cmp::min;
use std::marker::PhantomData;

use crate::chess_server::types::ChessBoard;
use crate::chess_server::types::Move;

use ordered_float::OrderedFloat;

const INF: OrderedFloat<f64> = OrderedFloat(1000.);

pub struct AlphaBetaSearcher<E: Evaluator> {
    max_depth: usize,
    phantom: PhantomData<E>,
}

impl<E: Evaluator> AlphaBetaSearcher<E> {
    pub fn new(max_depth: usize) -> AlphaBetaSearcher<E> {
        if max_depth == 0 {
            panic!("Max depth must be at least 1.")
        }

        AlphaBetaSearcher { max_depth, phantom: PhantomData }
    }

    fn search_impl(
        &self, chess_board: &ChessBoard, 
        evaluator: &E, 
        depth: usize,
        alpha: OrderedFloat<f64>,
        beta: OrderedFloat<f64>

    ) -> (OrderedFloat<f64>, Option<Move>) {

        if depth == self.max_depth {
            (evaluator.evaluate(chess_board), None)
        } else {
            match chess_board.get_game_status() {
                GameStatus::Ongoing => {
                    let color = chess_board.get_turn_color();
                    let allowed_moves = chess_board.get_allowed_moves(color);

                    if color == Color::White {
                        // Maximizing Player
                        let mut value = -INF;
                        let mut alpha = alpha;
                        let mut best_move: Option<Move> = None;
                        
                        for move_ in allowed_moves {
                            let search_result = self.search_impl(
                                &chess_board.next_state(&move_), &evaluator, depth + 1, alpha, beta
                            );
                            
                            if search_result.0 > value || best_move == None {
                                value = search_result.0;
                                best_move = Some(move_);
                            }

                            if value > beta {
                                break;
                            }

                            alpha = max(alpha, value);
                        }

                        return (value, best_move);

                    } else {
                        // Minimizing Player
                        let mut value = INF;
                        let mut beta = beta;
                        let mut best_move: Option<Move> = None;
                        
                        for move_ in allowed_moves {
                            let search_result = self.search_impl(
                                &chess_board.next_state(&move_), &evaluator, depth + 1, alpha, beta
                            );
                            
                            if search_result.0 < value || best_move == None {
                                value = search_result.0;
                                best_move = Some(move_);
                            }

                            if value < alpha {
                                break;
                            }

                            beta = min(beta, value);
                        }

                        return (value, best_move);
                    }
                },
                GameStatus::BlackWon => (EVAL_BLACK_WON, None),
                GameStatus::WhiteWon => (EVAL_WHITE_WON, None),
                GameStatus::Draw => (EVAL_DRAW, None)
            }
        }
    }
}

impl<E: Evaluator> Searcher<E> for AlphaBetaSearcher<E> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E) -> Move {
        self.search_impl(chess_board, evaluator, 0, -INF, INF).1.unwrap()
    }
}
