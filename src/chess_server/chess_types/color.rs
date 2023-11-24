
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn to_str(&self) -> String {
        match *self {
            Self::White => String::from("W"),
            Self::Black => String::from("B")
        }
    }
}