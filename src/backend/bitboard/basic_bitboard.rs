use std::convert::Infallible;

use rand::seq::index;

use crate::chess_server::types::*;

#[derive(Clone, Copy)]
pub struct BasicBitBoard {
    // Occupied Squares
    b_occupied: u64,

    // Pieces 
    b_pawns: u64,
    b_bishops: u64,
    b_knights: u64,
    b_rooks: u64,
    b_queens: u64,
    b_kings: u64,

    // Color
    b_color: u64,

    turn_color: Color
}

impl BasicBitBoard {
    pub fn get_turn_color(&self) -> Color {
        self.turn_color
    }

    pub fn get_cell_content(&self, coordinate: &Coordinate) -> Option<ColorPiece> {
        let indexer = 1 << coordinate.to_indexer();

        if (indexer | self.b_occupied) == 0 {
            return None;
        }

        // White piece
        if (indexer | self.b_color) != 0 {
            if (indexer | self.b_pawns) != 0 {
                return Some(ColorPiece { color: Color::White, piece: Piece::Pawn });
            } 
            if (indexer | self.b_bishops) != 0 {
                return Some(ColorPiece { color: Color::White, piece: Piece::Bishop });
            } 
            if (indexer | self.b_knights) != 0 {
                return Some(ColorPiece { color: Color::White, piece: Piece::Knight });
            } 
            if (indexer | self.b_rooks) != 0 {
                return Some(ColorPiece { color: Color::White, piece: Piece::Rook });
            } 
            if (indexer | self.b_queens) != 0 {
                return Some(ColorPiece { color: Color::White, piece: Piece::Queen });
            } 
            if (indexer | self.b_kings) != 0 {
                return Some(ColorPiece { color: Color::White, piece: Piece::King });
            } 
            
        } else {
            if (indexer | self.b_pawns) != 0 {
                return Some(ColorPiece { color: Color::Black, piece: Piece::Pawn });
            } 
            if (indexer | self.b_bishops) != 0 {
                return Some(ColorPiece { color: Color::Black, piece: Piece::Bishop });
            } 
            if (indexer | self.b_knights) != 0 {
                return Some(ColorPiece { color: Color::Black, piece: Piece::Knight });
            } 
            if (indexer | self.b_rooks) != 0 {
                return Some(ColorPiece { color: Color::Black, piece: Piece::Rook });
            } 
            if (indexer | self.b_queens) != 0 {
                return Some(ColorPiece { color: Color::Black, piece: Piece::Queen });
            } 
            if (indexer | self.b_kings) != 0 {
                return Some(ColorPiece { color: Color::Black, piece: Piece::King });
            } 
        }

        unreachable!("Fail on get_cell_content")
    }

    pub fn get_next_state(&self, move_: &Move) -> BasicBitBoard {
        let mut next_state = self.clone();

        next_state.turn_color = match self.turn_color {
            Color::Black => Color::White,
            Color::White => Color::Black
        };

        let next_piece = match move_ {
            Move::NormalMove(_) => self.get_cell_content(&move_.get_current_square()).unwrap(),
            Move::PromotionMove(_, piece) => Some(ColorPiece { color: self.turn_color, piece: *piece }).unwrap()
        };

        let mask_free = !(1 << move_.get_current_square().to_indexer());

        next_state.b_occupied &= mask_free;

        next_state.b_pawns &= mask_free;
        next_state.b_bishops &= mask_free;
        next_state.b_kings &= mask_free;
        next_state.b_rooks &= mask_free;
        next_state.b_queens &= mask_free;
        next_state.b_kings &= mask_free;

        next_state.b_color &= mask_free;

        let mask_enter = 1 << move_.get_next_square().to_indexer();

        next_state.b_occupied |= mask_enter;

        if next_piece.color == Color::White {
            next_state.b_color |= mask_enter;
        } else {
            next_state.b_color &= !mask_enter;
        }

        match next_piece.piece {
            Piece::Pawn => next_state.b_pawns |= mask_enter,
            Piece::Bishop => next_state.b_bishops |= mask_enter,
            Piece::Knight => next_state.b_knights |= mask_enter,
            Piece::Rook => next_state.b_rooks |= mask_enter,
            Piece::Queen => next_state.b_queens |= mask_enter,
            Piece::King => next_state.b_kings |= mask_enter,
        }

        next_state
    }


}