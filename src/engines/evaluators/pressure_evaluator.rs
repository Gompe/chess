use crate::chess_server::chess_types::{Color, Piece, ChessBoard, Square};
use crate::engines::engine_traits::*;

use ordered_float::OrderedFloat;

const VALUE_PAWN : f64 = 5.;
const VALUE_BISHOP : f64 = 3.0;
const VALUE_KNIGHT : f64 = 3.3;
const VALUE_ROOK : f64 = 2.;
const VALUE_QUEEN : f64 = 1.;
const VALUE_KING : f64 = 0.5;

pub struct PressureEvaluator;

impl PressureEvaluator {
    pub fn new() -> PressureEvaluator {
        PressureEvaluator {}
    }
}

impl Evaluator for PressureEvaluator {
    
    fn get_name(&self) -> String {
        "PressureEvaluator".to_string()
    }

    #[inline(always)]
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {
        let mut eval = 0.;
        let mut heatmap_white = [0.; 64];
        let mut heatmap_black = [0.; 64];

        for (coordinate, content) in chess_board.iter_coordinates() {
            
            if let Some(content) = content {

                let piece_value = match content.get_piece() {
                    Piece::Pawn => VALUE_PAWN,
                    Piece::Bishop => VALUE_BISHOP,
                    Piece::Knight => VALUE_KNIGHT,
                    Piece::Rook => VALUE_ROOK,
                    Piece::Queen => VALUE_QUEEN,
                    Piece::King => VALUE_KING,
                };
                
                let attacked_squares = chess_board.squares_attacked_by_piece(&coordinate);

                match content.get_color() {
                    Color::White => {
                        for square in attacked_squares.iter() {
                            heatmap_white[square.get_index() as usize] += piece_value;
                        }
                        if content.get_piece() == Piece::Pawn {
                            eval += 3.0 * (6 - coordinate.get_coordinates().0) as f64;
                        }
                    },
                    Color::Black => {
                        for square in attacked_squares.iter() {
                            heatmap_black[square.get_index() as usize] += piece_value;
                        }
                        if content.get_piece() == Piece::Pawn {
                            eval += -3.0 * (coordinate.get_coordinates().0 - 1) as f64;
                        }
                    }
                }
            }
        }

        for square_index in 0..64 {
            let square = unsafe { Square::from_index_unchecked(square_index) };
            let content = chess_board.get_square_content(&square);

            let pressure = heatmap_white[square_index as usize] - heatmap_black[square_index as usize];
            match content {
                None => eval += pressure,
                Some(content) => {
                    let piece_value = match content.get_piece() {
                        Piece::Pawn => VALUE_PAWN,
                        Piece::Bishop => VALUE_BISHOP,
                        Piece::Knight => VALUE_KNIGHT,
                        Piece::Rook => VALUE_ROOK,
                        Piece::Queen => VALUE_QUEEN,
                        Piece::King => VALUE_KING,
                    };

                    eval += pressure / piece_value;
                }
            };

        }

        OrderedFloat(eval)
    }
}