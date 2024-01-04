use crate::chess_server::chess_types::ChessStatus;
use crate::chess_server::chess_types::Color;
use crate::engines::engine_traits::*;
use crate::engines::evaluators::cache_evaluator;
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

#[derive(Clone, Copy)]
enum NodeType {
    PVNode(OrderedFloat<f64>),
    AllNode(OrderedFloat<f64>),
    CutNode(OrderedFloat<f64>)
}

#[derive(Clone, Copy)]
enum EvalType {
    WhiteMate(usize),
    BlackMate(usize),
    ExactEval(OrderedFloat<f64>)
}


const BIG_INF: OrderedFloat<f64> = OrderedFloat(1001.);
const SMALL_INF: OrderedFloat<f64> = OrderedFloat(999.);

pub struct ClunkySearcher<E: Evaluator> {
    phantom: PhantomData<E>,
    cache: RefCell<ZobristHashMap<(NodeType, Move, u8, ChessBoard)>>,
}

impl<E: Evaluator> ClunkySearcher<E> {
    pub fn new() -> ClunkySearcher<E> {
        ClunkySearcher {
            phantom: PhantomData,
            cache: RefCell::new(ZobristHashMap::new()),
        }
    }

    fn get_cached(&self, chess_board: &ChessBoard) -> Option<(NodeType, Move, u8, ChessBoard)> {
        if let Some(&t) =
            self.cache.borrow().get_key_value(chess_board)
        {
            return Some(t);
        }

        None
    }

    fn insert_cache(
        &self,
        node_type: NodeType,
        mv: Move,
        depth_from_point: usize,
        chess_board: &ChessBoard,
    ) {
        self.cache
            .borrow_mut()
            .insert(chess_board, (node_type, mv, depth_from_point as u8, *chess_board));
    }

    fn search_internals(
        &self,
        chess_board: &ChessBoard,
        evaluator: &E,
        depth: usize,
        alpha: OrderedFloat<f64>,
        beta: OrderedFloat<f64>,
        start_time: Instant,
        avail_time: Duration,
    ) -> NodeType {
        let color = chess_board.get_turn_color();
        let sign = OrderedFloat(color.as_sign());
        if depth == 0 {
            NodeType::PVNode(sign * evaluator.evaluate(chess_board))
        } else {
            match chess_board.get_game_status() {
                ChessStatus::Ongoing => {
                    let imut_alpha = alpha;
                    let mut alpha = alpha;
    
                    let mut cached_move = None;

                    if let Some((node_type, mv, cached_depth, other_board)) = self.get_cached(chess_board) {
                        if other_board == *chess_board {
                            
                            // Check if depth is enough to justify ending the search
                            if cached_depth >= depth as u8 {
                                match node_type {
                                    NodeType::AllNode(cached_alpha) => {
                                        if cached_alpha >= alpha {
                                            return NodeType::AllNode(cached_alpha);
                                        }
                                    },
                                    NodeType::CutNode(cached_beta) => {
                                        if cached_beta <= beta {
                                            return NodeType::AllNode(cached_beta);
                                        }
                                    },
                                    NodeType::PVNode(score) => {
                                        return NodeType::PVNode(score);
                                    },
                                }
                            }

                            // Depth is not enough - Look for cached move
                            cached_move = Some(mv);
                            let eval_search = self.search_internals(
                                &chess_board.next_state(&mv),
                                evaluator,
                                depth - 1,
                                -beta,
                                -alpha,
                                start_time,
                                avail_time,
                            );

                            match eval_search {
                                NodeType::AllNode(cached_alpha) => {
                                    // cached_alpha is an upper bound of the next node value
                                    // So -cached_alpha is a lower bound of the score in this position
                                    alpha = max(alpha, -cached_alpha);
                                },
                                NodeType::PVNode(score) => {
                                    // Exact score. Can do the same thing
                                    alpha = max(alpha, -score);
                                }
                                NodeType::CutNode(_) => {
                                    // cached_beta is a lower bound of the next node value
                                }
                            }

                            if alpha >= beta {
                                // Beta cutoff
                                if depth as u8 >= cached_depth {
                                    self.insert_cache(NodeType::CutNode(alpha), mv, depth, chess_board);
                                }

                                return NodeType::CutNode(alpha)
                            }
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
                            depth - 1,
                            -beta,
                            -alpha,
                            start_time,
                            avail_time,
                        );

                        match eval_search {
                            NodeType::AllNode(cached_alpha) => {
                                // cached_alpha is an upper bound of the next node value
                                // So -cached_alpha is a lower bound of the score in this position
                                if -cached_alpha > alpha {
                                    alpha = -cached_alpha;
                                    best_move = mv;
                                }
                            },
                            NodeType::PVNode(score) => {
                                // Exact score. Can do the same thing
                                if -score > alpha {
                                    alpha = -score;
                                    best_move = mv;
                                }
                            }
                            NodeType::CutNode(_) => {
                                // cached_beta is a lower bound of the next node value
                            }
                        }

                        if alpha >= beta {
                            // Beta cutoff
                            self.insert_cache(NodeType::CutNode(alpha), best_move, depth, chess_board);
                            return NodeType::CutNode(alpha)
                        }
                    }

                    let node_type = if alpha > imut_alpha {
                        NodeType::PVNode(alpha)
                    } else {
                        NodeType::AllNode(alpha)
                    };

                    self.insert_cache(node_type, best_move, depth, chess_board);
                    node_type
                }
                ChessStatus::BlackWon => NodeType::PVNode(sign * EVAL_BLACK_WON),
                ChessStatus::WhiteWon => NodeType::PVNode(sign * EVAL_WHITE_WON),
                ChessStatus::Draw => NodeType::PVNode(EVAL_DRAW),
            }
        }
    }
}

impl<E: Evaluator> TimedSearcher<E> for ClunkySearcher<E> {
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

                let eval_search = self.search_internals(
                    &chess_board.next_state(&mv),
                    evaluator,
                    max_depth,
                    -BIG_INF,
                    -local_value,
                    start_time,
                    avail_time,
                );
                
                match eval_search {
                    NodeType::AllNode(cached_alpha) => {
                        // cached_alpha is an upper bound of the next node value
                        // So -cached_alpha is a lower bound of the score in this position
                        if -cached_alpha > local_value {
                            local_value = -cached_alpha;
                            local_best_move = Some(mv);
                        }
                    },
                    NodeType::PVNode(score) => {
                        // Exact score. Can do the same thing
                        if -score > local_value {
                            local_value = -score;
                            local_best_move = Some(mv);
                        }
                    }
                    NodeType::CutNode(_) => {
                        // cached_beta is a lower bound of the next node value
                    }
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
