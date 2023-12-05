use std::cmp::{max, min};

use crate::chess_server::chess_types::{Color, Piece, ChessBoard, color};
use crate::engines::engine_traits::*;

use ordered_float::OrderedFloat;

const VALUE_PAWN : f64 = 1.;
const VALUE_BISHOP : f64 = 3.1;
const VALUE_KNIGHT : f64 = 3.;
const VALUE_ROOK : f64 = 5.;
const VALUE_QUEEN : f64 = 9.;

#[derive(Clone)]
pub struct CaptureEvaluator<E: Evaluator> {
    evaluator: E
}

unsafe impl<E: Evaluator> Send for CaptureEvaluator<E> 
where E: Send {}

unsafe impl<E: Evaluator> Sync for CaptureEvaluator<E> 
where E: Sync {}

impl<E: Evaluator> CaptureEvaluator<E> {
    pub fn new(evaluator: E) -> CaptureEvaluator<E> {
        CaptureEvaluator { evaluator }
    }
}

impl<E: Evaluator> Evaluator for CaptureEvaluator<E> {

    fn get_name(&self) -> String {
        format!("CaptureEvaluator({})", self.evaluator.get_name())
    }
    
    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        
        let mut eval = self.evaluator.evaluate(chess_board);
        let mut adjusted_eval = eval;

        let mut pressure = [0; 64];
        let mut content_value = [0.; 64];

        // Looks for hanging pieces on one side only
        for (coordinate, content) in chess_board.iter_coordinates() {
            
            if let Some(content) = content {
                let get_piece_value = |piece: Piece| {
                    match piece {
                        Piece::Pawn => VALUE_PAWN,
                        Piece::Bishop => VALUE_BISHOP,
                        Piece::Knight => VALUE_KNIGHT,
                        Piece::Rook => VALUE_ROOK,
                        Piece::Queen => VALUE_QUEEN,
                        _ => 0.,
                    }
                };

                let piece_value = get_piece_value(content.get_piece());

                match content.get_color() {
                    Color::White => content_value[coordinate.get_index() as usize] = piece_value,
                    Color::Black => content_value[coordinate.get_index() as usize] = -piece_value,
                };
                
                for coord in chess_board.squares_attacked_by_piece(&coordinate) {
                    // Does not verify pins for example
                    match content.get_color() {
                        Color::White => pressure[coord.get_index() as usize] += 1,
                        Color::Black => pressure[coord.get_index() as usize] -= 1,
                    };


                    // Check if we can capture a piece of larger value
                    if content.get_color() == chess_board.get_turn_color() {
                        if let Some(color_piece) = chess_board.get_square_content(&coord) {
                            if color_piece.get_color() != content.get_color() {
                                // We can capture the color piece!
                                let this_piece_value = get_piece_value(color_piece.get_piece());

                                match content.get_color() {
                                    Color::White => adjusted_eval = max(eval, eval + this_piece_value - piece_value),
                                    Color::Black => adjusted_eval = min(eval, eval - this_piece_value + piece_value)
                                };
                            }
                        }
                    }
                }
            }
        }

        // We have to check if the player who played has hanging pieces.
        // If so, we assume the other player will take the hanging piece and we
        // evaluate from there

        // Naive check for now...
        match chess_board.get_turn_color() {
            Color::White => {
                for index in 0..64 {
                    if pressure[index] > 0 && content_value[index] < 0. {
                        adjusted_eval = max(adjusted_eval, eval - content_value[index]);
                    }
                }
            },
            Color::Black => {
                for index in 0..64 {
                    if pressure[index] < 0 && content_value[index] > 0. {
                        adjusted_eval = min(adjusted_eval, eval - content_value[index]);
                    }
                }
            }
        };

        adjusted_eval
    }
}