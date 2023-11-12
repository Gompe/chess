use crate::chess_server::types::Color;
use crate::engines::engine_traits::*;

use crate::chess_server::types::ChessBoard;
use ordered_float::OrderedFloat;

pub struct StructureEvaluator;

impl StructureEvaluator {
    pub fn new() -> StructureEvaluator {
        StructureEvaluator {}
    }
}

impl Evaluator for StructureEvaluator {

    fn get_name(&self) -> String {
        "StructureEvaluator".to_string()
    }
    
    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        
        // 1 point of advantage for player who has the move
        let mut eval = 0.;

        for (coordinate, content) in chess_board.iter_coordinates() {
            
            if let Some(content) = content {
                let sign = match content.color {
                    Color::White => 1.,
                    Color::Black => -1.,
                };

                for coord in chess_board.squares_attacked_by_piece(&coordinate) {
                    if chess_board.contains_piece_of_color(&coord, content.color) {
                        eval += sign;
                    }
                }
            }
        }

        OrderedFloat(eval)
    }
}