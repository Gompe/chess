use std::f64::EPSILON;

use crate::chess_server::types::{Color, Piece};
use crate::engines::engine_traits::*;

use crate::chess_server::types::ChessBoard;
use ordered_float::OrderedFloat;

const VALUE_PAWN : f64 = 2.;
const VALUE_BISHOP : f64 = 1.;
const VALUE_KNIGHT : f64 = 3.;
const VALUE_ROOK : f64 = 1.;
const VALUE_QUEEN : f64 = 1.;
const VALUE_KING : f64 = 0.5;

pub struct PositionalEvaluator;

impl PositionalEvaluator {
    pub fn new() -> PositionalEvaluator {
        PositionalEvaluator {}
    }
}

impl Evaluator for PositionalEvaluator {
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        let mut eval = 0.;

        for (coordinate, content) in chess_board.iter_coordinates() {
            
            if let Some(content) = content {
                let sign = match content.color {
                    Color::White => 1.,
                    Color::Black => -1.,
                };

                let piece_value = match content.piece {
                    Piece::Pawn => VALUE_PAWN,
                    Piece::Bishop => VALUE_BISHOP,
                    Piece::Knight => VALUE_KNIGHT,
                    Piece::Rook => VALUE_ROOK,
                    Piece::Queen => VALUE_QUEEN,
                    Piece::King => VALUE_KING,
                };

                let number_attacked_squares = chess_board.squares_attacked_by_piece(&coordinate).len();
                eval += (number_attacked_squares as f64) * sign * piece_value;
            }
        }

        OrderedFloat(eval)
    }
}