use std::vec;

use super::color::Color;
use super::piece::Piece;
use super::color_piece::ColorPiece;
use super::color_piece::{
    WHITE_PAWN, WHITE_BISHOP, WHITE_KNIGHT, WHITE_ROOK, WHITE_QUEEN, WHITE_KING,
    BLACK_PAWN, BLACK_BISHOP, BLACK_KNIGHT, BLACK_ROOK, BLACK_QUEEN, BLACK_KING
};

use super::square::Square;
use super::chess_status::ChessStatus;
use super::chess_move::Move;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChessBoard {
    board : [Option<ColorPiece>; 64],
    turn_color : Color
}

impl ChessBoard {

    pub fn get_square_content(&self, square: &Square) -> Option<ColorPiece> {
        *self.board.get(square.get_index() as usize).unwrap()
    }

    fn set_square_content(&mut self, square: &Square, maybe_color_piece: &Option<ColorPiece>) {
        *self.board.get_mut(square.get_index() as usize).unwrap() = *maybe_color_piece;
    }

    pub fn get_turn_color(&self) -> Color {
        self.turn_color
    }
    
    pub fn get_game_status(&self) -> ChessStatus {
        if self.get_allowed_moves(self.turn_color).len() > 0 {
            ChessStatus::Ongoing
        } else {
            if self.is_king_in_check(self.turn_color) {
                match self.turn_color {
                    Color::White => ChessStatus::BlackWon,
                    Color::Black => ChessStatus::WhiteWon
                }
            } else {
                ChessStatus::Draw
            }
        }
    }

    pub fn next_state(&self, mv: &Move) -> Self {

        let mut next_board = self.clone();

        next_board.turn_color = match self.turn_color {
            Color::Black => Color::White,
            Color::White => Color::Black
        };

        let next_piece = match mv.get_promotion_piece() {
            None => self.get_square_content(&mv.get_current_square()),
            Some(piece) => Some(ColorPiece::new(self.turn_color, piece))
        };
        
        next_board.set_square_content(&mv.get_current_square(), &None);
        next_board.set_square_content(&mv.get_next_square(), &next_piece);
        
        next_board
        
    }

    pub fn starting_position() -> ChessBoard {
        let board : [Option<ColorPiece>; 64] = [
            Some(BLACK_ROOK), Some(BLACK_KNIGHT), Some(BLACK_BISHOP), Some(BLACK_QUEEN), Some(BLACK_KING), Some(BLACK_BISHOP), Some(BLACK_KNIGHT), Some(BLACK_ROOK),
            Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN),
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN),
            Some(WHITE_ROOK), Some(WHITE_KNIGHT), Some(WHITE_BISHOP), Some(WHITE_QUEEN), Some(WHITE_KING), Some(WHITE_BISHOP), Some(WHITE_KNIGHT), Some(WHITE_ROOK),
        ];

        ChessBoard { board, turn_color: Color::White }
    }

    pub fn print_board(self) {

        println!("{}'s turn", match self.turn_color { Color::White => "White", Color::Black => "Black"});

        for _ in 0..8 {
            print!("+---");
        }
        print!("+");

        print!("\n");
        for (num, maybe_color_piece) in self.board.iter().enumerate() {
            print!("| {} ", ColorPiece::to_str(maybe_color_piece));
            if num % 8 == 7 {
                print!("|\n");
                for _ in 0..8 {
                    print!("+---");
                }
    
                print!("+\n");
            }

        }
    }

    fn free_squares_in_direction(&self, coordinate: &Square, delta_row: i8, delta_col: i8) -> Vec<Square> {
        assert!(delta_row != 0 || delta_col != 0);

        let mut output: Vec<Square> = Vec::new();
        let mut maybe_position = coordinate.add(delta_row, delta_col);

        while let Some(position) = maybe_position {
            output.push(position);
            if self.get_square_content(&position) != None { 
                break;
            }

            maybe_position = position.add(delta_row, delta_col);
        }

        output
    } 

    pub fn squares_attacked_by_piece(&self, coordinate: &Square) -> Vec<Square> {

        let color_piece = match self.get_square_content(coordinate) {
            Some(color_piece) => color_piece,
            None => return vec![]
        };


        let mut output = vec![];

        match color_piece.get_piece() {
            Piece::Rook => {
                output.append(&mut self.free_squares_in_direction(coordinate,  -1,  0 ));
                output.append(&mut self.free_squares_in_direction(coordinate,  1,  0 ));
                output.append(&mut self.free_squares_in_direction(coordinate,  0,  -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate,  0, 1 ));
            },
            Piece::Bishop => {
                output.append(&mut self.free_squares_in_direction(coordinate, 1, 1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 1, -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, -1,  1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, -1, -1 ));
            },
            Piece::Queen => {
                output.append(&mut self.free_squares_in_direction(coordinate, -1,  0 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 1, 0 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 0, -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 0, 1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 1,  1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 1, -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, -1,  1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, -1, -1 ));
            },
            Piece::Knight => {
                output = (vec![
                    coordinate.add( -2, -1),
                    coordinate.add( -2,  1),
                    coordinate.add( -1, -2),
                    coordinate.add( -1, 2),
                    coordinate.add( 2,  -1),
                    coordinate.add( 2, 1),
                    coordinate.add( 1, -2),
                    coordinate.add( 1, 2),
                ]).iter().filter_map(|&any| any).collect();
            },
            Piece::King => {
                output = (vec![
                    coordinate.add( 0, -1),
                    coordinate.add( 0,  1),
                    coordinate.add( -1, -1),
                    coordinate.add( -1, 0),
                    coordinate.add( -1,  1),
                    coordinate.add( 1, -1),
                    coordinate.add( 1, 0),
                    coordinate.add( 1, 1),
                ]).iter().filter_map(|&any| any).collect();
            },
            Piece::Pawn => {
                match color_piece.get_color() {
                    Color::White => {
                        output = vec![
                            coordinate.add(-1, -1),
                            coordinate.add(-1, 1),
                        ].iter().filter_map(|&any| any).collect();
                    },
                    Color::Black => {
                        output = vec![
                            coordinate.add(1, -1),
                            coordinate.add(1, 1),
                        ].iter().filter_map(|&any| any).collect();
                    }
                }
            }
        };

        output
    }

    // Returns true if the square could be captured by a piece of the specified
    // color. This ignores checks.
    fn capture_helper(&self, coordinate: &Square, color: Color) -> bool {
        match self.get_square_content(&coordinate) {
            None => true,
            Some(color_piece) => color_piece.get_color() != color
        }
    }

    fn get_potential_moves(&self, coordinate: &Square) -> Vec<Move> {
        let color_piece = match self.get_square_content(coordinate) {
            Some(color_piece) => color_piece,
            None => return vec![]
        };

        let mut output = vec![];
        // Castle
        // ...

        let make_move_coordinates = |other_coordinate: Square| -> (Square, Square) {
            (*coordinate, other_coordinate)
        };

        // Other Moves
        if color_piece.get_piece() == Piece::Pawn {
            let make_promotion_moves = |x : Square| -> Vec<Move> {
                let (current_square, next_square) = make_move_coordinates(x);
                vec![
                    Move::new_promotion_move(current_square, next_square, Piece::Bishop),
                    Move::new_promotion_move(current_square, next_square, Piece::Knight),
                    Move::new_promotion_move(current_square, next_square, Piece::Rook),
                    Move::new_promotion_move(current_square, next_square, Piece::Queen),
                ]
            };

            let mut pawn_moves = vec![];
            let (row, _) = coordinate.get_coordinates(); 

            if color_piece.get_color() == Color::White {
                let front_square = coordinate.add(-1, 0).unwrap();
                if self.get_square_content(&front_square) == None {
                    pawn_moves.push(front_square);
                    if row == 6 {
                        let front_square = front_square.add(-1, 0).unwrap();
                        if self.get_square_content(&front_square) == None {
                            pawn_moves.push(front_square);
                        }
                    }
                }
                
                if let Some(square) = coordinate.add(-1, -1) {
                    if let Some(other_color_piece) = self.get_square_content(&square){
                        if other_color_piece.get_color() == Color::Black {
                            pawn_moves.push(square)
                        }
                    }
                }
                
                if let Some(square) = coordinate.add(-1, 1) {
                    if let Some(other_color_piece) = self.get_square_content(&square){
                        if other_color_piece.get_color() == Color::Black {
                            pawn_moves.push(square)
                        }
                    }
                }

            } else {
                let front_square = coordinate.add(1, 0).unwrap();
                if self.get_square_content(&front_square) == None {
                    pawn_moves.push(front_square);
                    if row == 1 {
                        let front_square = front_square.add(1, 0).unwrap();
                        if self.get_square_content(&front_square) == None {
                            pawn_moves.push(front_square);
                        }
                    }
                }
                
                if let Some(square) = coordinate.add(1, -1) {
                    if let Some(other_color_piece) = self.get_square_content(&square){
                        if other_color_piece.get_color() == Color::White {
                            pawn_moves.push(square)
                        }
                    }
                }
                
                if let Some(square) = coordinate.add(1, 1) {
                    if let Some(other_color_piece) = self.get_square_content(&square){
                        if other_color_piece.get_color() == Color::White {
                            pawn_moves.push(square)
                        }
                    }
                }
            }

            if (color_piece.get_color() == Color::White && row == 1) || (color_piece.get_color() == Color::Black && row == 6) {
                output = pawn_moves
                            .iter()
                            .flat_map(|&x| make_promotion_moves(x))
                            .collect();
            } else {
                output = pawn_moves
                            .iter()
                            .map(|&x| {
                                let (current_square, next_square) = make_move_coordinates(x);
                                Move::new_normal_move(current_square, next_square)
                            })
                            .collect()
            }

        } else {
            let moves = self.squares_attacked_by_piece(coordinate);
            output = moves.iter().filter_map(|&coordinate| {
                            if self.capture_helper(&coordinate, color_piece.get_color()) {
                                let (current_square, next_square) = make_move_coordinates(coordinate);
                                Some(Move::new_normal_move(current_square, next_square))
                            } else {
                                None
                            }
                        })
                        .collect()
        }

        output
    }

    pub fn find_piece(&self, color_piece: ColorPiece) -> Option<Square> {
        for (index, content) in self.board.iter().enumerate() {
            if Some(color_piece) == *content {
                return Some(Square::from_index(index as i8).unwrap());
            } 
        }
        None
    }

    pub fn find_king(&self, color: Color) -> Square {
        match color {
            Color::White => self.find_piece(WHITE_KING).unwrap(),
            Color::Black => self.find_piece(BLACK_KING).unwrap(),
        }
    }

    pub fn is_king_in_check(&self, color: Color) -> bool {
        // find king

        let king_coordinate = self.find_king(color);
        self.is_square_attacked_by_color(&king_coordinate, match color {
            Color::Black => Color::White,
            Color::White => Color::Black
        })
    }

    pub fn is_square_attacked_by_color(&self, coordinate: &Square, color: Color) -> bool {
        for index in 0..64 {
            let this_coordinate = unsafe { Square::from_index_unchecked(index) };
            if self.contains_piece_of_color(&this_coordinate, color) {
                if self.squares_attacked_by_piece(&this_coordinate).contains(&coordinate) {
                    return true;
                }
            }
        }
        false
    }

    pub fn contains_piece_of_color(&self, coordinate: &Square, color: Color) -> bool {
        let content = self.get_square_content(coordinate);
        match content {
            Some(color_piece) => {
                color_piece.get_color() == color
            }
            None => false,
        }
    }

    pub fn get_allowed_moves(&self, color: Color) -> Vec<Move> {
        let mut output = vec![];

        for index in 0..64 {
            let coordinate = unsafe { Square::from_index_unchecked(index as i8) };


            match self.get_square_content(&coordinate) {
                Some(color_piece) => {
                    if color_piece.get_color() == color {
                        let maybe_moves = self.get_potential_moves(&coordinate);

                        output.append(&mut maybe_moves
                            .iter()
                            .filter_map(|&x| {
                                let next_board = self.next_state(&x);
                                if next_board.is_king_in_check(color_piece.get_color()) {
                                    None
                                } else {
                                    Some(x)
                                }
                            })
                            .collect()
                        );
                    }
                },
                None => ()
            } 
        }

        output
    }

    pub fn iter_coordinates<'a>(&'a self) -> impl Iterator<Item = (Square, Option<ColorPiece>)> + 'a {
        (0..64).map(move |index| (Square::from_index(index as i8).unwrap(), *self.board.get(index).unwrap()))
    }
}
