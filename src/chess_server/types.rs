use std::vec;
use std::fmt;

use std::slice::Iter;

pub const BLACK_PAWN : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Pawn }; 
pub const BLACK_BISHOP : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Bishop }; 
pub const BLACK_KNIGHT : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Knight }; 
pub const BLACK_ROOK : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Rook }; 
pub const BLACK_QUEEN : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::Queen }; 
pub const BLACK_KING : ColorPiece = ColorPiece {color : Color::Black, piece : Piece::King }; 

pub const WHITE_BISHOP : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Bishop }; 
pub const WHITE_KNIGHT : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Knight }; 
pub const WHITE_ROOK : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Rook }; 
pub const WHITE_PAWN : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Pawn }; 
pub const WHITE_QUEEN : ColorPiece = ColorPiece {color : Color::White, piece : Piece::Queen }; 
pub const WHITE_KING : ColorPiece = ColorPiece {color : Color::White, piece : Piece::King }; 

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    pub fn from_chess_notation(coordinate: [char; 2]) -> Option<Coordinate> {
        let col = match coordinate[0] {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
            'e' | 'E' => 4,
            'f' | 'F' => 5,
            'g' | 'G' => 6,
            'h' | 'H' => 7,
            _ => return None
        };

        let row = match coordinate[1] {
            '1' => 7,
            '2' => 6,
            '3' => 5,
            '4' => 4,
            '5' => 3,
            '6' => 2,
            '7' => 1,
            '8' => 0,
            _ => return None
        };

        Some(Coordinate { row, col })
    }

    pub fn to_indexer(&self) -> usize {
        (8 * self.row + self.col) as usize
    }

    pub fn to_str(&self) -> String {
        let char_row = match self.row {
            0 => '8',
            1 => '7',
            2 => '6',
            3 => '5',
            4 => '4',
            5 => '3',
            6 => '2',
            7 => '1',
            _ => panic!("Panic => Coordinate {}, {}", self.row, self.col)
        };

        let char_col = match self.col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Panic => Coordinate {}, {}", self.row, self.col)
        };
    
        char_col.to_string() + &char_row.to_string()    
    }

    fn add(&self, delta: &CoordinateDelta) -> Coordinate {
        Coordinate { row: self.row + delta.delta_row, col: self.col + delta.delta_col }
    }

    pub fn is_inside_board(&self) -> bool {
        (self.row >= 0) && (self.row < 8) && (self.col >= 0) && (self.col < 8)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CoordinateDelta {
    delta_row: i32,
    delta_col: i32,
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ColorPiece {
    pub color : Color,
    pub piece : Piece
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MoveCoordinates {
    current_square: Coordinate,
    next_square: Coordinate,
}

impl MoveCoordinates {
    pub fn from_chess_notation(current_square: [char; 2], next_square: [char; 2]) -> Option<MoveCoordinates> {

        let current_square = match Coordinate::from_chess_notation(current_square) {
            Some(coordinate) => coordinate,
            None => return None
        };

        let next_square = match Coordinate::from_chess_notation(next_square) {
            Some(coordinate) => coordinate,
            None => return None
        };

        Some( MoveCoordinates { current_square, next_square } )
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    PromotionMove(MoveCoordinates, Piece),
    NormalMove(MoveCoordinates)
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Move::NormalMove(move_coordinates) => {
                write!(f, "Move: {}->{}", 
                    move_coordinates.current_square.to_str(), 
                    move_coordinates.next_square.to_str()
                )
            }, 
            Move::PromotionMove(move_coordinates, piece) => {
                write!(f, "Move: {}->{}, ={}", 
                    move_coordinates.current_square.to_str(), 
                    move_coordinates.next_square.to_str(),
                    piece.to_str()
                )
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct ChessBoard {
    board : [Option<ColorPiece>; 64],
    turn_color : Color
}

impl ChessBoard {
    pub fn get_cell_content(&self, coordinate: &Coordinate) -> Option<ColorPiece> {
        *self.board.get(coordinate.to_indexer()).unwrap()
    }

    pub fn set_cell_content(&mut self, coordinate: &Coordinate, content: Option<ColorPiece>) {
        *self.board.get_mut(coordinate.to_indexer()).unwrap() = content;
    }
}


impl Color {
    pub fn to_str(&self) -> String {
        match *self {
            Self::White => String::from("W"),
            Self::Black => String::from("B")
        }
    }
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
            
        }

        // match maybe_color_piece {
        //     None => String::from("  "),
        //     Some(color_piece) => color_piece.color.to_str() + &color_piece.piece.to_str()
        // }
    }
}

impl ChessBoard {
    pub fn get_turn_color(&self) -> Color {
        self.turn_color
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
}

// Slow, but safe implementation of Move
impl ChessBoard {
    pub fn next_state(&self, move_ : &Move) -> ChessBoard {
        
        let move_ = match move_ {
            Move::NormalMove(move_coords) => move_coords,
            _ => panic!()
        };

        let mut next_board = self.clone();

        next_board.turn_color = match self.turn_color {
            Color::Black => Color::White,
            Color::White => Color::Black
        };

        let moved_piece = next_board.get_cell_content(&move_.current_square);
        
        next_board.set_cell_content(&move_.current_square, None);
        next_board.set_cell_content(&move_.next_square, moved_piece);

        next_board
    }

    pub fn next_state_inplace(&mut self, move_ : &Move) {
        let move_ = match move_ {
            Move::NormalMove(move_coords) => move_coords,
            _ => panic!()
        };
        
        
        self.turn_color = match self.turn_color {
            Color::Black => Color::White,
            Color::White => Color::Black
        };
        
        let moved_piece = self.get_cell_content(&move_.current_square);

        self.set_cell_content(&move_.current_square, None);
        self.set_cell_content(&move_.next_square, moved_piece);

    }
}


pub fn collect_valid_coordinates(coordinates: &Vec<Coordinate>) -> Vec<Coordinate> {
    coordinates.iter().filter_map(|&coordinate| {
        if coordinate.is_inside_board() {
            Some(coordinate)
        } else {
            None
        }
    })
    .collect()
}

impl ChessBoard {
    fn free_squares_in_direction(&self, coordinate: &Coordinate, direction: &CoordinateDelta) -> Vec<Coordinate> {
        assert_ne!(*direction, CoordinateDelta{delta_row: 0, delta_col: 0}, "direction is 0 in free_squares_in_direction");
        
        let mut output: Vec<Coordinate> = Vec::new();
        
        let mut position = coordinate.add(direction);
        while position.is_inside_board() {
            output.push(position);
    
            if let Some(_) = self.get_cell_content(&position) {
                break;
            }
    
            position = position.add(direction);
        }
    
        output
    } 

    pub fn squares_attacked_by_piece(&self, coordinate: &Coordinate) -> Vec<Coordinate> {

        let color_piece = match self.get_cell_content(coordinate) {
            Some(color_piece) => color_piece,
            None => return vec![]
        };


        let mut output = vec![];

        match color_piece.piece {
            Piece::Rook => {
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: -1, delta_col: 0 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 1, delta_col: 0 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 0, delta_col: -1 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 0, delta_col: 1 }));
            },
            Piece::Bishop => {
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: -1, delta_col: -1 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 1, delta_col: -1 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: -1, delta_col: 1 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 1, delta_col: 1 }));
            },
            Piece::Queen => {
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: -1, delta_col: 0 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 1, delta_col: 0 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 0, delta_col: -1 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 0, delta_col: 1 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: -1, delta_col: -1 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 1, delta_col: -1 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: -1, delta_col: 1 }));
                output.append(&mut self.free_squares_in_direction(coordinate, &CoordinateDelta { delta_row: 1, delta_col: 1 }));
            },
            Piece::Knight => {
                output = collect_valid_coordinates(&vec![
                    coordinate.add(&CoordinateDelta {delta_row: -2, delta_col: -1}),
                    coordinate.add(&CoordinateDelta {delta_row: -2, delta_col: 1}),
                    coordinate.add(&CoordinateDelta {delta_row: -1, delta_col: -2}),
                    coordinate.add(&CoordinateDelta {delta_row: -1, delta_col: 2}),
                    coordinate.add(&CoordinateDelta {delta_row: 2, delta_col: -1}),
                    coordinate.add(&CoordinateDelta {delta_row: 2, delta_col: 1}),
                    coordinate.add(&CoordinateDelta {delta_row: 1, delta_col: -2}),
                    coordinate.add(&CoordinateDelta {delta_row: 1, delta_col: 2}),
                ]);
            },
            Piece::King => {
                output = collect_valid_coordinates(&vec![
                    coordinate.add(&CoordinateDelta {delta_row: 0, delta_col: -1}),
                    coordinate.add(&CoordinateDelta {delta_row: 0, delta_col: 1}),
                    coordinate.add(&CoordinateDelta {delta_row: -1, delta_col: -1}),
                    coordinate.add(&CoordinateDelta {delta_row: -1, delta_col: 0}),
                    coordinate.add(&CoordinateDelta {delta_row: -1, delta_col: 1}),
                    coordinate.add(&CoordinateDelta {delta_row: 1, delta_col: -1}),
                    coordinate.add(&CoordinateDelta {delta_row: 1, delta_col: 0}),
                    coordinate.add(&CoordinateDelta {delta_row: 1, delta_col: 1}),
                ]);
            },
            Piece::Pawn => {
                match color_piece.color {
                    Color::White => {
                        output = collect_valid_coordinates(&vec![
                            coordinate.add(&CoordinateDelta { delta_row: -1, delta_col: -1 }),
                            coordinate.add(&CoordinateDelta { delta_row: -1, delta_col: 1 }),
                        ]);
                    },
                    Color::Black => {
                        output = collect_valid_coordinates(&vec![
                            coordinate.add(&CoordinateDelta { delta_row: 1, delta_col: -1 }),
                            coordinate.add(&CoordinateDelta { delta_row: 1, delta_col: 1 }),
                        ]);
                    }
                }
            }
        };

        output
    }

// Returns true if the square could be captured by a piece of the specified
// color. This ignores checks.
fn capture_helper(&self, coordinate: &Coordinate, color: Color) -> bool {

    if coordinate.is_inside_board() {
        let content = self.get_cell_content(&coordinate);
        match content {
            None => return true,
            Some(other_piece) => {
                if other_piece.color != color {
                    return true
                }
            }
        }
    };

    false
}

fn get_potential_moves(&self, coordinate: &Coordinate) -> Vec<Move> {
    let color_piece = match self.get_cell_content(coordinate) {
        Some(color_piece) => color_piece,
        None => return vec![]
    };

    let mut output = vec![];
    // Castle
    // ...

    let make_move_coordinates = |other_coordinate: Coordinate| -> MoveCoordinates {
        MoveCoordinates { current_square: *coordinate, next_square: other_coordinate }
    };

    // Other Moves
    if color_piece.piece == Piece::Pawn {
        let make_promotion_moves = |x : Coordinate| -> Vec<Move> {
            vec![
                Move::PromotionMove(make_move_coordinates(x), Piece::Rook),
                Move::PromotionMove(make_move_coordinates(x), Piece::Queen),
                Move::PromotionMove(make_move_coordinates(x), Piece::Bishop),
                Move::PromotionMove(make_move_coordinates(x), Piece::Knight),
            ]
        };

        let mut pawn_moves = vec![];

        if color_piece.color == Color::White {
            let front_square = coordinate.add(&CoordinateDelta { delta_row: -1, delta_col: 0 });
            if self.get_cell_content(&front_square) == None {
                pawn_moves.push(front_square);
                if coordinate.row == 6 {
                    let front_square = front_square.add(&CoordinateDelta { delta_row: -1, delta_col: 0 });
                    if self.get_cell_content(&front_square) == None {
                        pawn_moves.push(front_square);
                    }
                }
            }
            
            let square = coordinate.add(&CoordinateDelta { delta_row: -1, delta_col: -1 });
            if square.is_inside_board() {
                if let Some(other_color_piece) = self.get_cell_content(&square){
                    if other_color_piece.color == Color::Black {
                        pawn_moves.push(square)
                    }
                }
            }
            
            let square = coordinate.add(&CoordinateDelta { delta_row: -1, delta_col: 1 });
            if square.is_inside_board() {
                if let Some(other_color_piece) = self.get_cell_content(&square){
                    if other_color_piece.color == Color::Black {
                        pawn_moves.push(square)
                    }
                }
            }

        } else {
            let front_square = coordinate.add(&CoordinateDelta { delta_row: 1, delta_col: 0 });
            if self.get_cell_content(&front_square) == None {
                pawn_moves.push(front_square);
                if coordinate.row == 1 {
                    let front_square = front_square.add(&CoordinateDelta { delta_row: 1, delta_col: 0 });
                    if self.get_cell_content(&front_square) == None {
                        pawn_moves.push(front_square);
                    }
                }
            }
            
            let square = coordinate.add(&CoordinateDelta { delta_row: 1, delta_col: -1 });
            if square.is_inside_board() {
                if let Some(other_color_piece) = self.get_cell_content(&square){
                    if other_color_piece.color == Color::White {
                        pawn_moves.push(square)
                    }
                }
            }
            
            let square = coordinate.add(&CoordinateDelta { delta_row: 1, delta_col: 1 });
            if square.is_inside_board() {
                if let Some(other_color_piece) = self.get_cell_content(&square){
                    if other_color_piece.color == Color::White {
                        pawn_moves.push(square)
                    }
                }
            }
        }

        if (color_piece.color == Color::White && coordinate.row == 1) || (color_piece.color == Color::Black && coordinate.row == 6) {
            output = pawn_moves
                        .iter()
                        .flat_map(|&x| make_promotion_moves(x))
                        .collect();
        } else {
            output = pawn_moves
                        .iter()
                        .map(|&x| Move::NormalMove(make_move_coordinates(x)))
                        .collect()
        }

    } else {
        let moves = self.squares_attacked_by_piece(coordinate);
        output = moves.iter().filter_map(|&coordinate| {
                        if self.capture_helper(&coordinate, color_piece.color) {
                            Some(Move::NormalMove(make_move_coordinates(coordinate)))
                        } else {
                            None
                        }
                    })
                    .collect()
    }

    output
}

}

impl Coordinate {
    pub fn from_indexer(index: usize) -> Coordinate {
        let row: i32 = index as i32 / 8;
        let col: i32 = index as i32 - 8 * row;
        
        // Debug Helper 
        let coordinate = Coordinate { row, col };
        if !coordinate.is_inside_board() {
            panic!("Created Coordinate from indexer {}", index);
        }

        coordinate
    }
}

impl ChessBoard {
    pub fn find_piece(&self, color_piece: ColorPiece) -> Option<Coordinate> {
        for (index, content) in self.board.iter().enumerate() {
            if Some(color_piece) == *content {
                return Some(Coordinate::from_indexer(index));
            } 
        }
        None
    }

    pub fn find_king(&self, color: Color) -> Coordinate {
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

    pub fn is_square_attacked_by_color(&self, coordinate: &Coordinate, color: Color) -> bool {
        for index in 0..64 {
            let this_coordinate = Coordinate::from_indexer(index as usize);
            if self.contains_piece_of_color(&this_coordinate, color) {
                if self.squares_attacked_by_piece(&this_coordinate).contains(&coordinate) {
                    return true;
                }
            }
        }
        false
    }

    pub fn contains_piece_of_color(&self, coordinate: &Coordinate, color: Color) -> bool {
        let content = self.get_cell_content(coordinate);
        match content {
            Some(color_piece) => {
                color_piece.color == color
            }
            None => false,
        }
    }


}




impl ChessBoard {


pub fn get_allowed_moves(&self, color: Color) -> Vec<Move> {
    let mut output = vec![];
    for index in 0..64 {
        let coordinate = Coordinate::from_indexer(index as usize);
        match self.get_cell_content(&coordinate) {
            Some(color_piece) => {
                if color_piece.color == color {
                    let maybe_moves = self.get_potential_moves(&coordinate);
                    output.append(&mut maybe_moves
                        .iter()
                        .filter_map(|&x| {
                            let next_board = self.next_state(&x);
                            if next_board.is_king_in_check(color_piece.color) {
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

}

#[derive(Debug, PartialEq, Eq)]
pub enum GameStatus {
    Ongoing,
    WhiteWon,
    BlackWon,
    Draw,
}

impl ChessBoard {
    pub fn get_game_status(&self) -> GameStatus {
        if self.get_allowed_moves(self.turn_color).len() > 0 {
            GameStatus::Ongoing
        } else {
            if self.is_king_in_check(self.turn_color) {
                match self.turn_color {
                    Color::White => GameStatus::BlackWon,
                    Color::Black => GameStatus::WhiteWon
                }
            } else {
                GameStatus::Draw
            }
        }
    }
}

impl ChessBoard {
    pub fn iter_coordinates<'a>(&'a self) -> impl Iterator<Item = (Coordinate, Option<ColorPiece>)> + 'a {
        (0..64).map(move |index| (Coordinate::from_indexer(index), *self.board.get(index).unwrap()))
    }
}
