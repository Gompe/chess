


use crate::chess_server::chess_types::{ChessBoard, ChessStatus};
use crate::engines::engine_traits::*;

use ordered_float::OrderedFloat;

use rand::prelude::*;
use rand::distributions::WeightedIndex;

use smallvec::{SmallVec};
use std::thread;

#[derive(Clone)]
pub struct StochasticRollout<P: Policy, E: Evaluator>
where P: Send + Sync, E: Send + Sync {
    policy: P,
    evaluator: E,
    max_depth: usize,
    iter_per_thread: usize
}

unsafe impl<P: Policy, E: Evaluator> Send for StochasticRollout<P, E> 
where P: Send + Sync, E: Send + Sync {}

unsafe impl<P: Policy, E: Evaluator> Sync for StochasticRollout<P, E> 
where P: Send + Sync, E: Send + Sync {}

impl<P: Policy, E: Evaluator> StochasticRollout<P, E> 
where P: Send + Sync, E: Send + Sync  {
    pub fn new(policy: P, evaluator: E, max_depth: usize, iter_per_thread: usize) -> StochasticRollout<P, E> {
        if max_depth == 0 {
            panic!("Max Depth must be positive")
        }

        StochasticRollout { policy, evaluator, max_depth, iter_per_thread }
    }
}

impl<P: Policy, E:Evaluator> StochasticRollout<P, E> 
where P: Send + Sync, E: Send + Sync  {
    fn evaluate_monte_carlo(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        
        let mut rng = rand::thread_rng();

        let iteration = |rng: &mut ThreadRng| {
            let mut chess_board = *chess_board;
            let mut depth = 0;
            
            loop {
                depth += 1;
    
                let allowed_moves = chess_board.get_allowed_moves(chess_board.get_turn_color());
    
                match chess_board.get_game_status_from_precomputed(&allowed_moves) {
                    ChessStatus::Draw => return OrderedFloat(0.),
                    ChessStatus::WhiteWon => return OrderedFloat(1.),
                    ChessStatus::BlackWon => return OrderedFloat(-1.),
                    ChessStatus::Ongoing => {
                        
                        let priors = self.policy.get_priors(&chess_board, &allowed_moves);
    
                        let index = WeightedIndex::new(&priors).unwrap().sample(rng);
    
                        let mv = allowed_moves[index];
    
                        chess_board = chess_board.next_state(&mv);
                    }
                }
    
                if depth >= self.max_depth {
                    break;
                }
            }
    
            self.evaluator.evaluate(&chess_board)
        };

        let mut sum = OrderedFloat(0.);

        for _ in 0..self.iter_per_thread {
            sum += iteration(&mut rng);
        }

        sum / OrderedFloat( self.iter_per_thread as f64 )
    }
}

impl<P: Policy, E: Evaluator> Evaluator for StochasticRollout<P, E> 
where P: Send + Sync, E: Send + Sync {

    fn get_name(&self) -> String {
        format!("StochasticRollout({})", self.evaluator.get_name())
    }
    
    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        
        const N_THREADS: usize = 16;

        let sum: OrderedFloat<f64> = thread::scope(|s|{
            let mut handles: SmallVec<[_; N_THREADS]> = SmallVec::new();

            for _ in 0..N_THREADS {
                let handle = s.spawn(|| {
                    self.evaluate_monte_carlo(chess_board)
    
                });
    
                handles.push(handle);
            }

            let mut sum = OrderedFloat(0.);
            for handle in handles {
                sum += handle.join().unwrap()
            }

            sum
        });

        sum / OrderedFloat(N_THREADS as f64)
    }
}