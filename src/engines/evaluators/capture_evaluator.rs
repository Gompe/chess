use std::cmp::{max, min};
use std::iter::Zip;

use crate::chess_server::types::{Color, Piece};
use crate::engines::engine_traits::*;

use crate::chess_server::types::ChessBoard;
use ordered_float::OrderedFloat;

const VALUE_PAWN : f64 = 1.;
const VALUE_BISHOP : f64 = 3.1;
const VALUE_KNIGHT : f64 = 3.;
const VALUE_ROOK : f64 = 5.;
const VALUE_QUEEN : f64 = 9.;

pub struct CaptureEvaluator<E: Evaluator> {
    evaluator: E
}

impl<E: Evaluator> CaptureEvaluator<E> {
    pub fn new(evaluator: E) -> CaptureEvaluator<E> {
        CaptureEvaluator { evaluator }
    }
}

impl<E: Evaluator> Evaluator for CaptureEvaluator<E> {

    fn get_name(&self) -> String {
        "CaptureEvaluator".to_string()
    }
    
    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        
        let mut eval = self.evaluator.evaluate(chess_board);

        let mut pressure = [0; 64];
        let mut content_value = [0.; 64];

        // Looks for hanging pieces on one side only
        for (coordinate, content) in chess_board.iter_coordinates() {
            
            if let Some(content) = content {
                let piece_value = match content.get_piece() {
                    Piece::Pawn => VALUE_PAWN,
                    Piece::Bishop => VALUE_BISHOP,
                    Piece::Knight => VALUE_KNIGHT,
                    Piece::Rook => VALUE_ROOK,
                    Piece::Queen => VALUE_QUEEN,
                    _ => 0.,
                };

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
                }
            }
        }

        // We have to check if the player who played has hanging pieces.
        // If so, we assume the other player will take the hanging piece and we
        // evaluate from there

        // Naive check for now...
        match chess_board.get_turn_color() {
            Color::White => {
                let mut adjust_eval = eval;
                for index in 0..64 {
                    if pressure[index] > 0 && content_value[index] < 0. {
                        adjust_eval = max(adjust_eval, adjust_eval - content_value[index]);
                    }
                }

                eval = adjust_eval;
            },
            Color::Black => {
                let mut adjust_eval = eval;
                for index in 0..64 {
                    if pressure[index] < 0 && content_value[index] > 0. {
                        adjust_eval = min(adjust_eval, adjust_eval - content_value[index]);
                    }
                }

                eval = adjust_eval;
            }
        };

        eval
    }
}