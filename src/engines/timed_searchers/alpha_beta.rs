use crate::backend::ChessStatus;
use crate::backend::Color;
use crate::engines::engine_traits::*;
use std::cmp::max;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;

use crate::backend::ChessBoard;
use crate::backend::Move;

use log::info;
use ordered_float::OrderedFloat;

const BIG_INF: OrderedFloat<f64> = OrderedFloat(1001.);
const SMALL_INF: OrderedFloat<f64> = OrderedFloat(999.);

pub struct AlphaBeta<E: Evaluator> {
    max_depth: usize,
    phantom: PhantomData<E>,
}

impl<E: Evaluator> AlphaBeta<E> {
    pub fn new(max_depth: usize) -> AlphaBeta<E> {
        if max_depth == 0 {
            panic!("Max depth must be at least 1.")
        }

        AlphaBeta {
            max_depth,
            phantom: PhantomData,
        }
    }

    fn search_internals(
        &self,
        chess_board: &ChessBoard,
        evaluator: &E,
        depth: usize,
        alpha: OrderedFloat<f64>,
        beta: OrderedFloat<f64>,
        start_time: Instant,
        avail_time: Duration
    ) -> OrderedFloat<f64> {
        let color = chess_board.get_turn_color();
        let sign = OrderedFloat(color.as_sign());
        if depth >= self.max_depth {

            sign * evaluator.evaluate(chess_board)

        } else {
            match chess_board.get_game_status() {
                ChessStatus::Ongoing => {
                    let allowed_moves = chess_board.get_allowed_moves(color);

                    let mut value: OrderedFloat<f64> = -BIG_INF;
                    
                    let mut alpha = alpha;

                    for mv in allowed_moves {
                        if Instant::now() - start_time > avail_time {
                            info!("Internal Time break!");
                            break;
                        }

                        let eval_search = -self.search_internals(
                            &chess_board.next_state(&mv),
                            evaluator,
                            depth + 1,
                            -beta,
                            -alpha,
                            start_time,
                            avail_time
                        );

                        value = max(value, eval_search);
                        alpha = max(value, alpha);
                        
                        // Too good to be True, won't get to this state
                        if value > beta {
                            return SMALL_INF;
                        }
                    }
                    
                    value
                }
                ChessStatus::BlackWon => sign * EVAL_BLACK_WON,
                ChessStatus::WhiteWon => sign * EVAL_WHITE_WON,
                ChessStatus::Draw => EVAL_DRAW,
            }
        }
    }
}

impl<E: Evaluator> TimedSearcher<E> for AlphaBeta<E> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E, avail_time: Duration) -> Option<Move> {

        let start_time = Instant::now();

        let avail_time = Duration::from_nanos(
           ( avail_time.as_nanos() as f64 * 0.90 ) as u64
        );

        let color = chess_board.get_turn_color();
        let sign = OrderedFloat(color.as_sign());

        let allowed_moves = chess_board.get_allowed_moves(color);

        let mut value = -BIG_INF;

        // Assign arbitrary move to assure that output won't be None 
        let mut best_move = Some(allowed_moves[0]);
        
        let total_count = allowed_moves.len();
        let mut count = 0;

        for mv in allowed_moves {
            if Instant::now() - start_time > avail_time {
                info!("Time break!");
                break;
            }

            let eval_search = -self.search_internals(
                &chess_board.next_state(&mv),
                evaluator,
                1,
                -BIG_INF,
                -value,
                start_time,
                avail_time
            );

            // info!("Move: {} - {}, eval search: {}", count, mv, eval_search);

            if eval_search > value {
                value = eval_search;
                best_move = Some(mv);
            }

            count += 1;
        }

        info!("Eval: {}, Depth: {}. Analyzed {} moves out of {}", sign * value, self.max_depth, count, total_count);

        best_move
    }
}
