use crate::chess_server::chess_types::{Color, ChessBoard};
use crate::engines::engine_traits::*;

use ordered_float::OrderedFloat;

#[derive(Clone)]
pub struct DynamicEvaluator;

impl DynamicEvaluator {
    pub fn new() -> DynamicEvaluator {
        DynamicEvaluator {}
    }
}

unsafe impl Send for DynamicEvaluator {}
unsafe impl Sync for DynamicEvaluator {}

impl Evaluator for DynamicEvaluator {

    fn get_name(&self) -> String {
        "DynamicEvaluator".to_string()
    }
    
    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        
        // 1 point of advantage for player who has the move
        let mut eval = if chess_board.get_turn_color() == Color::White {1.0} else {-1.0};

        for (coordinate, content) in chess_board.iter_coordinates() {
            
            if let Some(content) = content {
                let sign = match content.get_color() {
                    Color::White => 1.,
                    Color::Black => -1.,
                };

                for coord in chess_board.squares_attacked_by_piece(&coordinate) {
                    let (row, col) = coord.get_coordinates();
                    let rad = (row as f64 - 3.5).abs() + (col as f64 - 3.5).abs();
                    eval += ((1.0 - rad) / 8.0).exp() * sign;
                }
            }
        }

        OrderedFloat(eval)
    }
}