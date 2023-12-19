

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
            Self::Pawn => String::from("p"),
            Self::Bishop => String::from("b"),
            Self::Knight => String::from("n"),
            Self::Rook => String::from("r"),
            Self::Queen => String::from("q"),
            Self::King => String::from("k")
        }
    }
}

