use std::fmt;

use super::piece::Piece;
use super::square::Square;

pub type Move = BitMove;

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.get_promotion_piece() {
            None => {
                write!(
                    f,
                    "{}{}",
                    self.get_current_square().to_str(),
                    self.get_next_square().to_str()
                )
            }
            Some(piece) => {
                write!(
                    f,
                    "{}{}{}",
                    self.get_current_square().to_str(),
                    self.get_next_square().to_str(),
                    piece.to_str()
                )
            }
        }
    }
}

/// BitMove

const PAWN_ID: u16 = 0;
const KING_ID: u16 = 0;
const BISHOP_ID: u16 = 0b0001 << 12;
const KNIGHT_ID: u16 = 0b0010 << 12;
const ROOK_ID: u16 = 0b0011 << 12;
const QUEEN_ID: u16 = 0b0100 << 12;

const MASK_CURRENT_SQUARE: u16 = (1 << 6) - 1;
const MASK_NEXT_SQUARE: u16 = (1 << 12) - (1 << 6);
const MASK_PIECE: u16 = 0b0111 << 12;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BitMove {
    // 6 bits current square
    // 6 bits next square
    // 3 bits piece
    payload: u16,
}

impl BitMove {
    pub const fn new_promotion_move(
        current_square: Square,
        next_square: Square,
        piece: Piece,
    ) -> BitMove {
        let piece_mask = match piece {
            Piece::Bishop => BISHOP_ID,
            Piece::Knight => KNIGHT_ID,
            Piece::Rook => ROOK_ID,
            Piece::Queen => QUEEN_ID,
            Piece::Pawn => 0,
            Piece::King => 0,
        };

        BitMove {
            payload: current_square.get_index() as u16
                | ((next_square.get_index() as u16) << 6)
                | piece_mask,
        }
    }

    pub const fn new_normal_move(current_square: Square, next_square: Square) -> BitMove {
        BitMove {
            payload: (current_square.get_index() as u16) | ((next_square.get_index() as u16) << 6),
        }
    }

    pub fn get_current_square(&self) -> Square {
        Square::from_index((self.payload & MASK_CURRENT_SQUARE) as i8).unwrap()
    }

    pub fn get_next_square(&self) -> Square {
        Square::from_index(((self.payload & MASK_NEXT_SQUARE) >> 6) as i8).unwrap()
    }

    // Special cases
    pub fn get_is_promotion(&self) -> bool {
        self.payload & MASK_PIECE != 0
    }

    pub fn get_promotion_piece(&self) -> Option<Piece> {
        match self.payload & MASK_PIECE {
            BISHOP_ID => Some(Piece::Bishop),
            KNIGHT_ID => Some(Piece::Knight),
            ROOK_ID => Some(Piece::Rook),
            QUEEN_ID => Some(Piece::Queen),
            _ => None,
        }
    }

    pub fn get_is_castle(&self) -> bool {
        false
    }

    pub fn get_is_enpassant(&self) -> bool {
        false
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BasicMove {
    PromotionMove(MoveCoordinates, Piece),
    NormalMove(MoveCoordinates),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MoveCoordinates {
    current_square: Square,
    next_square: Square,
}

impl BasicMove {
    pub const fn new_promotion_move(
        current_square: Square,
        next_square: Square,
        piece: Piece,
    ) -> BasicMove {
        BasicMove::PromotionMove(
            MoveCoordinates {
                current_square,
                next_square,
            },
            piece,
        )
    }

    pub const fn new_normal_move(current_square: Square, next_square: Square) -> BasicMove {
        BasicMove::NormalMove(MoveCoordinates {
            current_square,
            next_square,
        })
    }

    pub fn get_current_square(&self) -> Square {
        match *self {
            BasicMove::PromotionMove(move_coords, _) => move_coords.current_square,
            BasicMove::NormalMove(move_coords) => move_coords.current_square,
        }
    }

    pub fn get_next_square(&self) -> Square {
        match *self {
            BasicMove::PromotionMove(move_coords, _) => move_coords.next_square,
            BasicMove::NormalMove(move_coords) => move_coords.next_square,
        }
    }

    // Special cases
    pub fn get_is_promotion(&self) -> bool {
        match *self {
            BasicMove::NormalMove(_) => false,
            BasicMove::PromotionMove(_, _) => true,
        }
    }

    pub fn get_promotion_piece(&self) -> Option<Piece> {
        match *self {
            BasicMove::NormalMove(_) => None,
            BasicMove::PromotionMove(_, piece) => Some(piece),
        }
    }

    pub fn get_is_castle(&self) -> bool {
        false
    }

    pub fn get_is_enpassant(&self) -> bool {
        false
    }
}
