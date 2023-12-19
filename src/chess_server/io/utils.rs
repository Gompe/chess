use crate::chess_server::chess_types::*;

use std::io::{stdin, stdout, Write};

use std::fs::File;
use log::{error, info, LevelFilter};

use csv::{Writer, WriterBuilder};

use rand::{thread_rng, Rng, random};
use rand::distributions::Alphanumeric;

use serde::Serialize;

pub fn random_string(n: usize) -> String {
    let string = thread_rng().sample_iter(&Alphanumeric)
                .take(n)
                .collect();

    String::from_utf8(string).unwrap().to_lowercase()
}

#[derive(Serialize)]
pub struct Row {
    pub board_string: String,
    pub dynamic_eval: f64,
    pub king_safety_eval: f64,
    pub material_eval: f64,
    pub positional_eval: f64,
    pub pressure_eval: f64,
    pub capture_eval: f64,
    pub target_eval: f64
}

pub fn board_to_string(chess_board: &ChessBoard) -> String {
    let mut string = String::with_capacity(64 * 12);

    let piece_order = [
        WHITE_PAWN,
        WHITE_BISHOP,
        WHITE_KNIGHT,
        WHITE_ROOK,
        WHITE_QUEEN,
        WHITE_ROOK,
        BLACK_PAWN,
        BLACK_BISHOP,
        BLACK_KNIGHT,
        BLACK_ROOK,
        BLACK_QUEEN,
        BLACK_KING
    ];

    for piece in piece_order {
        for square in 0..64 {
            let square = Square::from_index(square).unwrap();
            if chess_board.get_square_content(&square) == Some(piece) {
                string += "1";
            } else {
                string += "0";
            }
        }
    }

    string
}

pub fn parse_move(move_str: &str) -> Move {

    let move_str: Vec<char> = move_str.trim().chars().collect();

    assert!((move_str.len() == 4) || (move_str.len() == 5));

    let parse_col = |ch: char| {
        match ch {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => unreachable!()
        }
    };

    let parse_row = |ch: char| {
        match ch {
            '1' => 7,
            '2' => 6,
            '3' => 5,
            '4' => 4,
            '5' => 3,
            '6' => 2,
            '7' => 1,
            '8' => 0,
            _ => unreachable!()
        }
    };

    let parse_piece = |ch: char| {
        match ch {
            'b' => Piece::Bishop,
            'k' => Piece::Knight,
            'r' => Piece::Rook,
            'q' => Piece::Queen,
            _ => unreachable!()
        }
    };

    let current_square = Square::from_coordinates(
        parse_row(move_str[1]), parse_col(move_str[0])
    ).unwrap();

    let next_square = Square::from_coordinates(
        parse_row(move_str[3]), parse_col(move_str[2])
    ).unwrap();

    let mv = {
        if move_str.len() == 4 {
            Move::new_normal_move(current_square, next_square)
        } else {
            let piece = parse_piece(move_str[4]);
            Move::new_promotion_move(current_square, next_square, piece)
        }
    };

    mv
}
