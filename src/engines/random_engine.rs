use crate::backend::{ChessBoard, Move};
use crate::chess_server::game::Player;

use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng}; // 0.6.1
use std::cell::RefCell;

pub struct RandomEngine {
    rng: RefCell<ThreadRng>,
}

impl RandomEngine {
    pub fn new() -> RandomEngine {
        RandomEngine {
            rng: RefCell::new(thread_rng()),
        }
    }
}

impl Player for RandomEngine {
    fn select_move(&self, chess_board: &ChessBoard) -> Move {
        let allowed_moves = chess_board.get_allowed_moves(chess_board.get_turn_color());
        let mut rng = self.rng.borrow_mut();

        if let Some(random_move) = allowed_moves.choose(&mut *rng) {
            *random_move
        } else {
            panic!("Game is Over - No allowed moves.");
        }
    }
}
