use crate::chess_server::chess_types::ChessStatus;
use crate::chess_server::chess_types::Color;
use crate::engines::engine_traits::*;
use std::cmp::max;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;

use crate::chess_server::chess_types::ChessBoard;
use crate::chess_server::chess_types::Move;

use log::info;
use ordered_float::OrderedFloat;

use std::cell::RefCell;
use crate::engines::zobrist_hash::ZobristHashMap;

const BIG_INF: OrderedFloat<f64> = OrderedFloat(1001.);
const SMALL_INF: OrderedFloat<f64> = OrderedFloat(999.);

pub struct IterativeDeepening<E: Evaluator> {
    phantom: PhantomData<E>,
    cache: RefCell<ZobristHashMap<(Move, u8, ChessBoard)>>,
}

impl<E: Evaluator> IterativeDeepening<E> {
    pub fn new() -> IterativeDeepening<E> {
        IterativeDeepening {
            phantom: PhantomData,
            cache: RefCell::new(ZobristHashMap::new()),
        }
    }

    fn get_cached(&self, chess_board: &ChessBoard) -> Option<(Move, u8, ChessBoard)> {
        if let Some(&t) =
            self.cache.borrow().get_key_value(chess_board)
        {
            return Some(t);
        }

        None
    }

    fn insert_cache(
        &self,
        move_: Move,
        depth_from_point: usize,
        chess_board: &ChessBoard,
    ) {
        self.cache
            .borrow_mut()
            .insert(chess_board, (move_, depth_from_point as u8, *chess_board));
    }

    fn search_internals(
        &self,
        chess_board: &ChessBoard,
        evaluator: &E,
        depth: usize,
        max_depth: usize,
        alpha: OrderedFloat<f64>,
        beta: OrderedFloat<f64>,
        start_time: Instant,
        avail_time: Duration,
    ) -> Option<OrderedFloat<f64>> {
        let color = chess_board.get_turn_color();
        let sign = OrderedFloat(color.as_sign());
        if depth >= max_depth {
            Some(sign * evaluator.evaluate(chess_board))
        } else {
            match chess_board.get_game_status() {
                ChessStatus::Ongoing => {
                    let mut value: OrderedFloat<f64> = -BIG_INF;
                    let mut alpha = alpha;
                    
                    let mut cached_move = None;

                    if let Some((mv, _, other_board)) = self.get_cached(chess_board) {
                        
                        if other_board == *chess_board {
                            cached_move = Some(mv);
                            let eval_search = self.search_internals(
                                &chess_board.next_state(&mv),
                                evaluator,
                                depth + 1,
                                max_depth,
                                -beta,
                                -alpha,
                                start_time,
                                avail_time,
                            );

                            if let Some(eval_search) = eval_search {
                            
                                let eval_search = -eval_search;
    
                                value = max(value, eval_search);
                                alpha = max(value, alpha);
    
                                // Too good to be True, won't get to this state
                                if value >= beta {
                                    self.insert_cache(mv, max_depth - depth, chess_board);
                                    return Some(SMALL_INF);
                                }
                            } else {
                                return None
                            }
                        } else {
                            info!("Cached boards comparison failed!");
                        }
                    } 

                    let allowed_moves = chess_board.get_allowed_moves(color);

                    let mut best_move = allowed_moves[0];

                    for mv in allowed_moves {
                        if Some(mv) == cached_move {
                            continue;
                        }

                        if Instant::now() - start_time > avail_time {
                            info!("Internal Time break!");
                            break;
                        }

                        let eval_search = self.search_internals(
                            &chess_board.next_state(&mv),
                            evaluator,
                            depth + 1,
                            max_depth,
                            -beta,
                            -alpha,
                            start_time,
                            avail_time,
                        );

                        if let Some(eval_search) = eval_search {
                            let eval_search = -eval_search;

                            if eval_search > value {
                                best_move = mv;
                            }

                            value = max(value, eval_search);
                            alpha = max(value, alpha);

                            // Too good to be True, won't get to this state
                            if value >= beta {
                                self.insert_cache(mv, max_depth - depth, chess_board);
                                return Some(SMALL_INF);
                            }
                        } else {
                            return None
                        }
                    }

                    self.insert_cache(best_move, max_depth - depth, chess_board);
                    Some(value)
                }
                ChessStatus::BlackWon => Some(sign * EVAL_BLACK_WON),
                ChessStatus::WhiteWon => Some(sign * EVAL_WHITE_WON),
                ChessStatus::Draw => Some(EVAL_DRAW),
            }
        }
    }
}

impl<E: Evaluator> TimedSearcher<E> for IterativeDeepening<E> {
    fn search(
        &self,
        chess_board: &ChessBoard,
        evaluator: &E,
        avail_time: Duration,
    ) -> Option<Move> {
        let start_time = Instant::now();

        self.cache.borrow_mut().clear();

        let avail_time = Duration::from_nanos((avail_time.as_nanos() as f64 * 0.90) as u64);

        let color = chess_board.get_turn_color();
        let allowed_moves = chess_board.get_allowed_moves(color);

        let mut value = -BIG_INF;

        // Assign arbitrary move to assure that output won't be None
        let mut best_move = Some(allowed_moves[0]);

        for max_depth in 0.. {
            if Instant::now() - start_time > avail_time {
                info!("Cutoff at max depth: {}", max_depth - 1);
                break;
            }

            let mut local_value = -BIG_INF;
            let mut local_best_move = None;

            let mut time_cutoff = false;

            for &mv in allowed_moves.iter() {
                if Instant::now() - start_time > avail_time {
                    info!("Time break!");
                    time_cutoff = true;
                    break;
                }

                if let Some(eval_search) = self.search_internals(
                    &chess_board.next_state(&mv),
                    evaluator,
                    1,
                    max_depth,
                    -BIG_INF,
                    -local_value,
                    start_time,
                    avail_time,
                ) {
                    let eval_search = -eval_search;
                    // info!("Move: {} - {}, eval search: {}", count, mv, eval_search);
    
                    if eval_search > local_value {
                        local_value = eval_search;
                        local_best_move = Some(mv);
    
                        // if eval_search > value {
                        //     value = eval_search;
                        //     best_move = Some(mv);
                        // }
                    }
                } else {
                    time_cutoff = true;
                    break;
                }
            }

            if !time_cutoff {
                best_move = local_best_move;
                value = local_value;
                info!("Completed depth {}. Eval {}. Best Move: {}", max_depth, value, best_move.unwrap());
            }
            
        }
        
        info!("Completed Search: Eval {}. Best Move: {}", value, best_move.unwrap());
        best_move
    }
}
