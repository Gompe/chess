use crate::chess_server::types::{Color, Piece};
use crate::engines::engine_traits::*;

use crate::chess_server::types::ChessBoard;
use ordered_float::OrderedFloat;

const VALUE_PAWN : f64 = 1.;
const VALUE_BISHOP : f64 = 3.;
const VALUE_KNIGHT : f64 = 3.;
const VALUE_ROOK : f64 = 5.;
const VALUE_QUEEN : f64 = 9.;

pub struct MaterialEvaluator {
    value_pawn: f64,
    value_bishop: f64,
    value_knight: f64,
    value_rook: f64,
    value_queen: f64,
}

impl MaterialEvaluator {
    pub fn new() -> MaterialEvaluator {
        MaterialEvaluator { 
            value_pawn: VALUE_PAWN, 
            value_bishop: VALUE_BISHOP, 
            value_knight: VALUE_KNIGHT, 
            value_rook: VALUE_ROOK, 
            value_queen: VALUE_QUEEN 
        }
    }

    // TODO: Create a "ParametrizedEvaluator" trait 
    pub fn from_coef(coef: [f64; 5]) -> MaterialEvaluator {
        MaterialEvaluator { 
            value_pawn: coef[0], 
            value_bishop: coef[1], 
            value_knight: coef[2], 
            value_rook: coef[3], 
            value_queen: coef[4] 
        }
    }
}

impl Evaluator for MaterialEvaluator {

    fn get_name(&self) -> String {
        "MaterialEvaluator".to_string()
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        let mut eval = 0.;

        for (_, content) in chess_board.iter_coordinates() {
            
            if let Some(content) = content {
                let sign = match content.color {
                    Color::White => 1.,
                    Color::Black => -1.,
                };

                let piece_value = match content.piece {
                    Piece::Pawn => self.value_pawn,
                    Piece::Bishop => self.value_bishop,
                    Piece::Knight => self.value_knight,
                    Piece::Rook => self.value_rook,
                    Piece::Queen => self.value_queen,
                    _ => 0.,
                };

                eval += sign * piece_value;
            }
        }

        OrderedFloat(eval)
    }
}