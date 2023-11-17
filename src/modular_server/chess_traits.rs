

// Main Traits 

use super::primitive_types::{PrimitiveColor, PrimitivePiece, PrimitiveColorPiece, PrimitiveSquare, PrimitiveMove, PrimitiveChessBoard, ChessStatus};


pub trait ChessColor : Sized + Eq + Copy + Clone {
    fn to_primitive(&self) -> PrimitiveColor;
}

pub trait ChessPiece : Sized + Eq + Copy + Clone {
    fn to_primitive(&self) -> PrimitivePiece;
}

pub trait ChessColorPiece<Color: ChessColor, Piece: ChessPiece> : Sized + Eq + Copy + Clone 
{
    fn to_primitive(&self) -> PrimitiveColorPiece;

    fn get_color(&self) -> Color;
    fn get_piece(&self) -> Piece;
} 

pub trait ChessSquare : Sized {

    fn to_primitive(&self) -> PrimitiveSquare;

    unsafe fn from_coordinates_unchecked(row: i8, col: i8) -> Self;
    unsafe fn from_index_unchecked(index: i8) -> Self;

    fn from_coordinates(row: i8, col: i8) -> Option<Self> {
        if (row >= 0) && (row < 8) && (col >= 0) && (col < 8) 
        {
            unsafe {
                Some(Self::from_coordinates_unchecked(row, col))
            }
        } else {    
            None
        }
    }

    fn from_index(index: i8) -> Option<Self> {
        if (index >= 0) & (index < 64) {
            unsafe {
                Some(Self::from_index_unchecked(index))
            }
        } else {
            None
        }
    }
    
    fn get_coordinates(&self) -> (u8, u8);
    fn get_index(&self) -> u8;

    unsafe fn add_unchecked(&self, d_row: i8, d_col: i8) -> Self {
        let (row, col) = self.get_coordinates();
        Self::from_coordinates_unchecked(row as i8 + d_row, col as i8 + d_col)
    }

    fn add(&self, d_row: i8, d_col: i8) -> Option<Self> {
        let (row, col) = self.get_coordinates();
        
        let new_row = (row as i16) + (d_row as i16);
        let new_col = (col as i16) + (d_col as i16);

        if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
            unsafe {
                Some(Self::from_coordinates_unchecked(new_row as i8, new_col as i8))
            }
        } else{
            None
        }
    }

}

pub trait ChessMove<Square : ChessSquare, Piece : ChessPiece> {

    fn to_primitive(&self) -> PrimitiveMove;

    fn get_current_square(&self) -> Square;
    fn get_next_square(&self) -> Square;

    // Special cases
    fn get_is_promotion(&self) -> bool;
    fn get_promotion_piece(&self) -> Option<Piece>;

    fn get_is_castle(&self) -> bool;
    fn get_is_enpassant(&self) -> bool;
}

pub trait ChessBoard<
        Move : ChessMove<Square, Piece>, 
        Square: ChessSquare,
        ColorPiece: ChessColorPiece<Color, Piece>,
        Color: ChessColor,
        Piece: ChessPiece,
> {

    fn to_primitive(&self) -> PrimitiveChessBoard;

    fn get_square_content(&self, square: &Square) -> Option<ColorPiece>;
    fn set_square_content(&mut self, square: &Square, color_piece: &Option<ColorPiece>);

    
    fn get_turn_color(&self) -> Color;
    
    fn get_game_status(&self) -> ChessStatus;

    fn next_state(&self, mv: &Move) -> Self;
}

