
use crate::chess_server::types::{ChessBoard, Move, Color, GameStatus};

pub trait Player {
    fn select_move(&self, chess_board: &ChessBoard) -> Move;
}


pub struct GameManager<'a, 'b> {
    chess_board: ChessBoard,
    player_white: &'a dyn Player,
    player_black: &'b dyn Player,
    game_status: GameStatus,
    round_number: u32,
}

impl<'a, 'b> GameManager<'a, 'b> {
    pub fn new(player_white: &'a dyn Player, player_black: &'b dyn Player) -> GameManager<'a, 'b> {
        GameManager{ 
            chess_board: ChessBoard::starting_position(), 
            player_white, 
            player_black,
            game_status: GameStatus::Ongoing,
            round_number: 0,
        }
    }

    pub fn get_board(&self) -> ChessBoard {
        self.chess_board.clone()
    }


    pub fn get_turn(&self) -> Color {
        self.chess_board.get_turn_color()
    }

    pub fn is_game_ongoing(&self) -> bool {
        self.game_status == GameStatus::Ongoing
    }


    pub fn make_move(&mut self) {
        let selected_move = match self.chess_board.get_turn_color() {
            Color::White => self.player_white.select_move(&self.chess_board),
            Color::Black => self.player_black.select_move(&self.chess_board)
        };

        self.round_number += 1;
        println!("Selected Move: {}", selected_move);

        self.chess_board.next_state_inplace(&selected_move);    

        // Check the game status
        self.game_status = self.chess_board.get_game_status();
        if self.game_status != GameStatus::Ongoing {
            println!("Game Over! Status: {:?}", self.game_status);
        }
    }

    pub fn show(&self) {
        println!("Round Number: {}", self.round_number);
        self.chess_board.print_board();
    }
}