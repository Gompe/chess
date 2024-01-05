use std::iter::Sum;

use crate::backend::chess_board::{MoveContainer, MOVE_CONTAINER_SIZE};
use crate::backend::{ChessBoard, Color};
use crate::engines::engine_traits::*;

use ordered_float::OrderedFloat;
use smallvec::SmallVec;

#[derive(Clone)]
pub struct SoftmaxPolicy<E: Evaluator> {
    evaluator: E,
    temperature: f64,
}

unsafe impl<E: Evaluator> Send for SoftmaxPolicy<E> where E: Send {}

unsafe impl<E: Evaluator> Sync for SoftmaxPolicy<E> where E: Sync {}

impl<E: Evaluator> SoftmaxPolicy<E> {
    pub fn new(evaluator: E, temperature: f64) -> SoftmaxPolicy<E> {
        SoftmaxPolicy {
            evaluator,
            temperature,
        }
    }
}

impl<E: Evaluator> Policy for SoftmaxPolicy<E> {
    fn get_priors(
        &self,
        chess_board: &ChessBoard,
        moves: &MoveContainer,
    ) -> SmallVec<[f64; MOVE_CONTAINER_SIZE]> {
        let color = chess_board.get_turn_color();
        let sign = match color {
            Color::White => OrderedFloat(1.),
            Color::Black => OrderedFloat(-1.),
        };

        let mut output: SmallVec<[_; MOVE_CONTAINER_SIZE]> = moves
            .iter()
            .map(|x| {
                (sign * self.evaluator.evaluate(&chess_board.next_state(x)) / self.temperature)
                    .exp()
            })
            .collect();

        let scaling: f64 = output.iter().sum();
        for x in output.iter_mut() {
            *x /= scaling;
        }

        output
    }
}
