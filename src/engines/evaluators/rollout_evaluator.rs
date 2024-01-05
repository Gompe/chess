use crate::backend::{ChessBoard, ChessStatus, Color};
use crate::engines::engine_traits::*;

use ordered_float::OrderedFloat;

#[derive(Clone)]
pub struct RolloutEvaluator<P: Policy, E: Evaluator> {
    policy: P,
    evaluator: E,
    max_depth: usize,
}

unsafe impl<P: Policy, E: Evaluator> Send for RolloutEvaluator<P, E>
where
    P: Send,
    E: Send,
{
}

unsafe impl<P: Policy, E: Evaluator> Sync for RolloutEvaluator<P, E>
where
    P: Sync,
    E: Sync,
{
}

impl<P: Policy, E: Evaluator> RolloutEvaluator<P, E> {
    pub fn new(policy: P, evaluator: E, max_depth: usize) -> RolloutEvaluator<P, E> {
        if max_depth == 0 {
            panic!("Max Depth must be positive")
        }

        RolloutEvaluator {
            policy,
            evaluator,
            max_depth,
        }
    }
}

impl<P: Policy, E: Evaluator> Evaluator for RolloutEvaluator<P, E> {
    fn get_name(&self) -> String {
        format!("RolloutEvaluator({})", self.evaluator.get_name())
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        let mut chess_board = *chess_board;

        let sign = match chess_board.get_turn_color() {
            Color::White => OrderedFloat(1.),
            Color::Black => OrderedFloat(-1.),
        };

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

                    let index = (0..priors.len())
                        .max_by_key(|&index| OrderedFloat(priors[index]) * sign)
                        .unwrap();

                    let mv = allowed_moves[index];

                    chess_board = chess_board.next_state(&mv);
                }
            }

            if depth >= self.max_depth {
                break;
            }
        }

        self.evaluator.evaluate(&chess_board)
    }
}
