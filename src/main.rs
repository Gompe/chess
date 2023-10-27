

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    White,
    Black
}

impl Color {
    fn to_str(color : &Color) -> String {
        match color {
            Self::White => String::from("W"),
            Self::Black => String::from("B")
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen, 
    King
}

impl Piece {
    pub fn to_str(piece : &Piece) -> String {
        match piece {
            Self::Pawn => String::from("P"),
            Self::Bishop => String::from("B"),
            Self::Knight => String::from("K"),
            Self::Rook => String::from("R"),
            Self::Queen => String::from("Q"),
            Self::King => String::from("K")
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ColorPiece {
    color : Color,
    piece : Piece
}

impl ColorPiece {
    pub fn to_str(maybe_color_piece : &Option<ColorPiece>) -> String {
        match maybe_color_piece {
            None => String::from("  "),
            Some(color_piece) => Color::to_str(&color_piece.color) + &Piece::to_str(&color_piece.piece)
        }
    }
}

pub const BLACK_PAWN : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Pawn }; 
pub const BLACK_BISHOP : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Bishop }; 
pub const BLACK_KNIGH : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Knight }; 
pub const BLACK_ROOK : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Rook }; 
pub const BLACK_QUEEN : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Queen }; 
pub const BLACK_KING : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::King }; 

pub const WHITE_BISHOP : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Bishop }; 
pub const WHITE_KNIGH : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Knight }; 
pub const WHITE_ROOK : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Rook }; 
pub const WHITE_PAWN : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Pawn }; 
pub const WHITE_QUEEN : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Queen }; 
pub const WHITE_KING : ColorPiece = ColorPiece {color : Color::White, piece : Piece::King }; 

pub struct ChessBoard {
    board : [Option<ColorPiece>; 64],
    turn_color : Color
}

impl ChessBoard {
    pub fn starting_position() -> ChessBoard {
        let board : [Option<ColorPiece>; 64] = [
            Some(BLACK_ROOK), Some(BLACK_KNIGH), Some(BLACK_BISHOP), Some(BLACK_QUEEN), Some(BLACK_KING), Some(BLACK_BISHOP), Some(BLACK_KNIGH), Some(BLACK_ROOK),
            Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN),
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN),
            Some(WHITE_ROOK), Some(WHITE_KNIGH), Some(WHITE_BISHOP), Some(WHITE_QUEEN), Some(WHITE_KING), Some(WHITE_BISHOP), Some(WHITE_KNIGH), Some(WHITE_ROOK),
        ];

        ChessBoard { board, turn_color: Color::White }
    }

    pub fn print_board(self) {
        for _ in 0..8 {
            print!(" ----");
        }

        print!("\n");
        for (num, maybe_color_piece) in self.board.iter().enumerate() {
            print!("| {} ", ColorPiece::to_str(maybe_color_piece));
            if num % 8 == 7 {
                print!("|\n");
                for _ in 0..8 {
                    print!("|----");
                }
    
                print!("|\n");
            }

        }
    }
}



fn main() {
    println!("Hello, world!");

    let board = ChessBoard::starting_position();
    board.print_board();

}
