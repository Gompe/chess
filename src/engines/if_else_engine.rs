use crate::chess_server::chess_types::{ChessBoard, Move};
use crate::chess_server::game::Player;

// 0.6.1

#[derive(Clone)]
pub struct IfElseEngine<P1: Player, P2: Player, F>
where
    F: Fn(&ChessBoard) -> bool,
{
    player_1: P1,
    player_2: P2,
    func: F,
}

impl<P1: Player, P2: Player, F> IfElseEngine<P1, P2, F>
where
    F: Fn(&ChessBoard) -> bool,
{
    pub fn new(player_1: P1, player_2: P2, func: F) -> IfElseEngine<P1, P2, F> {
        IfElseEngine {
            player_1,
            player_2,
            func,
        }
    }
}

impl<P1: Player, P2: Player, F> Player for IfElseEngine<P1, P2, F>
where
    F: Fn(&ChessBoard) -> bool,
{
    fn select_move(&self, chess_board: &ChessBoard) -> Move {
        if (self.func)(chess_board) {
            self.player_1.select_move(chess_board)
        } else {
            self.player_2.select_move(chess_board)
        }
    }
}
