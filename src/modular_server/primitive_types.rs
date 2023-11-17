use std::vec;
use std::fmt;

use std::slice::Iter;
use crate::modular_server::chess_traits::*;

pub const BLACK_PAWN : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::Black, piece : PrimitivePiece::Pawn }; 
pub const BLACK_BISHOP : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::Black, piece : PrimitivePiece::Bishop }; 
pub const BLACK_KNIGHT : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::Black, piece : PrimitivePiece::Knight }; 
pub const BLACK_ROOK : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::Black, piece : PrimitivePiece::Rook }; 
pub const BLACK_QUEEN : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::Black, piece : PrimitivePiece::Queen }; 
pub const BLACK_KING : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::Black, piece : PrimitivePiece::King }; 

pub const WHITE_BISHOP : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::White, piece : PrimitivePiece::Bishop }; 
pub const WHITE_KNIGHT : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::White, piece : PrimitivePiece::Knight }; 
pub const WHITE_ROOK : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::White, piece : PrimitivePiece::Rook }; 
pub const WHITE_PAWN : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::White, piece : PrimitivePiece::Pawn }; 
pub const WHITE_QUEEN : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::White, piece : PrimitivePiece::Queen }; 
pub const WHITE_KING : PrimitiveColorPiece = PrimitiveColorPiece {color : PrimitiveColor::White, piece : PrimitivePiece::King }; 

#[derive(Debug, PartialEq, Eq)]
pub enum ChessStatus {
    Ongoing,
    WhiteWon,
    BlackWon,
    Draw,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitiveColor {
    White,
    Black
}

impl ChessColor for PrimitiveColor {
    fn to_primitive(&self) -> PrimitiveColor {
        *self
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PrimitivePiece {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen, 
    King
}

impl ChessPiece for PrimitivePiece {
    fn to_primitive(&self) -> PrimitivePiece {
        *self
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PrimitiveColorPiece {
    color : PrimitiveColor,
    piece : PrimitivePiece
}

impl ChessColorPiece<PrimitiveColor, PrimitivePiece> for PrimitiveColorPiece {

    fn to_primitive(&self) -> PrimitiveColorPiece {
        *self
    }

    fn get_color(&self) -> PrimitiveColor {
        self.color
    }

    fn get_piece(&self) -> PrimitivePiece {
        self.piece
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PrimitiveSquare {
    row: i32,
    col: i32
}

impl ChessSquare for PrimitiveSquare {

    fn to_primitive(&self) -> PrimitiveSquare {
        *self
    }

    unsafe fn from_coordinates_unchecked(row: i8, col: i8) -> Self {
        PrimitiveSquare { row: row as i32, col: col as i32 }
    }
    unsafe fn from_index_unchecked(index: i8) -> Self {
        Self::from_coordinates_unchecked(index / 8, index % 8)
    }

    fn get_coordinates(&self) -> (u8, u8) {
        (self.row as u8, self.col as u8)
    }
    fn get_index(&self) -> u8 {
        self.row as u8 * 8 + self.col as u8
    }

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct PrimitiveMove {
    move_impl: MoveImpl
}

impl ChessMove<PrimitiveSquare, PrimitivePiece> for PrimitiveMove {

    fn to_primitive(&self) -> PrimitiveMove {
        *self
    }

    fn get_current_square(&self) -> PrimitiveSquare {
        self.move_impl.get_current_square()
    }

    fn get_next_square(&self) -> PrimitiveSquare {
        self.move_impl.get_next_square()
    }

    // Special cases
    fn get_is_promotion(&self) -> bool {
        match self.move_impl {
            MoveImpl::NormalMove(_) => false,
            MoveImpl::PromotionMove(_, _) => true
        }
    }
    
    fn get_promotion_piece(&self) -> Option<PrimitivePiece> {
        match self.move_impl {
            MoveImpl::NormalMove(_) => None,
            MoveImpl::PromotionMove(_, piece) => Some(piece)
        }
    }

    fn get_is_castle(&self) -> bool {
        false
    }

    fn get_is_enpassant(&self) -> bool {
        false
    }
}

#[derive(Clone, Copy)]
pub struct PrimitiveChessBoard {
    board : [Option<PrimitiveColorPiece>; 64],
    turn_color : PrimitiveColor
}

impl ChessBoard<
        PrimitiveMove, 
        PrimitiveSquare,
        PrimitiveColorPiece,
        PrimitiveColor,
        PrimitivePiece,
> for PrimitiveChessBoard {

    fn to_primitive(&self) -> PrimitiveChessBoard {
        *self
    }

    fn get_square_content(&self, square: &PrimitiveSquare) -> Option<PrimitiveColorPiece> {
        *self.board.get(square.get_index() as usize).unwrap()
    }

    fn set_square_content(&mut self, square: &PrimitiveSquare, maybe_color_piece: &Option<PrimitiveColorPiece>) {
        *self.board.get_mut(square.get_index() as usize).unwrap() = *maybe_color_piece;
    }

    fn get_turn_color(&self) -> PrimitiveColor {
        self.turn_color
    }
    
    fn get_game_status(&self) -> ChessStatus {
        if self.get_allowed_moves(self.turn_color).len() > 0 {
            ChessStatus::Ongoing
        } else {
            if self.is_king_in_check(self.turn_color) {
                match self.turn_color {
                    PrimitiveColor::White => ChessStatus::BlackWon,
                    PrimitiveColor::Black => ChessStatus::WhiteWon
                }
            } else {
                ChessStatus::Draw
            }
        }
    }

    fn next_state(&self, mv: &PrimitiveMove) -> Self {

        let mut next_board = self.clone();

        next_board.turn_color = match self.turn_color {
            PrimitiveColor::Black => PrimitiveColor::White,
            PrimitiveColor::White => PrimitiveColor::Black
        };

        let next_piece = match mv.move_impl {
            MoveImpl::NormalMove(_) => self.get_square_content(&mv.get_current_square()),
            MoveImpl::PromotionMove(_, piece) => Some(PrimitiveColorPiece { color: self.turn_color, piece: piece })
        };
        
        next_board.set_square_content(&mv.get_current_square(), &None);
        next_board.set_square_content(&mv.get_next_square(), &next_piece);
        
        next_board
        
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MoveImpl {
    PromotionMove(MoveCoordinates, PrimitivePiece),
    NormalMove(MoveCoordinates)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MoveCoordinates {
    current_square: PrimitiveSquare,
    next_square: PrimitiveSquare,
}


impl PrimitiveSquare {
    pub fn from_chess_notation(coordinate: [char; 2]) -> Option<PrimitiveSquare> {
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

        Some(PrimitiveSquare { row, col })
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
            _ => unreachable!("Panic => PrimitiveSquare {}, {}", self.row, self.col)
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
            _ => unreachable!("Panic => PrimitiveSquare {}, {}", self.row, self.col)
        };
    
        char_col.to_string() + &char_row.to_string()    
    }
}

impl MoveCoordinates {
    pub fn from_chess_notation(current_square: [char; 2], next_square: [char; 2]) -> Option<MoveCoordinates> {

        let current_square = match PrimitiveSquare::from_chess_notation(current_square) {
            Some(coordinate) => coordinate,
            None => return None
        };

        let next_square = match PrimitiveSquare::from_chess_notation(next_square) {
            Some(coordinate) => coordinate,
            None => return None
        };

        Some( MoveCoordinates { current_square, next_square } )
    }
}

impl MoveImpl {
    pub fn get_current_square(&self) -> PrimitiveSquare {
        match *self {
            MoveImpl::PromotionMove(move_coords, _) => move_coords.current_square,
            MoveImpl::NormalMove(move_coords) => move_coords.current_square
        }
    }

    pub fn get_next_square(&self) -> PrimitiveSquare {
        match *self {
            MoveImpl::PromotionMove(move_coords, _) => move_coords.next_square,
            MoveImpl::NormalMove(move_coords) => move_coords.next_square
        }
    }
}

impl fmt::Display for MoveImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MoveImpl::NormalMove(move_coordinates) => {
                write!(f, "MoveImpl: {}->{}", 
                    move_coordinates.current_square.to_str(), 
                    move_coordinates.next_square.to_str()
                )
            }, 
            MoveImpl::PromotionMove(move_coordinates, piece) => {
                write!(f, "MoveImpl: {}->{}, ={}", 
                    move_coordinates.current_square.to_str(), 
                    move_coordinates.next_square.to_str(),
                    piece.to_str()
                )
            }
        }
    }
}

impl PrimitiveColor {
    pub fn to_str(&self) -> String {
        match *self {
            Self::White => String::from("W"),
            Self::Black => String::from("B")
        }
    }
}

impl PrimitivePiece {

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

impl PrimitiveColorPiece {
    pub fn to_str(maybe_color_piece : &Option<PrimitiveColorPiece>) -> String {
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

impl PrimitiveChessBoard {
    pub fn starting_position() -> PrimitiveChessBoard {
        let board : [Option<PrimitiveColorPiece>; 64] = [
            Some(BLACK_ROOK), Some(BLACK_KNIGHT), Some(BLACK_BISHOP), Some(BLACK_QUEEN), Some(BLACK_KING), Some(BLACK_BISHOP), Some(BLACK_KNIGHT), Some(BLACK_ROOK),
            Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN), Some(BLACK_PAWN),
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            None, None, None, None, None, None, None, None, 
            Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN), Some(WHITE_PAWN),
            Some(WHITE_ROOK), Some(WHITE_KNIGHT), Some(WHITE_BISHOP), Some(WHITE_QUEEN), Some(WHITE_KING), Some(WHITE_BISHOP), Some(WHITE_KNIGHT), Some(WHITE_ROOK),
        ];

        PrimitiveChessBoard { board, turn_color: PrimitiveColor::White }
    }

    pub fn print_board(self) {

        println!("{}'s turn", match self.turn_color { PrimitiveColor::White => "White", PrimitiveColor::Black => "Black"});

        for _ in 0..8 {
            print!("+---");
        }
        print!("+");

        print!("\n");
        for (num, maybe_color_piece) in self.board.iter().enumerate() {
            print!("| {} ", PrimitiveColorPiece::to_str(maybe_color_piece));
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


impl PrimitiveChessBoard {

    fn free_squares_in_direction(&self, coordinate: &PrimitiveSquare, delta_row: i8, delta_col: i8) -> Vec<PrimitiveSquare> {
        assert!(delta_row != 0 || delta_col != 0);
        let mut output: Vec<PrimitiveSquare> = Vec::new();
        let mut maybe_position = coordinate.add(delta_row, delta_col);

        while let Some(position) = maybe_position {
            output.push(position);
            if self.get_square_content(&position) == None { 
                continue; 
            }
            maybe_position = position.add(delta_row, delta_col);
        }

        output
    } 

    pub fn squares_attacked_by_piece(&self, coordinate: &PrimitiveSquare) -> Vec<PrimitiveSquare> {

        let color_piece = match self.get_square_content(coordinate) {
            Some(color_piece) => color_piece,
            None => return vec![]
        };


        let mut output = vec![];

        match color_piece.piece {
            PrimitivePiece::Rook => {
                output.append(&mut self.free_squares_in_direction(coordinate,  -1,  0 ));
                output.append(&mut self.free_squares_in_direction(coordinate,  1,  0 ));
                output.append(&mut self.free_squares_in_direction(coordinate,  0,  -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate,  0, 1 ));
            },
            PrimitivePiece::Bishop => {
                output.append(&mut self.free_squares_in_direction(coordinate, 1, -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 1, -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, -1,  1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 1, 1 ));
            },
            PrimitivePiece::Queen => {
                output.append(&mut self.free_squares_in_direction(coordinate, -1,  0 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 1, 0 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 0, -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 0, 1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, -1,  -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 1, -1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, -1,  1 ));
                output.append(&mut self.free_squares_in_direction(coordinate, 1, 1 ));
            },
            PrimitivePiece::Knight => {
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
            PrimitivePiece::King => {
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
            PrimitivePiece::Pawn => {
                match color_piece.color {
                    PrimitiveColor::White => {
                        output = vec![
                            coordinate.add(-1, -1),
                            coordinate.add(-1, 1),
                        ].iter().filter_map(|&any| any).collect();
                    },
                    PrimitiveColor::Black => {
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
fn capture_helper(&self, coordinate: &PrimitiveSquare, color: PrimitiveColor) -> bool {
    match self.get_square_content(&coordinate) {
        None => true,
        Some(color_piece) => color_piece.color != color
    }
}

fn get_potential_moves(&self, coordinate: &PrimitiveSquare) -> Vec<MoveImpl> {
    let color_piece = match self.get_square_content(coordinate) {
        Some(color_piece) => color_piece,
        None => return vec![]
    };

    let mut output = vec![];
    // Castle
    // ...

    let make_move_coordinates = |other_coordinate: PrimitiveSquare| -> MoveCoordinates {
        MoveCoordinates { current_square: *coordinate, next_square: other_coordinate }
    };

    // Other Moves
    if color_piece.piece == PrimitivePiece::Pawn {
        let make_promotion_moves = |x : PrimitiveSquare| -> Vec<MoveImpl> {
            vec![
                MoveImpl::PromotionMove(make_move_coordinates(x), PrimitivePiece::Rook),
                MoveImpl::PromotionMove(make_move_coordinates(x), PrimitivePiece::Queen),
                MoveImpl::PromotionMove(make_move_coordinates(x), PrimitivePiece::Bishop),
                MoveImpl::PromotionMove(make_move_coordinates(x), PrimitivePiece::Knight),
            ]
        };

        let mut pawn_moves = vec![];

        if color_piece.color == PrimitiveColor::White {
            let front_square = coordinate.add(-1, 0).unwrap();
            if self.get_square_content(&front_square) == None {
                pawn_moves.push(front_square);
                if coordinate.row == 6 {
                    let front_square = front_square.add(-1, 0).unwrap();
                    if self.get_square_content(&front_square) == None {
                        pawn_moves.push(front_square);
                    }
                }
            }
            
            if let Some(square) = coordinate.add(-1, -1) {
                if let Some(other_color_piece) = self.get_square_content(&square){
                    if other_color_piece.color == PrimitiveColor::Black {
                        pawn_moves.push(square)
                    }
                }
            }
            
            if let Some(square) = coordinate.add(-1, 1) {
                if let Some(other_color_piece) = self.get_square_content(&square){
                    if other_color_piece.color == PrimitiveColor::Black {
                        pawn_moves.push(square)
                    }
                }
            }

        } else {
            let front_square = coordinate.add(1, 0).unwrap();
            if self.get_square_content(&front_square) == None {
                pawn_moves.push(front_square);
                if coordinate.row == 1 {
                    let front_square = front_square.add(1, 0).unwrap();
                    if self.get_square_content(&front_square) == None {
                        pawn_moves.push(front_square);
                    }
                }
            }
            
            if let Some(square) = coordinate.add(1, -1) {
                if let Some(other_color_piece) = self.get_square_content(&square){
                    if other_color_piece.color == PrimitiveColor::White {
                        pawn_moves.push(square)
                    }
                }
            }
            
            if let Some(square) = coordinate.add(1, 1) {
                if let Some(other_color_piece) = self.get_square_content(&square){
                    if other_color_piece.color == PrimitiveColor::White {
                        pawn_moves.push(square)
                    }
                }
            }
        }

        if (color_piece.color == PrimitiveColor::White && coordinate.row == 1) || (color_piece.color == PrimitiveColor::Black && coordinate.row == 6) {
            output = pawn_moves
                        .iter()
                        .flat_map(|&x| make_promotion_moves(x))
                        .collect();
        } else {
            output = pawn_moves
                        .iter()
                        .map(|&x| MoveImpl::NormalMove(make_move_coordinates(x)))
                        .collect()
        }

    } else {
        let moves = self.squares_attacked_by_piece(coordinate);
        output = moves.iter().filter_map(|&coordinate| {
                        if self.capture_helper(&coordinate, color_piece.color) {
                            Some(MoveImpl::NormalMove(make_move_coordinates(coordinate)))
                        } else {
                            None
                        }
                    })
                    .collect()
    }

    output
}

}


impl PrimitiveChessBoard {
    pub fn find_piece(&self, color_piece: PrimitiveColorPiece) -> Option<PrimitiveSquare> {
        for (index, content) in self.board.iter().enumerate() {
            if Some(color_piece) == *content {
                return Some(PrimitiveSquare::from_index(index as i8).unwrap());
            } 
        }
        None
    }

    pub fn find_king(&self, color: PrimitiveColor) -> PrimitiveSquare {
        match color {
            PrimitiveColor::White => self.find_piece(WHITE_KING).unwrap(),
            PrimitiveColor::Black => self.find_piece(BLACK_KING).unwrap(),
        }
    }

    pub fn is_king_in_check(&self, color: PrimitiveColor) -> bool {
        // find king
        let king_coordinate = self.find_king(color);
        self.is_square_attacked_by_color(&king_coordinate, match color {
            PrimitiveColor::Black => PrimitiveColor::White,
            PrimitiveColor::White => PrimitiveColor::Black
        })
    }

    pub fn is_square_attacked_by_color(&self, coordinate: &PrimitiveSquare, color: PrimitiveColor) -> bool {
        for index in 0..64 {
            let this_coordinate = PrimitiveSquare::from_index(index as i8).unwrap();
            if self.contains_piece_of_color(&this_coordinate, color) {
                if self.squares_attacked_by_piece(&this_coordinate).contains(&coordinate) {
                    return true;
                }
            }
        }
        false
    }

    pub fn contains_piece_of_color(&self, coordinate: &PrimitiveSquare, color: PrimitiveColor) -> bool {
        let content = self.get_square_content(coordinate);
        match content {
            Some(color_piece) => {
                color_piece.color == color
            }
            None => false,
        }
    }

    pub fn get_allowed_moves(&self, color: PrimitiveColor) -> Vec<MoveImpl> {
        let mut output = vec![];
        for index in 0..64 {
            let coordinate = PrimitiveSquare::from_index(index as i8).unwrap();
            match self.get_square_content(&coordinate) {
                Some(color_piece) => {
                    if color_piece.color == color {
                        let maybe_moves = self.get_potential_moves(&coordinate);
                        output.append(&mut maybe_moves
                            .iter()
                            .filter_map(|&x| {
                                let next_board = self.next_state(&PrimitiveMove { move_impl: x });
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

    pub fn get_game_status(&self) -> ChessStatus {
        if self.get_allowed_moves(self.turn_color).len() > 0 {
            ChessStatus::Ongoing
        } else {
            if self.is_king_in_check(self.turn_color) {
                match self.turn_color {
                    PrimitiveColor::White => ChessStatus::BlackWon,
                    PrimitiveColor::Black => ChessStatus::WhiteWon
                }
            } else {
                ChessStatus::Draw
            }
        }
    }

    pub fn iter_coordinates<'a>(&'a self) -> impl Iterator<Item = (PrimitiveSquare, Option<PrimitiveColorPiece>)> + 'a {
        (0..64).map(move |index| (PrimitiveSquare::from_index(index as i8).unwrap(), *self.board.get(index).unwrap()))
    }
}
