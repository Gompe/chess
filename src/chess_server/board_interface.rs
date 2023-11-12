
use crate::chess_server::types::*;

trait BoardInterface {

    pub fn starting_position() -> Self;

    // Getters
    pub fn get_turn_color(&self) -> Color;
    
    pub fn get_cell_content(&self, coordinate: &Coordinate) -> Option<ColorPiece>;
    pub fn iter_coordinates<'a>(&'a self) -> impl Iterator<Item = (Coordinate, Option<ColorPiece>)> + 'a;

    pub fn get_next_state(&self, move_: &Move) -> Self;

    pub fn get_allowed_moves(&self, color: Color) -> Vec<Move>;
    pub fn get_game_status(&self) -> GameStatus;
    
}

trait BoardInterfaceExtensions: BoardInterface{
    pub fn squares_attacked_by_piece(&self, coordinate: &Coordinate) -> Vec<Coordinate>;
    pub fn find_king(&self, color: Color) -> Coordinate;
    pub fn is_king_in_check(&self, color: Color) -> Coordinate;
    pub fn is_square_attacked_by_color(&self, coordinate: &Coordinate, color: Color) -> bool;
    pub fn contains_piece_of_color(&self, coordinate: &Coordinate, color: Color) -> bool;
}