use std::cell::RefCell;
use std::rc::Rc;

use log::{error, info};

use crate::chess_server::chess_types::Color;
use crate::chess_server::chess_types::ChessStatus;
use crate::chess_server::chess_types::Piece;
use crate::chess_server::chess_types::chess_board::MOVE_CONTAINER_SIZE;
use crate::chess_server::chess_types::chess_board::MoveContainer;

use crate::engines::engine_traits::*;

use std::cmp::max;
use std::cmp::min;
use std::marker::PhantomData;

use crate::chess_server::chess_types::ChessBoard;
use crate::chess_server::chess_types::Move;

use ordered_float::Float;
use ordered_float::OrderedFloat;
use smallvec::{SmallVec, smallvec};


use crate::engines::zobrist_hash::ZobristHashMap;

const BLACK_WON_EVAL: OrderedFloat<f64> = OrderedFloat(-1.);
const WHITE_WON_EVAL: OrderedFloat<f64> = OrderedFloat(1.);
const DRAW_EVAL: OrderedFloat<f64> = OrderedFloat(0.);

struct MctsNode {
    allowed_moves: MoveContainer,
    priors: SmallVec<[f64; MOVE_CONTAINER_SIZE]>,
    action_count: SmallVec<[usize; MOVE_CONTAINER_SIZE]>,
    q_values: SmallVec<[f64; MOVE_CONTAINER_SIZE]>,

    node_value: f64
}

impl MctsNode {
    fn new(allowed_moves: MoveContainer, priors:SmallVec<[f64; MOVE_CONTAINER_SIZE]>, node_value: f64) -> MctsNode {

        let n = allowed_moves.len();

        // uniform prior
        let priors  = priors;
        
        let action_count: SmallVec<[usize; MOVE_CONTAINER_SIZE]> = smallvec![0; n];
        
        let q_values: SmallVec<[f64; MOVE_CONTAINER_SIZE]> = smallvec![0.; n];

        MctsNode { 
            allowed_moves: allowed_moves, 
            priors, 
            action_count, 
            q_values,

            node_value
        }
    }
}

fn relu(x: f64) -> f64 {
    max(OrderedFloat(x), OrderedFloat(0.)).0
}

#[derive(Clone)]
pub struct MonteCarloTreeSearch<E: Evaluator, P: Policy> {
    policy: P,
    max_depth: usize,
    max_iter: usize,
    c_puct: f64,
    cache: RefCell< ZobristHashMap< Rc<RefCell<MctsNode>> > >,
    phantom: PhantomData<E>,
}

impl<E: Evaluator, P: Policy> MonteCarloTreeSearch<E, P> {
    
    pub fn new(
        policy: P, 
        max_depth: usize, 
        max_iter: usize, 
        c_puct: f64
    ) -> MonteCarloTreeSearch<E, P> {

        if max_depth == 0 {
            panic!("Max depth must be at least 1.")
        }

        MonteCarloTreeSearch { 
            policy,
            max_depth, 
            max_iter,
            c_puct,
            cache: RefCell::new(ZobristHashMap::new()),
            phantom: PhantomData, 
        }
    }

    fn get_mut_node(&self, chess_board: &ChessBoard) -> Option< Rc< RefCell<MctsNode> > > {
        if let Some(node_ref) = self.cache.borrow_mut().get_key_value(chess_board) {
            Some(node_ref.clone())
        } else {
            None
        }
    }

    fn insert_cache(&self, chess_board: &ChessBoard, node: Rc< RefCell<MctsNode> >) {
        self.cache.borrow_mut().insert(chess_board, node);
    }

    fn search_internals(
        &self,
        chess_board: &ChessBoard, 
        evaluator: &E, 
        depth: usize
    ) -> OrderedFloat<f64> {

        let color = chess_board.get_turn_color();
        let sign = match color  {
            Color::White => 1.,
            Color::Black => -1.
        };

        if depth == self.max_depth {
            return evaluator.evaluate(chess_board);
        } 

        let (node_ref, was_visited) = {
            if let Some(node_ref) = self.get_mut_node(chess_board) {
                (node_ref, true)
            } else {
                let allowed_moves = chess_board.get_allowed_moves(color);
                let priors = self.policy.get_priors(chess_board, &allowed_moves);

                let value = evaluator.evaluate(chess_board);

                let node = Rc::new(
                    RefCell::new(
                        MctsNode::new(
                            allowed_moves,
                            priors,
                            value.0
                        )
                    )
                );

                self.insert_cache(chess_board, node.clone());
                
                // It was never visited, return eval
                return value;
            }
        };
        
        let node_ref = node_ref.borrow_mut();
        let allowed_moves = &node_ref.allowed_moves;

        match chess_board.get_game_status_from_precomputed(&allowed_moves) {
            ChessStatus::Ongoing => {
                if !was_visited {
                    return evaluator.evaluate(chess_board);
                }
                // Was visited before
                
                let mut best_action = None;
                let mut max_ubc = -1000.;

                let total_visits: usize = node_ref.action_count.iter().sum();

                for i in 0..allowed_moves.len() {
                    let frac_action = (1. + total_visits as f64).sqrt() / (1. + node_ref.action_count[i] as f64);
                    let u = node_ref.q_values[i] + self.c_puct * node_ref.priors[i] * frac_action;

                    if u > max_ubc {
                        best_action = Some(i);
                        max_ubc = u;
                    }
                }

                let mv = allowed_moves[best_action.unwrap()];

                drop(node_ref);
                let v = self.search_internals(
                    &chess_board.next_state(&mv), 
                    evaluator, 
                    depth + 1
                );

                let node_ref = self.get_mut_node(chess_board).unwrap();
                let mut node_ref = node_ref.borrow_mut();

                let current_value =  node_ref.q_values[best_action.unwrap()] as f64;
                let current_action_count = node_ref.action_count[best_action.unwrap()];

                let node_value = node_ref.node_value;

                node_ref.q_values[best_action.unwrap()] += (
                    ( (v - node_value) * sign - current_value) 
                    / (current_action_count as f64 + 1.)
                ).0;

                node_ref.action_count[best_action.unwrap()] += 1;

                v
            },
            ChessStatus::BlackWon => BLACK_WON_EVAL,
            ChessStatus::WhiteWon => WHITE_WON_EVAL,
            ChessStatus::Draw => DRAW_EVAL,
        } 

    }

    fn search_impl(
        &self, chess_board: &ChessBoard, 
        evaluator: &E, 
    ) -> (OrderedFloat<f64>, Option<Move>) {
        
        // self.cache.borrow_mut().clear();

        // Execute MCTS
        for _ in 1..self.max_iter {
            self.search_internals(chess_board, evaluator, 0);
        };

        let node_ref = self.get_mut_node(chess_board).unwrap();
        let node_ref = node_ref.borrow();

        // let sign = match chess_board.get_turn_color()  {
        //     Color::White => 1.,
        //     Color::Black => -1.
        // };

        let mut best_action = None;
        // let mut max_value = -1000.;
        let mut max_value = 0;


        info!("");
        info!("Move Eval:");
        for i in 0..node_ref.allowed_moves.len() {
            if node_ref.action_count[i] > max_value {
                best_action = Some(i);
                max_value = node_ref.action_count[i];
                // continue;
            }

            info!("Move {}, Count: {}, Prior: {}, Q-Value: {}", 
                node_ref.allowed_moves[i], 
                node_ref.action_count[i], 
                100. * node_ref.priors[i],
                100. * node_ref.q_values[i],
            );

            // let u = node_ref.q_values[i] * sign;

            // if u > max_value {
            //     best_action = Some(i);
            //     max_value = u;
            // }
        }
        info!("");

        let eval = OrderedFloat(node_ref.q_values[best_action.unwrap()]);
        let mv = Some(node_ref.allowed_moves[best_action.unwrap()]);
        
        (eval, mv)
    }
}

impl<E: Evaluator, P: Policy> Searcher<E> for MonteCarloTreeSearch<E, P> {
    fn search(&self, chess_board: &ChessBoard, evaluator: &E) -> Move {

        let (eval, mv) = self.search_impl(chess_board, evaluator);
        
        info!("Size of cache: {}", self.cache.borrow().len());
        info!("Evaluation: {} with move {}", eval, mv.unwrap());

        mv.unwrap()
    }
}
