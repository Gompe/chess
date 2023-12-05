use crate::chess_server::chess_types::{Color, ChessBoard};
use crate::engines::engine_traits::*;

use ordered_float::OrderedFloat;

#[derive(Clone)]
pub struct StructureEvaluator;

impl StructureEvaluator {
    pub fn new() -> StructureEvaluator {
        StructureEvaluator {}
    }
}

unsafe impl Send for StructureEvaluator {}
unsafe impl Sync for StructureEvaluator {}

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
                let sign = match content.get_color() {
                    Color::White => 1.,
                    Color::Black => -1.,
                };

                for coord in chess_board.squares_attacked_by_piece(&coordinate) {
                    if chess_board.contains_piece_of_color(&coord, content.get_color()) {
                        eval += sign;
                    }
                }
            }
        }

        OrderedFloat(eval)
    }
}