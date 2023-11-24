use std::num::NonZeroU8;

use super::color::Color;
use super::piece::Piece;

pub const BLACK_PAWN : ColorPiece = ColorPiece::new(Color::Black, Piece::Pawn); 
pub const BLACK_BISHOP : ColorPiece = ColorPiece::new(Color::Black, Piece::Bishop); 
pub const BLACK_KNIGHT : ColorPiece = ColorPiece::new(Color::Black, Piece::Knight); 
pub const BLACK_ROOK : ColorPiece = ColorPiece::new(Color::Black, Piece::Rook); 
pub const BLACK_QUEEN : ColorPiece = ColorPiece::new(Color::Black, Piece::Queen); 
pub const BLACK_KING : ColorPiece = ColorPiece::new(Color::Black, Piece::King); 

pub const WHITE_BISHOP : ColorPiece = ColorPiece::new(Color::White, Piece::Bishop); 
pub const WHITE_KNIGHT : ColorPiece = ColorPiece::new(Color::White, Piece::Knight); 
pub const WHITE_ROOK : ColorPiece = ColorPiece::new(Color::White, Piece::Rook); 
pub const WHITE_PAWN : ColorPiece = ColorPiece::new(Color::White, Piece::Pawn); 
pub const WHITE_QUEEN : ColorPiece = ColorPiece::new(Color::White, Piece::Queen); 
pub const WHITE_KING : ColorPiece = ColorPiece::new(Color::White, Piece::King); 


pub type ColorPiece = BitColorPiece; 

impl ColorPiece {
    pub fn to_str(maybe_color_piece : &Option<ColorPiece>) -> String {
        match *maybe_color_piece {
            None => String::from(" "),

            Some(BLACK_KING) => String::from("k"),
            Some(BLACK_QUEEN) => String::from("q"),
            Some(BLACK_ROOK) => String::from("r"),
            Some(BLACK_BISHOP) => String::from("b"),
            Some(BLACK_KNIGHT) => String::from("n"),
            Some(BLACK_PAWN) => String::from("p"),

            Some(WHITE_KING) => String::from("K"),
            Some(WHITE_QUEEN) => String::from("Q"),
            Some(WHITE_ROOK) => String::from("R"),
            Some(WHITE_BISHOP) => String::from("B"),
            Some(WHITE_KNIGHT) => String::from("N"),
            Some(WHITE_PAWN) => String::from("P"),

            Some(_) => unreachable!()
        }
    }
}

/// BitColorPiece

const PAWN_ID : u8 = 1;
const BISHOP_ID : u8 = 2;
const KNIGHT_ID : u8 = 3;
const ROOK_ID : u8 = 4;
const QUEEN_ID : u8 = 5;
const KING_ID : u8 = 6;

const WHITE_ID : u8 = 8;
const BLACK_ID : u8 = 0;

const MASK_PIECE : u8 = 0b0111;
const MASK_COLOR : u8 = 0b1000;


// Later try to implement non-nullable optimization
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct BitColorPiece {
    payload: NonZeroU8,
}

impl BitColorPiece {
    #[inline(always)]
    pub const fn new(color: Color, piece: Piece) -> Self {
        let piece_mask = match piece {
            Piece::Pawn => PAWN_ID,
            Piece::Bishop => BISHOP_ID,
            Piece::Knight => KNIGHT_ID,
            Piece::Rook => ROOK_ID,
            Piece::Queen => QUEEN_ID,
            Piece::King => KING_ID,
        };

        let color_mask = match color {
            Color::White => WHITE_ID,
            Color::Black => BLACK_ID
        };

        Self { payload: unsafe { NonZeroU8::new_unchecked(piece_mask | color_mask) } }
    }

    #[inline(always)]
    pub fn get_color(&self) -> Color {
        match self.payload.get() & MASK_COLOR {
            MASK_COLOR => Color::White,
            _ => Color::Black
        }
    }

    #[inline(always)]
    pub fn get_piece(&self) -> Piece {
        match self.payload.get() & MASK_PIECE {
            PAWN_ID => Piece::Pawn,
            BISHOP_ID => Piece::Bishop,
            KNIGHT_ID => Piece::Knight,
            ROOK_ID => Piece::Rook,
            QUEEN_ID => Piece::Queen,
            _ => Piece::King
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BasicColorPiece {
    color : Color,
    piece : Piece
}

impl BasicColorPiece {

    #[inline(always)]
    pub const fn new(color: Color, piece: Piece) -> Self {
        Self { color, piece }
    }

    #[inline(always)]
    pub fn get_color(&self) -> Color {
        self.color
    }

    #[inline(always)]
    pub fn get_piece(&self) -> Piece {
        self.piece
    }
}