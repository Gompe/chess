

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

    pub fn to_str(&self) -> String {
        match *self {
            Self::Pawn => String::from("P"),
            Self::Bishop => String::from("B"),
            Self::Knight => String::from("N"),
            Self::Rook => String::from("R"),
            Self::Queen => String::from("Q"),
            Self::King => String::from("K")
        }
    }
}

