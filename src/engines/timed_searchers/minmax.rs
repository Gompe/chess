use crate::chess_server::chess_types::ChessStatus;
use crate::chess_server::chess_types::Color;
use crate::engines::engine_traits::*;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;

use crate::chess_server::chess_types::ChessBoard;
use crate::chess_server::chess_types::Move;

use ordered_float::OrderedFloat;

const INF: OrderedFloat<f64> = OrderedFloat(1000.);
pub struct MinMax<E: Evaluator> {
    max_depth: usize,
    phantom: PhantomData<E>,
}

impl<E: Evaluator> MinMax<E> {
    pub fn new(max_depth: usize) -> MinMax<E> {
        if max_depth == 0 {
            panic!("Max depth must be at least 1.")
        }

        MinMax {
            max_depth,
            phantom: PhantomData,
        }
    }

    fn search_impl(
        &self,
        chess_board: &ChessBoard,
        evaluator: &E,
        depth: usize,
        start_time: Instant,
        avail_time: Duration
    ) -> (OrderedFloat<f64>, Option<Move>) {
        if depth == self.max_depth {
            (evaluator.evaluate(chess_board), None)
        } else {
            match chess_board.get_game_status() {
                ChessStatus::Ongoing => {
                    let color = chess_board.get_turn_color();
                    let sign = OrderedFloat(color.as_sign());

                    let allowed_moves = chess_board.get_allowed_moves(color);

                    let mut value = -INF;

                    // Assign arbitrary move to assure that output won't be None 
                    let mut best_move = Some(allowed_moves[0]);
                    
                    for mv in allowed_moves {
                        if Instant::now() - start_time > avail_time {
                            break;
                        }

                        let eval_search = sign * self.search_impl(
                            &chess_board.next_state(&mv),
                            evaluator,
                            depth + 1,
                            start_time,
                            avail_time
                        ).0;

                        if eval_search > value || best_move.is_none() {
                            value = eval_search;
                            best_move = Some(mv);
                        }
                    }

                    (sign * value, best_move)

                }
                ChessStatus::BlackWon => (EVAL_BLACK_WON, None),
                ChessStatus::WhiteWon => (EVAL_WHITE_WON, None),
                ChessStatus::Draw => (EVAL_DRAW, None),
            }
        }
    }
}

impl<E: Evaluator> TimedSearcher<E> for MinMax<E> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E, avail_time: Duration) -> Option<Move> {

        let avail_time = Duration::from_nanos(
           ( avail_time.as_nanos() as f64 * 0.90 ) as u64
        );

        self.search_impl(
            chess_board, 
            evaluator, 
            0,
            Instant::now(),
            avail_time
        ).1

    }
}
