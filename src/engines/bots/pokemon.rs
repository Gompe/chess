use crate::backend::*;
use crate::chess_server::game::Player;

pub struct Pokemon {
    engine: Box<dyn Player>,
}

impl Pokemon {
    pub fn new(engine: Box<dyn Player>) -> Pokemon {
        Pokemon { engine }
    }
}

impl Player for Pokemon {
    fn select_move(&self, chess_board: &ChessBoard) -> Move {
        self.engine.select_move(chess_board)
    }
}
