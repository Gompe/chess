use std::vec;
use std::fmt;

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

#[derive(Debug, PartialEq, Eq)]
pub enum ChessStatus {
    Ongoing,
    WhiteWon,
    BlackWon,
    Draw,
}

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


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ColorPiece {
    color : Color,
    piece : Piece
}

impl ColorPiece {

    #[inline(always)]
    pub fn get_color(&self) -> Color {
        self.color
    }

    #[inline(always)]
    pub fn get_piece(&self) -> Piece {
        self.piece
    }

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


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Square {
    board_index: u8
}

impl Square {

    #[inline(always)]
    pub unsafe fn from_coordinates_unchecked(row: i8, col: i8) -> Self {
        let board_index = (row * 8 + col) as u8;
        Square { board_index }
    }

    #[inline(always)]
    pub unsafe fn from_index_unchecked(index: i8) -> Self {
        Square { board_index: index as u8 }
    }

    pub fn from_coordinates(row: i8, col: i8) -> Option<Self> {
        if (row >= 0) && (row < 8) && (col >= 0) && (col < 8) 
        {
            unsafe {
                Some(Self::from_coordinates_unchecked(row, col))
            }
        } else {    
            None
        }
    }

    pub fn from_index(index: i8) -> Option<Self> {
        if (index >= 0) & (index < 64) {
            unsafe {
                Some(Self::from_index_unchecked(index))
            }
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_coordinates(&self) -> (u8, u8) {
        (self.board_index / 8, self.board_index % 8)
    }

    #[inline(always)]
    pub fn get_index(&self) -> u8 {
        self.board_index
    }

    unsafe fn add_unchecked(&self, d_row: i8, d_col: i8) -> Self {
        let (row, col) = self.get_coordinates();
        Self::from_coordinates_unchecked(row as i8 + d_row, col as i8 + d_col)
    }

    pub fn add(&self, d_row: i8, d_col: i8) -> Option<Self> {
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

    pub fn from_chess_notation(coordinate: [char; 2]) -> Option<Square> {
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

        Square::from_coordinates(row, col)
    }

    pub fn to_str(&self) -> String {
        let (row, col) = self.get_coordinates();

        let char_row = match row {
            0 => '8',
            1 => '7',
            2 => '6',
            3 => '5',
            4 => '4',
            5 => '3',
            6 => '2',
            7 => '1',
            _ => unreachable!("Panic => Square {}, {}", row, col)
        };

        let char_col = match col {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => unreachable!("Panic => Square {}, {}", row, col)
        };
    
        char_col.to_string() + &char_row.to_string()    
    }

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    PromotionMove(MoveCoordinates, Piece),
    NormalMove(MoveCoordinates)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MoveCoordinates {
    current_square: Square,
    next_square: Square,
}

impl MoveCoordinates {
    pub fn from_chess_notation(current_square: [char; 2], next_square: [char; 2]) -> Option<MoveCoordinates> {

        let current_square = match Square::from_chess_notation(current_square) {
            Some(coordinate) => coordinate,
            None => return None
        };

        let next_square = match Square::from_chess_notation(next_square) {
            Some(coordinate) => coordinate,
            None => return None
        };

        Some( MoveCoordinates { current_square, next_square } )
    }
}


impl Move {
    pub fn get_current_square(&self) -> Square {
        match *self {
            Move::PromotionMove(move_coords, _) => move_coords.current_square,
            Move::NormalMove(move_coords) => move_coords.current_square
        }
    }

    pub fn get_next_square(&self) -> Square {
        match *self {
            Move::PromotionMove(move_coords, _) => move_coords.next_square,
            Move::NormalMove(move_coords) => move_coords.next_square
        }
    }

    // Special cases
    pub fn get_is_promotion(&self) -> bool {
        match *self {
            Move::NormalMove(_) => false,
            Move::PromotionMove(_, _) => true
        }
    }
    
    pub fn get_promotion_piece(&self) -> Option<Piece> {
        match *self {
            Move::NormalMove(_) => None,
            Move::PromotionMove(_, piece) => Some(piece)
        }
    }

    pub fn get_is_castle(&self) -> bool {
        false
    }

    pub fn get_is_enpassant(&self) -> bool {
        false
    }
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

        let next_piece = match mv {
            Move::NormalMove(_) => self.get_square_content(&mv.get_current_square()),
            Move::PromotionMove(_, piece) => Some(ColorPiece { color: self.turn_color, piece: *piece })
        };
        
        next_board.set_square_content(&mv.get_current_square(), &None);
        next_board.set_square_content(&mv.get_next_square(), &next_piece);
        
        next_board
        
    }
}


impl ChessBoard {
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

        match color_piece.piece {
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

    let make_move_coordinates = |other_coordinate: Square| -> MoveCoordinates {
        MoveCoordinates { current_square: *coordinate, next_square: other_coordinate }
    };

    // Other Moves
    if color_piece.piece == Piece::Pawn {
        let make_promotion_moves = |x : Square| -> Vec<Move> {
            vec![
                Move::PromotionMove(make_move_coordinates(x), Piece::Rook),
                Move::PromotionMove(make_move_coordinates(x), Piece::Queen),
                Move::PromotionMove(make_move_coordinates(x), Piece::Bishop),
                Move::PromotionMove(make_move_coordinates(x), Piece::Knight),
            ]
        };

        let mut pawn_moves = vec![];
        let (row, _) = coordinate.get_coordinates(); 

        if color_piece.color == Color::White {
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
                    if other_color_piece.color == Color::Black {
                        pawn_moves.push(square)
                    }
                }
            }
            
            if let Some(square) = coordinate.add(-1, 1) {
                if let Some(other_color_piece) = self.get_square_content(&square){
                    if other_color_piece.color == Color::Black {
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
                    if other_color_piece.color == Color::White {
                        pawn_moves.push(square)
                    }
                }
            }
            
            if let Some(square) = coordinate.add(1, 1) {
                if let Some(other_color_piece) = self.get_square_content(&square){
                    if other_color_piece.color == Color::White {
                        pawn_moves.push(square)
                    }
                }
            }
        }

        if (color_piece.color == Color::White && row == 1) || (color_piece.color == Color::Black && row == 6) {
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
                        if self.capture_helper(&coordinate, color_piece.get_color()) {
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


impl ChessBoard {
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
                color_piece.color == color
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

    pub fn iter_coordinates<'a>(&'a self) -> impl Iterator<Item = (Square, Option<ColorPiece>)> + 'a {
        (0..64).map(move |index| (Square::from_index(index as i8).unwrap(), *self.board.get(index).unwrap()))
    }
}
