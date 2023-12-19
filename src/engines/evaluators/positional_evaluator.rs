use crate::chess_server::chess_types::{ChessBoard, Color, Piece};
use crate::engines::engine_traits::*;

use ordered_float::OrderedFloat;

const VALUE_PAWN: f64 = 2.;
const VALUE_BISHOP: f64 = 1.;
const VALUE_KNIGHT: f64 = 3.;
const VALUE_ROOK: f64 = 1.;
const VALUE_QUEEN: f64 = 1.;
const VALUE_KING: f64 = 0.5;

#[derive(Clone)]
pub struct PositionalEvaluator;

unsafe impl Send for PositionalEvaluator {}
unsafe impl Sync for PositionalEvaluator {}

impl PositionalEvaluator {
    pub fn new() -> PositionalEvaluator {
        PositionalEvaluator {}
    }
}

impl Evaluator for PositionalEvaluator {
    fn get_name(&self) -> String {
        "PositionalEvaluator".to_string()
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        let mut eval = 0.;

        for (coordinate, content) in chess_board.iter_coordinates() {
            if let Some(content) = content {
                let sign = match content.get_color() {
                    Color::White => 1.,
                    Color::Black => -1.,
                };

                let piece_value = match content.get_piece() {
                    Piece::Pawn => VALUE_PAWN,
                    Piece::Bishop => VALUE_BISHOP,
                    Piece::Knight => VALUE_KNIGHT,
                    Piece::Rook => VALUE_ROOK,
                    Piece::Queen => VALUE_QUEEN,
                    Piece::King => VALUE_KING,
                };

                let number_attacked_squares =
                    chess_board.squares_attacked_by_piece(&coordinate).len();
                eval += (number_attacked_squares as f64) * sign * piece_value;

                if content.get_piece() == Piece::Pawn {
                    match content.get_color() {
                        Color::White => {
                            eval += 3.0 * (6 - coordinate.get_coordinates().0) as f64;
                        }
                        Color::Black => {
                            eval += -3.0 * (coordinate.get_coordinates().0 - 1) as f64;
                        }
                    }
                }
            }
        }

        OrderedFloat(eval)
    }
}
