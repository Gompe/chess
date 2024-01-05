use crate::backend::ChessStatus;
use crate::backend::Color;
use crate::backend::Piece;
use crate::backend::chess_board;
use crate::backend::chess_move::BitMove;
use crate::backend::color_piece;
use crate::engines::engine_traits::*;

use crate::engines::evaluators::CaptureEvaluator;
use crate::engines::evaluators::{TrivialEvaluator, CacheEvaluator};

use std::cmp::max;
use std::marker::PhantomData;
use std::time::Duration;
use std::time::Instant;

use crate::backend::ChessBoard;
use crate::backend::Move;

use log::info;
use ordered_float::OrderedFloat;
use smallvec::SmallVec;

use std::cell::RefCell;
use crate::engines::zobrist_hash::ZobristHashMap;

use std::ops::Neg;
use std::cmp::{PartialOrd, Ord, Ordering};

use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EvalType {
    MaximizerMate(usize),
    MinimizerMate(usize),
    ExactEval(OrderedFloat<f64>)
}

impl EvalType {
    fn forward(self) -> Self {
        match self {
            Self::MaximizerMate(depth) => {
                Self::MinimizerMate(depth + 1)    
            },
            Self::MinimizerMate(depth) => {
                Self::MaximizerMate(depth + 1)
            },
            Self::ExactEval(score) => {
                Self::ExactEval(-score)
            }
        }
    }
}

impl Neg for EvalType {
    type Output = Self;
    fn neg(self) -> Self {
        match self {
            Self::MaximizerMate(depth) => {
                Self::MinimizerMate(depth)    
            },
            Self::MinimizerMate(depth) => {
                Self::MaximizerMate(depth)
            },
            Self::ExactEval(score) => {
                Self::ExactEval(-score)
            }
        }
    }
}

impl PartialOrd for EvalType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match *self {
            Self::MaximizerMate(depth) => {
                match *other {
                    Self::MaximizerMate(other_depth) => {
                        other_depth.partial_cmp(&depth) 
                    },
                    _ => Some(Ordering::Greater)
                }
            },
            Self::MinimizerMate(depth) => {
                match *other {
                    Self::MinimizerMate(other_depth) => {
                        depth.partial_cmp(&other_depth)
                    },
                    _ => Some(Ordering::Less)
                }
            },
            Self::ExactEval(score) => {
                match *other {
                    Self::MaximizerMate(_) => Some(Ordering::Less),
                    Self::MinimizerMate(_) => Some(Ordering::Greater),
                    Self::ExactEval(other_score) => score.partial_cmp(&other_score) 
                }
            }
        }
    }
}

impl Ord for EvalType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for EvalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Clone, Copy)]
enum NodeType {
    PVNode(EvalType),
    AllNode(EvalType),
    CutNode(EvalType)
}

pub struct ClunkySearcherV3<E: Evaluator> {
    phantom: PhantomData<E>,
    cache: RefCell<ZobristHashMap<(NodeType, Move, u8, ChessBoard)>>,
    quiet_eval: CaptureEvaluator<TrivialEvaluator>
}


fn sort_moves(moves: &mut SmallVec<[BitMove; 64]>, chess_board: &ChessBoard) {
    moves.sort_by_key(|mv| { 
        match chess_board.get_square_content(&mv.get_next_square()) {
            None => 0,
            Some(color_piece) => {
                match color_piece.get_piece() {
                    Piece::Pawn => -1,
                    Piece::Knight => -2,
                    Piece::Bishop => -3,
                    Piece::Rook => -4,
                    Piece::Queen => -5,
                    _ => 0,
                }
            }
        }
    })
}

impl<E: Evaluator> ClunkySearcherV3<E> {
    pub fn new() -> ClunkySearcherV3<E> {
        ClunkySearcherV3 {
            phantom: PhantomData,
            cache: RefCell::new(ZobristHashMap::new()),
            quiet_eval: CaptureEvaluator::new(TrivialEvaluator::new())
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

    fn is_quiet(&self, chess_board: &ChessBoard) -> bool {
        self.quiet_eval.evaluate(chess_board) == OrderedFloat(0.)
    }

    fn quiescence_search(
        &self,
        chess_board: &ChessBoard,
        evaluator: &E,
        depth: usize,
        alpha: EvalType,
        beta: EvalType,
        start_time: Instant,
        avail_time: Duration,
    ) -> NodeType {
        
        let color = chess_board.get_turn_color();

        if depth == 0 || self.is_quiet(chess_board) {
            let sign = OrderedFloat(color.as_sign());
            return NodeType::PVNode(EvalType::ExactEval(sign * evaluator.evaluate(chess_board))) 
        }

        let allowed_moves = chess_board.get_allowed_moves(color);

        // Only keep captures
        let mut captures: SmallVec<[BitMove; 64]> = allowed_moves.iter().filter(
            |&&mv| { chess_board.get_square_content(&mv.get_next_square()).is_some() }
        ).map(|&mv| { mv }).collect();

        sort_moves(&mut captures, chess_board);

        if captures.is_empty() {
            let sign = OrderedFloat(color.as_sign());
            NodeType::PVNode(EvalType::ExactEval(sign * evaluator.evaluate(chess_board)))
        } else {
            match chess_board.get_game_status_from_precomputed(&allowed_moves) {
                ChessStatus::Ongoing => {
                    let imut_alpha = alpha;
                    let mut alpha = alpha;

                    for mv in captures {

                        let eval_search = self.quiescence_search(
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
                                let eval_move = cached_alpha.forward();

                                if eval_move > alpha {
                                    alpha = eval_move;
                                }
                            },
                            NodeType::PVNode(score) => {
                                // Exact score. Can do the same thing
                                let eval_move = score.forward();

                                if eval_move > alpha {
                                    alpha = eval_move;
                                }
                            }
                            NodeType::CutNode(_) => {
                                // cached_beta is a lower bound of the next node value
                            }
                        }

                        if alpha >= beta {
                            // Beta cutoff
                            return NodeType::CutNode(alpha)
                        }
                    }

                    let node_type = if alpha > imut_alpha {
                        NodeType::PVNode(alpha)
                    } else {
                        NodeType::AllNode(alpha)
                    };

                    node_type
                }
                ChessStatus::BlackWon => {
                    match color {
                        Color::White => NodeType::PVNode(EvalType::MinimizerMate(0)),
                        Color::Black => NodeType::PVNode(EvalType::MaximizerMate(0)),
                    }
                },
                ChessStatus::WhiteWon => {
                    match color {
                        Color::White => NodeType::PVNode(EvalType::MaximizerMate(0)),
                        Color::Black => NodeType::PVNode(EvalType::MinimizerMate(0)),
                    }
                },
                ChessStatus::Draw => NodeType::PVNode(EvalType::ExactEval(OrderedFloat(0.))),
            }
        }
    }

    fn search_internals(
        &self,
        chess_board: &ChessBoard,
        evaluator: &E,
        depth: usize,
        alpha: EvalType,
        beta: EvalType,
        start_time: Instant,
        avail_time: Duration,
    ) -> NodeType {
        let color = chess_board.get_turn_color();
        if depth == 0 {
            let sign = OrderedFloat(color.as_sign());
            let this_eval = EvalType::ExactEval(sign * evaluator.evaluate(chess_board));
            if self.is_quiet(chess_board) {
                NodeType::PVNode(this_eval)
            } else {
                // We use the Null-Move Observation
                let alpha = this_eval;

                self.quiescence_search(chess_board, evaluator, 2, alpha, beta, start_time, avail_time)
            }
        } else {

            let imut_alpha = alpha;
            let mut alpha = alpha;

            let mut cached_move = None;
            let mut mut_cached_depth = -1;

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
                    mut_cached_depth = cached_depth as i32;

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
                            alpha = max(alpha, cached_alpha.forward());
                        },
                        NodeType::PVNode(score) => {
                            // Exact score. Can do the same thing
                            alpha = max(alpha, score.forward());
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

            let mut allowed_moves = chess_board.get_allowed_moves(color);
            sort_moves(&mut allowed_moves, chess_board);

            match chess_board.get_game_status_from_precomputed(&allowed_moves) {
                ChessStatus::Ongoing => {
                    

                    let mut best_move = if cached_move.is_none() {
                        allowed_moves[0] 
                    } else {
                        cached_move.unwrap()
                    };

                    for mv in allowed_moves {
                        if Some(mv) == cached_move {
                            continue;
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
                                let eval_move = cached_alpha.forward();

                                if eval_move > alpha {
                                    alpha = eval_move;
                                    best_move = mv;
                                }
                            },
                            NodeType::PVNode(score) => {
                                // Exact score. Can do the same thing
                                let eval_move = score.forward();

                                if eval_move > alpha {
                                    alpha = eval_move;
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

                    if depth as i32 >= mut_cached_depth {
                        self.insert_cache(node_type, best_move, depth, chess_board);
                    } 

                    node_type
                }
                ChessStatus::BlackWon => {
                    match color {
                        Color::White => NodeType::PVNode(EvalType::MinimizerMate(0)),
                        Color::Black => NodeType::PVNode(EvalType::MaximizerMate(0)),
                    }
                },
                ChessStatus::WhiteWon => {
                    match color {
                        Color::White => NodeType::PVNode(EvalType::MaximizerMate(0)),
                        Color::Black => NodeType::PVNode(EvalType::MinimizerMate(0)),
                    }
                },
                ChessStatus::Draw => NodeType::PVNode(EvalType::ExactEval(OrderedFloat(0.))),
            }
        }
    }
}

impl<E: Evaluator> TimedSearcher<E> for ClunkySearcherV3<E> {
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
        let mut allowed_moves = chess_board.get_allowed_moves(color);

        sort_moves(&mut allowed_moves, chess_board);

        let mut value = EvalType::MinimizerMate(0);

        // Assign arbitrary move to assure that output won't be None
        let mut best_move = Some(allowed_moves[0]);

        for max_depth in 0.. {
            if Instant::now() - start_time > avail_time {
                info!("Cutoff at max depth: {}", max_depth - 1);
                break;
            }

            let mut local_value = EvalType::MinimizerMate(0);

            let mut time_cutoff = false;

            let mut cached_move = None;

            if let Some((node_type, mv, cached_depth, other_board)) = self.get_cached(chess_board) {
                if other_board == *chess_board {   
                    
                    // Check if depth is enough to justify ending the search    
                    if cached_depth >= max_depth as u8 {
                        match node_type {
                            NodeType::PVNode(score) => {
                                value = score;
                                best_move = Some(mv);
                                continue;
                            },
                            _ => ()
                        }
                    };

                    // Depth is not enough - Look for cached move
                    cached_move = Some(mv);

                    let eval_search = self.search_internals(
                        &chess_board.next_state(&mv),
                        evaluator,
                        max_depth,
                        EvalType::MinimizerMate(0),
                        -local_value,
                        start_time,
                        avail_time,
                    );

                    match eval_search {
                        NodeType::AllNode(cached_alpha) => {
                            // cached_alpha is an upper bound of the next node value
                            // So -cached_alpha is a lower bound of the score in this position
                            local_value = cached_alpha.forward();
                        },
                        NodeType::PVNode(score) => {
                            // Exact score. Can do the same thing
                            local_value = score.forward();
                        }
                        NodeType::CutNode(_) => {
                            // cached_beta is a lower bound of the next node value
                        }
                    }
                    
                    best_move = Some(mv);
                } 
            }

            let num_moves = allowed_moves.len();
            let mut count = 0;


            for &mv in allowed_moves.iter() {
                if Instant::now() - start_time > avail_time {
                    info!("Time break! Analyzed {}/{} moves.", count, num_moves);
                    time_cutoff = true;
                    break;
                }

                count += 1;

                if Some(mv) == cached_move { continue; }

                let eval_search = self.search_internals(
                    &chess_board.next_state(&mv),
                    evaluator,
                    max_depth,
                    EvalType::MinimizerMate(0),
                    -local_value,
                    start_time,
                    avail_time,
                );
                
                match eval_search {
                    NodeType::AllNode(cached_alpha) => {
                        // cached_alpha is an upper bound of the next node value
                        // So -cached_alpha is a lower bound of the score in this position
                        let eval_move = cached_alpha.forward();
                        if eval_move > local_value {
                            local_value = eval_move;
                            best_move = Some(mv);
                        }
                    },
                    NodeType::PVNode(score) => {
                        // Exact score. Can do the same thing
                        let eval_move = score.forward();
                        if eval_move > local_value {
                            local_value = eval_move;
                            best_move = Some(mv);
                        }
                    }
                    NodeType::CutNode(_) => {
                        // cached_beta is a lower bound of the next node value
                    }
                }
            }

            if !time_cutoff {
                value = local_value;
                self.insert_cache(NodeType::PVNode(local_value), best_move.unwrap(), max_depth, chess_board);

                info!("Completed depth {}. Eval {}. Best Move: {}", max_depth, value, best_move.unwrap());
            }
            
        }
        
        info!("Completed Search: Eval {}. Best Move: {}", value, best_move.unwrap());
        best_move
    }
}
