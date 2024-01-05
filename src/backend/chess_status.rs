#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ChessStatus {
    Ongoing,
    WhiteWon,
    BlackWon,
    Draw,
}
