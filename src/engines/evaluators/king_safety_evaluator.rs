use crate::chess_server::types::{Color, Coordinate};
use crate::engines::engine_traits::*;

use crate::chess_server::types::ChessBoard;
use ordered_float::OrderedFloat;

const TURN_ADVANTAGE: f64 = 0.3;

pub struct KingSafetyEvaluator;

impl KingSafetyEvaluator {
    pub fn new() -> KingSafetyEvaluator {
        KingSafetyEvaluator {}
    }
}

impl Evaluator for KingSafetyEvaluator {
    
    fn get_name(&self) -> String {
        "KingSafetyEvaluator".to_string()
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        
        // 1 point of advantage for player who has the move
        let mut eval = if chess_board.get_turn_color() == Color::White {TURN_ADVANTAGE} else {-TURN_ADVANTAGE};
        
        let coord_white_king = chess_board.find_king(Color::White);
        let coord_black_king = chess_board.find_king(Color::Black);

        let dist = |c0: &Coordinate, c1: &Coordinate| -> f64 {
            ((c0.row - c1.row).abs() + (c0.col - c1.col).abs()) as f64
        };

        for (coordinate, content) in chess_board.iter_coordinates() {
            
            if let Some(content) = content {
                match content.color {
                    Color::White => {
                        let sign = 1.;
                        for coord in chess_board.squares_attacked_by_piece(&coordinate) {
                            let rad = (coord.row as f64 - 3.5).abs() + (coord.col as f64 - 3.5).abs();
                            let distance_to_king = dist(&coord, &coord_black_king);

                            eval += ((1.0 - rad - 3. * distance_to_king) / 8.0).exp() * sign;
                        }
                    },
                    Color::Black => {
                        let sign = -1.;
                        for coord in chess_board.squares_attacked_by_piece(&coordinate) {
                            let rad = (coord.row as f64 - 3.5).abs() + (coord.col as f64 - 3.5).abs();
                            let distance_to_king = dist(&coord, &coord_white_king);

                            eval += ((1.0 - rad - 3. * distance_to_king) / 8.0).exp() * sign;
                        }
                    }
                }
            }
        }

        OrderedFloat(eval)
    }
}