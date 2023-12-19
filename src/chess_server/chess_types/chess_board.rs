use smallvec::{smallvec, SmallVec};

use super::color::Color;
use super::color_piece::ColorPiece;
use super::color_piece::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
    WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};
use super::piece::Piece;

use super::chess_move::Move;
use super::chess_status::ChessStatus;
use super::square::Square;

pub type ChessBoard = SmallVecChessBoard;

pub const MOVE_CONTAINER_SIZE: usize = 64;

pub type MoveContainer = SmallVec<[Move; MOVE_CONTAINER_SIZE]>;
pub type SquareContainer = SmallVec<[Square; 16]>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SmallVecChessBoard {
    board: [Option<ColorPiece>; 64],
    turn_color: Color,
}

#[allow(dead_code)]
impl SmallVecChessBoard {
    pub fn get_square_content(&self, square: &Square) -> Option<ColorPiece> {
        *self.board.get(square.get_index() as usize).unwrap()
    }

    fn set_square_content(&mut self, square: &Square, maybe_color_piece: &Option<ColorPiece>) {
        *self.board.get_mut(square.get_index() as usize).unwrap() = *maybe_color_piece;
    }

    pub fn get_turn_color(&self) -> Color {
        self.turn_color
    }

    pub fn get_game_status_from_precomputed(&self, allowed_moves: &MoveContainer) -> ChessStatus {
        if !allowed_moves.is_empty() {
            ChessStatus::Ongoing
        } else if self.is_king_in_check(self.turn_color) {
            match self.turn_color {
                Color::White => ChessStatus::BlackWon,
                Color::Black => ChessStatus::WhiteWon,
            }
        } else {
            ChessStatus::Draw
        }
    }

    pub fn get_game_status(&self) -> ChessStatus {
        if !self.get_allowed_moves(self.turn_color).is_empty() {
            ChessStatus::Ongoing
        } else if self.is_king_in_check(self.turn_color) {
            match self.turn_color {
                Color::White => ChessStatus::BlackWon,
                Color::Black => ChessStatus::WhiteWon,
            }
        } else {
            ChessStatus::Draw
        }
    }

    pub fn next_state(&self, mv: &Move) -> Self {
        let mut next_board = *self;

        next_board.turn_color = match self.turn_color {
            Color::Black => Color::White,
            Color::White => Color::Black,
        };

        let next_piece = match mv.get_promotion_piece() {
            None => self.get_square_content(&mv.get_current_square()),
            Some(piece) => Some(ColorPiece::new(self.turn_color, piece)),
        };

        next_board.set_square_content(&mv.get_current_square(), &None);
        next_board.set_square_content(&mv.get_next_square(), &next_piece);

        next_board
    }

    pub fn starting_position() -> ChessBoard {
        let board: [Option<ColorPiece>; 64] = [
            Some(BLACK_ROOK),
            Some(BLACK_KNIGHT),
            Some(BLACK_BISHOP),
            Some(BLACK_QUEEN),
            Some(BLACK_KING),
            Some(BLACK_BISHOP),
            Some(BLACK_KNIGHT),
            Some(BLACK_ROOK),
            Some(BLACK_PAWN),
            Some(BLACK_PAWN),
            Some(BLACK_PAWN),
            Some(BLACK_PAWN),
            Some(BLACK_PAWN),
            Some(BLACK_PAWN),
            Some(BLACK_PAWN),
            Some(BLACK_PAWN),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(WHITE_PAWN),
            Some(WHITE_PAWN),
            Some(WHITE_PAWN),
            Some(WHITE_PAWN),
            Some(WHITE_PAWN),
            Some(WHITE_PAWN),
            Some(WHITE_PAWN),
            Some(WHITE_PAWN),
            Some(WHITE_ROOK),
            Some(WHITE_KNIGHT),
            Some(WHITE_BISHOP),
            Some(WHITE_QUEEN),
            Some(WHITE_KING),
            Some(WHITE_BISHOP),
            Some(WHITE_KNIGHT),
            Some(WHITE_ROOK),
        ];

        ChessBoard {
            board,
            turn_color: Color::White,
        }
    }

    pub fn board_string(self) -> String {
        let mut s = format!(
            "{}'s turn",
            match self.turn_color {
                Color::White => "White",
                Color::Black => "Black",
            }
        );
        s += "\n";

        for _ in 0..8 {
            s += "+---";
        }
        s += "+";

        s += "\n";
        for (num, maybe_color_piece) in self.board.iter().enumerate() {
            s += &format!("| {} ", ColorPiece::to_str(maybe_color_piece));
            if num % 8 == 7 {
                s += "|\n";
                for _ in 0..8 {
                    s += "+---";
                }

                s += "+\n";
            }
        }

        s
    }

    pub fn print_board(self) {
        println!(
            "{}'s turn",
            match self.turn_color {
                Color::White => "White",
                Color::Black => "Black",
            }
        );

        for _ in 0..8 {
            print!("+---");
        }
        print!("+");

        println!();
        for (num, maybe_color_piece) in self.board.iter().enumerate() {
            print!("| {} ", ColorPiece::to_str(maybe_color_piece));
            if num % 8 == 7 {
                println!("|");
                for _ in 0..8 {
                    print!("+---");
                }

                println!("+");
            }
        }
    }

    fn inplace_free_squares_in_direction(
        &self,
        coordinate: &Square,
        delta_row: i8,
        delta_col: i8,
        output: &mut SquareContainer,
    ) {
        assert!(delta_row != 0 || delta_col != 0);

        let mut maybe_position = coordinate.add(delta_row, delta_col);

        while let Some(position) = maybe_position {
            output.push(position);
            if self.get_square_content(&position).is_some() {
                break;
            }

            maybe_position = position.add(delta_row, delta_col);
        }
    }

    // Optimizing squares_attacked by_piece
    // todo: Change signature to take vector and modify it inplace

    pub fn squares_attacked_by_piece(&self, coordinate: &Square) -> SquareContainer {
        let mut output = SquareContainer::new();
        self.inplace_squares_attacked_by_piece(coordinate, &mut output);
        output
    }

    pub fn inplace_squares_attacked_by_rook(
        &self,
        coordinate: &Square,
        output: &mut SquareContainer,
    ) {
        self.inplace_free_squares_in_direction(coordinate, -1, 0, output);
        self.inplace_free_squares_in_direction(coordinate, 1, 0, output);
        self.inplace_free_squares_in_direction(coordinate, 0, -1, output);
        self.inplace_free_squares_in_direction(coordinate, 0, 1, output)
    }

    pub fn inplace_squares_attacked_by_bishop(
        &self,
        coordinate: &Square,
        output: &mut SquareContainer,
    ) {
        self.inplace_free_squares_in_direction(coordinate, 1, 1, output);
        self.inplace_free_squares_in_direction(coordinate, 1, -1, output);
        self.inplace_free_squares_in_direction(coordinate, -1, 1, output);
        self.inplace_free_squares_in_direction(coordinate, -1, -1, output)
    }

    pub fn inplace_squares_attacked_by_queen(
        &self,
        coordinate: &Square,
        output: &mut SquareContainer,
    ) {
        self.inplace_squares_attacked_by_bishop(coordinate, output);
        self.inplace_squares_attacked_by_rook(coordinate, output)
    }

    pub fn inplace_squares_attacked_by_knight(
        &self,
        coordinate: &Square,
        output: &mut SquareContainer,
    ) {
        let temp: SmallVec<[_; 16]> = smallvec![
            coordinate.add(-2, -1),
            coordinate.add(-2, 1),
            coordinate.add(-1, -2),
            coordinate.add(-1, 2),
            coordinate.add(2, -1),
            coordinate.add(2, 1),
            coordinate.add(1, -2),
            coordinate.add(1, 2),
        ];

        output.append::<[_; 16]>(&mut temp.iter().filter_map(|&any| any).collect())
    }

    pub fn inplace_squares_attacked_by_king(
        &self,
        coordinate: &Square,
        output: &mut SquareContainer,
    ) {
        let temp: SmallVec<[_; 16]> = smallvec![
            coordinate.add(0, -1),
            coordinate.add(0, 1),
            coordinate.add(-1, -1),
            coordinate.add(-1, 0),
            coordinate.add(-1, 1),
            coordinate.add(1, -1),
            coordinate.add(1, 0),
            coordinate.add(1, 1),
        ];

        output.append::<[_; 16]>(&mut temp.iter().filter_map(|&any| any).collect())
    }

    pub fn inplace_squares_attacked_by_piece(
        &self,
        coordinate: &Square,
        output: &mut SquareContainer,
    ) {
        let color_piece = match self.get_square_content(coordinate) {
            Some(color_piece) => color_piece,
            None => return,
        };

        match color_piece.get_piece() {
            Piece::Rook => self.inplace_squares_attacked_by_rook(coordinate, output),
            Piece::Bishop => self.inplace_squares_attacked_by_bishop(coordinate, output),
            Piece::Queen => self.inplace_squares_attacked_by_queen(coordinate, output),
            Piece::Knight => self.inplace_squares_attacked_by_knight(coordinate, output),
            Piece::King => self.inplace_squares_attacked_by_king(coordinate, output),
            Piece::Pawn => match color_piece.get_color() {
                Color::White => {
                    let temp: SmallVec<[_; 2]> =
                        smallvec![coordinate.add(-1, -1), coordinate.add(-1, 1),];

                    output.append::<[_; 2]>(&mut temp.iter().filter_map(|&any| any).collect());
                }
                Color::Black => {
                    let temp: SmallVec<[_; 2]> =
                        smallvec![coordinate.add(1, -1), coordinate.add(1, 1),];

                    output.append::<[_; 2]>(&mut temp.iter().filter_map(|&any| any).collect());
                }
            },
        };
    }

    fn inplace_get_potential_moves(&self, coordinate: &Square, output: &mut MoveContainer) {
        let color_piece = match self.get_square_content(coordinate) {
            Some(color_piece) => color_piece,
            None => return,
        };

        let make_move_coordinates =
            |other_coordinate: Square| -> (Square, Square) { (*coordinate, other_coordinate) };

        // Other Moves
        if color_piece.get_piece() == Piece::Pawn {
            let make_promotion_moves = |x: Square| -> [Move; 4] {
                let (current_square, next_square) = make_move_coordinates(x);
                [
                    Move::new_promotion_move(current_square, next_square, Piece::Bishop),
                    Move::new_promotion_move(current_square, next_square, Piece::Knight),
                    Move::new_promotion_move(current_square, next_square, Piece::Rook),
                    Move::new_promotion_move(current_square, next_square, Piece::Queen),
                ]
            };

            let mut pawn_moves = SmallVec::<[Square; 4]>::new();
            let (row, _) = coordinate.get_coordinates();

            if color_piece.get_color() == Color::White {
                let front_square = coordinate.add(-1, 0).unwrap();
                if self.get_square_content(&front_square).is_none() {
                    pawn_moves.push(front_square);
                    if row == 6 {
                        let front_square = front_square.add(-1, 0).unwrap();
                        if self.get_square_content(&front_square).is_none() {
                            pawn_moves.push(front_square);
                        }
                    }
                }

                if let Some(square) = coordinate.add(-1, -1) {
                    if let Some(other_color_piece) = self.get_square_content(&square) {
                        if other_color_piece.get_color() == Color::Black {
                            pawn_moves.push(square)
                        }
                    }
                }

                if let Some(square) = coordinate.add(-1, 1) {
                    if let Some(other_color_piece) = self.get_square_content(&square) {
                        if other_color_piece.get_color() == Color::Black {
                            pawn_moves.push(square)
                        }
                    }
                }
            } else {
                let front_square = coordinate.add(1, 0).unwrap();
                if self.get_square_content(&front_square).is_none() {
                    pawn_moves.push(front_square);
                    if row == 1 {
                        let front_square = front_square.add(1, 0).unwrap();
                        if self.get_square_content(&front_square).is_none() {
                            pawn_moves.push(front_square);
                        }
                    }
                }

                if let Some(square) = coordinate.add(1, -1) {
                    if let Some(other_color_piece) = self.get_square_content(&square) {
                        if other_color_piece.get_color() == Color::White {
                            pawn_moves.push(square)
                        }
                    }
                }

                if let Some(square) = coordinate.add(1, 1) {
                    if let Some(other_color_piece) = self.get_square_content(&square) {
                        if other_color_piece.get_color() == Color::White {
                            pawn_moves.push(square)
                        }
                    }
                }
            }

            if (color_piece.get_color() == Color::White && row == 1)
                || (color_piece.get_color() == Color::Black && row == 6)
            {
                output.append::<[_; 4]>(
                    &mut pawn_moves
                        .iter()
                        .flat_map(|&x| make_promotion_moves(x))
                        .collect(),
                )
            } else {
                output.append::<[_; 4]>(
                    &mut pawn_moves
                        .iter()
                        .map(|&x| {
                            let (current_square, next_square) = make_move_coordinates(x);
                            Move::new_normal_move(current_square, next_square)
                        })
                        .collect(),
                )
            }
        } else {
            let mut moves = SquareContainer::new();
            self.inplace_squares_attacked_by_piece(coordinate, &mut moves);

            output.append::<[_; 16]>(
                &mut moves
                    .iter()
                    .filter_map(|&coordinate| {
                        if self.capture_helper(&coordinate, color_piece.get_color()) {
                            let (current_square, next_square) = make_move_coordinates(coordinate);
                            Some(Move::new_normal_move(current_square, next_square))
                        } else {
                            None
                        }
                    })
                    .collect(),
            )
        }
    }

    // Returns true if the square could be captured by a piece of the specified
    // color. This ignores checks.
    fn capture_helper(&self, coordinate: &Square, color: Color) -> bool {
        match self.get_square_content(coordinate) {
            None => true,
            Some(color_piece) => color_piece.get_color() != color,
        }
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

        let king_coordinate: Square = self.find_king(color);
        self.is_square_attacked_by_color(
            &king_coordinate,
            match color {
                Color::Black => Color::White,
                Color::White => Color::Black,
            },
        )
    }

    pub fn is_square_attacked_by_color(&self, coordinate: &Square, color: Color) -> bool {
        match color {
            Color::White => {
                if let Some(coord) = coordinate.add(1, -1) {
                    if self.get_square_content(&coord) == Some(WHITE_PAWN) {
                        return true;
                    }
                }
                if let Some(coord) = coordinate.add(1, 1) {
                    if self.get_square_content(&coord) == Some(WHITE_PAWN) {
                        return true;
                    }
                }
            }
            Color::Black => {
                if let Some(coord) = coordinate.add(-1, -1) {
                    if self.get_square_content(&coord) == Some(BLACK_PAWN) {
                        return true;
                    }
                }
                if let Some(coord) = coordinate.add(-1, 1) {
                    if self.get_square_content(&coord) == Some(BLACK_PAWN) {
                        return true;
                    }
                }
            }
        }

        // Now Check for other pieces
        let mut output = SquareContainer::new();

        let check_piece = |output: &SquareContainer, piece| {
            output
                .iter()
                .map(|square| self.get_square_content(square))
                .any(|x| x == Some(ColorPiece::new(color, piece)))
        };

        // Bishop
        self.inplace_squares_attacked_by_bishop(coordinate, &mut output);
        if check_piece(&output, Piece::Bishop) || check_piece(&output, Piece::Queen) {
            return true;
        }

        output.clear();

        // Rook
        self.inplace_squares_attacked_by_rook(coordinate, &mut output);
        if check_piece(&output, Piece::Rook) || check_piece(&output, Piece::Queen) {
            return true;
        }

        output.clear();

        // Knight
        self.inplace_squares_attacked_by_knight(coordinate, &mut output);
        if check_piece(&output, Piece::Knight) {
            return true;
        }

        output.clear();

        // King
        self.inplace_squares_attacked_by_king(coordinate, &mut output);
        if check_piece(&output, Piece::King) {
            return true;
        }

        false
    }

    pub fn contains_piece_of_color(&self, coordinate: &Square, color: Color) -> bool {
        let content = self.get_square_content(coordinate);
        match content {
            Some(color_piece) => color_piece.get_color() == color,
            None => false,
        }
    }

    pub fn get_allowed_moves(&self, color: Color) -> MoveContainer {
        let mut output = MoveContainer::new();
        let mut maybe_moves = MoveContainer::new();

        for index in 0..64 {
            let coordinate = unsafe { Square::from_index_unchecked(index as i8) };

            match self.get_square_content(&coordinate) {
                Some(color_piece) => {
                    if color_piece.get_color() == color {
                        self.inplace_get_potential_moves(&coordinate, &mut maybe_moves);
                    }
                }
                None => (),
            }
        }

        output.append::<[_; 64]>(
            &mut maybe_moves
                .iter()
                .filter_map(|&x| {
                    let next_board = self.next_state(&x);
                    if next_board.is_king_in_check(color) {
                        None
                    } else {
                        Some(x)
                    }
                })
                .collect(),
        );

        output
    }

    pub fn iter_coordinates(&self) -> impl Iterator<Item = (Square, Option<ColorPiece>)> + '_ {
        (0..64).map(move |index| {
            (
                Square::from_index(index as i8).unwrap(),
                *self.board.get(index).unwrap(),
            )
        })
    }
}
