use super::{ChessBoard, ChessStatus, Move, Piece};

use std::time::Duration;

pub struct ChessGame {
    chess_board: ChessBoard,
    moves_since_start: usize,
    moves_since_progress: usize,

    time_used_white: Duration,
    time_used_black: Duration,

    game_outcome: ChessStatus
}

impl ChessGame {
    ////
    /// Creational methods

    pub fn new() -> Self {
        ChessGame {
            chess_board: ChessBoard::starting_position(),
            moves_since_start: 0,
            moves_since_progress: 0,
            time_used_white: Duration::ZERO,
            time_used_black: Duration::ZERO,
            game_outcome: ChessStatus::Ongoing
        }
    }

    pub fn set_position(&mut self, chess_board: &ChessBoard) {
        // Set new position and reset stats
        self.chess_board = *chess_board;

        self.moves_since_start = 0;
        self.moves_since_progress = 0;

        self.time_used_white = Duration::ZERO;
        self.time_used_black = Duration::ZERO;

        self.game_outcome = chess_board.get_game_status()
    }

    ////
    /// Game Status methods

    pub fn is_game_over(&self) -> bool {
        self.game_outcome != ChessStatus::Ongoing
    }

    pub fn get_board(&self) -> ChessBoard {
        self.chess_board
    }
    
    pub fn get_game_status(&self) -> ChessStatus {
        self.game_outcome
    }

    ////
    /// Transitional methods
    
    pub fn apply_move(&mut self, mv: &Move) -> Result<(), ()> {
        if self.is_game_over() { return Err(()) }

        if !self
            .chess_board
            .get_allowed_moves(self.chess_board.get_turn_color())
            .contains(mv)
        {
            return Err(());
        }

        self.moves_since_start += 1;
        
        let content_current = self.chess_board.get_square_content(&mv.get_current_square());
        let content_next = self.chess_board.get_square_content(&mv.get_next_square());

        if let Some(color_piece) = content_current {
            // Is a pawn move

            if color_piece.get_piece() == Piece::Pawn { 
                self.moves_since_progress = 0;
            }
        } else if let Some(color_piece) = content_next {
            // Is a capture

            self.moves_since_progress = 0;
        } else {
            self.moves_since_progress += 1;
        }
        
        self.chess_board = self.chess_board.next_state(mv);

        if self.moves_since_progress >= 50 {
            // 50 move rule => Game ends in a draw!
            self.game_outcome = ChessStatus::Draw;
        } else {
            self.game_outcome = self.chess_board.get_game_status()
        }

        Ok(())
    }

}
