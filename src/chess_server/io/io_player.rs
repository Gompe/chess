use crate::chess_server::chess_types::*;
use crate::chess_server::game::*;

use crate::chess_server::io::utils;

use std::io::stdin;

pub struct IOPlayer;

impl IOPlayer {
    pub fn new() -> IOPlayer {
        IOPlayer {}
    }
}

impl Player for IOPlayer {
    fn select_move(&self, chess_board: &ChessBoard) -> Move {
        let allowed_moves = chess_board.get_allowed_moves(chess_board.get_turn_color());

        loop {
            let mut s = String::new();
            stdin().read_line(&mut s).expect("Crashed waiting for move");

            let mv = utils::parse_move(&s);

            if allowed_moves.contains(&mv) {
                return mv;
            } else {
                println!("Invalid move: {}", mv);
            }
        }
    }
}
