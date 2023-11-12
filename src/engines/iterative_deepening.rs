use std::cell::RefCell;

use crate::chess_server::types::Color;
use crate::chess_server::types::GameStatus;
use crate::chess_server::types::Piece;
use crate::engines::engine_traits::*;

use std::cmp::max;
use std::cmp::min;
use std::marker::PhantomData;

use crate::chess_server::types::ChessBoard;
use crate::chess_server::types::Move;

use ordered_float::OrderedFloat;

use super::zobrist_hash::ZobristHashMap;

const INF: OrderedFloat<f64> = OrderedFloat(1000.);

pub struct IterativeDeepening<E: Evaluator> {
    max_depth: usize,
    cache: RefCell< ZobristHashMap<(OrderedFloat<f64>, Move, u8)> >,
    phantom: PhantomData<E>,
}

impl<E: Evaluator> IterativeDeepening<E> {
    pub fn new(max_depth: usize) -> IterativeDeepening<E> {
        if max_depth == 0 {
            panic!("Max depth must be at least 1.")
        }

        IterativeDeepening { max_depth, phantom: PhantomData, cache: RefCell::new(ZobristHashMap::new()) }
    }

    fn get_cached(&self, chess_board: &ChessBoard) -> Option<(OrderedFloat<f64>, Move, u8)> {
        if let Some(&(eval, move_, depth_from_point)) = self.cache.borrow().get_key_value(chess_board) {
            return Some((eval, move_, depth_from_point));
        }

        None
    }

    fn insert_cache(&self, chess_board: &ChessBoard, depth_from_point: usize, eval: OrderedFloat<f64>, move_: Move) {
        self.cache.borrow_mut().insert(chess_board, (eval, move_, depth_from_point as u8));
    }

    fn search_impl(
        &self, chess_board: &ChessBoard, 
        evaluator: &E, 
        depth: usize,
        alpha: OrderedFloat<f64>,
        beta: OrderedFloat<f64>,
        max_depth: usize

    ) -> (OrderedFloat<f64>, Option<Move>) {

        if depth == max_depth {
            return (evaluator.evaluate(chess_board), None);
        } 

        let mut best_move = None;

        if let Some((eval, move_, depth_from_point)) = self.get_cached(chess_board) {
            if depth_from_point >= (max_depth - depth) as u8 {
                return (eval, Some(move_))
            }

            best_move = Some(move_);
        }

        match chess_board.get_game_status() {
            GameStatus::Ongoing => {
                let color = chess_board.get_turn_color();
                let mut allowed_moves = chess_board.get_allowed_moves(color);

                
                if depth == 0 {
                    if color == Color::White {
                        allowed_moves.sort_by_key(|&move_| -evaluator.evaluate(&chess_board.next_state(&move_)));
                    } else {
                        allowed_moves.sort_by_key(|&move_| evaluator.evaluate(&chess_board.next_state(&move_)));
                    }
                } else {
                    let get_piece_value = |x: Piece| {
                        match x {
                            Piece::Pawn => 1,
                            Piece::Knight => 2,
                            Piece::Bishop => 3,
                            Piece::Rook => 4,
                            Piece::Queen => 5,
                            Piece::King => 6,
                        }
                    };
                    
                    if color == Color::White {
                        allowed_moves.sort_by_key(|&move_| {
                            if chess_board.contains_piece_of_color(&move_.get_next_square(), Color::Black) {
                                -get_piece_value(chess_board.get_cell_content(&move_.get_next_square()).unwrap().piece)
                            } else {
                                0
                            }
                        });
                    } else {
                        allowed_moves.sort_by_key(|&move_| {
                            if chess_board.contains_piece_of_color(&move_.get_next_square(), Color::White) {
                                -get_piece_value(chess_board.get_cell_content(&move_.get_next_square()).unwrap().piece)
                            } else {
                                0
                            }
                        });
                    }
                }
            
                
                if color == Color::White {
                    // Maximizing Player
                    let mut value = -INF;
                    let mut alpha = alpha;
                    
                    // Killer Heuristic
                    if let Some(move_) = best_move {
                        let search_result = self.search_impl(
                            &chess_board.next_state(&move_), &evaluator, depth + 1, alpha, beta, max_depth
                        );
                        
                        if search_result.0 > value {
                            value = search_result.0;
                            best_move = Some(move_);
                        }

                        alpha = max(alpha, value);
                    };

                    
                    for move_ in allowed_moves {
                        let search_result = self.search_impl(
                            &chess_board.next_state(&move_), &evaluator, depth + 1, alpha, beta, max_depth
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

                    self.insert_cache(chess_board, max_depth - depth, value, best_move.unwrap());

                    return (value, best_move);

                } else {
                    // Minimizing Player
                    let mut value = INF;
                    let mut beta = beta;
                    
                    // Killer Heuristic
                    if let Some(move_) = best_move {
                        let search_result = self.search_impl(
                            &chess_board.next_state(&move_), &evaluator, depth + 1, alpha, beta, max_depth
                        );
                        
                        if search_result.0 < value {
                            value = search_result.0;
                            best_move = Some(move_);
                        }

                        beta = min(beta, value);
                    };

                    for move_ in allowed_moves {
                        let search_result = self.search_impl(
                            &chess_board.next_state(&move_), &evaluator, depth + 1, alpha, beta, max_depth
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
                    
                    self.insert_cache(chess_board, max_depth - depth, value, best_move.unwrap());

                    return (value, best_move);
                }
            },
            GameStatus::BlackWon => (EVAL_BLACK_WON, None),
            GameStatus::WhiteWon => (EVAL_WHITE_WON, None),
            GameStatus::Draw => (EVAL_DRAW, None)
        }
    }
}

impl<E: Evaluator> Searcher<E> for IterativeDeepening<E> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E) -> Move {
        for max_depth in 1..self.max_depth {
            self.search_impl(chess_board, evaluator, 0, -INF, INF, max_depth);
        }

        self.search_impl(chess_board, evaluator, 0, -INF, INF, self.max_depth).1.unwrap()
    }
}
