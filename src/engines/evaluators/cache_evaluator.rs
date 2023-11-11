use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;

use std::hash::{Hash, Hasher};

use crate::engines::engine_traits::*;

use crate::chess_server::types::{ChessBoard, Coordinate, ColorPiece};
use crate::chess_server::types::{
    WHITE_KING,
    WHITE_QUEEN,
    WHITE_ROOK,
    WHITE_BISHOP,
    WHITE_KNIGHT,
    WHITE_PAWN,
    BLACK_KING,
    BLACK_QUEEN,
    BLACK_ROOK,
    BLACK_BISHOP,
    BLACK_KNIGHT,
    BLACK_PAWN,
};

use ordered_float::OrderedFloat;

const ZOBRIST_SEED: u64 = 0x7f4a_8e2d_2a19_a0c3;
const TABLE_SIZE: usize = 12 * 64;

fn init_zobrist(local_state: u64) -> [u64; TABLE_SIZE] {

    let mut local_state: u64 = local_state;
    let mut output = [0; TABLE_SIZE];

    let mut hasher = DefaultHasher::new();
    
    for index in 0..TABLE_SIZE {
        local_state.hash(&mut hasher);
        local_state = hasher.finish();

        output[index] = local_state;
        println!("Hasher state: {}", local_state);
    }

    output
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct BoardHash {
    value: u64,
}

impl BoardHash {
    fn new(chess_board: &ChessBoard, zobrist_table: [u64; TABLE_SIZE]) -> BoardHash {
        let mut state: u64 = 0;

        let make_indexer = |coordinate: Coordinate, content: ColorPiece| {
            let content_indexer = match content {
                WHITE_KING => 0,
                WHITE_QUEEN => 1,
                WHITE_ROOK => 2,
                WHITE_BISHOP => 3,
                WHITE_KNIGHT => 4,
                WHITE_PAWN => 5,
                BLACK_KING => 6,
                BLACK_QUEEN => 7,
                BLACK_ROOK => 8,
                BLACK_BISHOP => 9,
                BLACK_KNIGHT => 10,
                BLACK_PAWN => 11,
            };

            return coordinate.to_indexer() * 12 + content_indexer 
        };

        for (coordinate, content) in chess_board.iter_coordinates() {
            if let Some(content) = content {
                state = state ^ zobrist_table[make_indexer(coordinate, content)];
            }
        }

        BoardHash { value: state }
    }
}


pub struct CacheEvaluator<E: Evaluator> {
    evaluator: E,
    zobrist_table: [u64; TABLE_SIZE],
    
    cache: RefCell< HashMap<BoardHash, OrderedFloat<f64>> >
}

impl<E: Evaluator> CacheEvaluator<E> {
    pub fn new(evaluator: E) -> CacheEvaluator<E> {
        CacheEvaluator { 
            evaluator, 
            zobrist_table : init_zobrist(ZOBRIST_SEED),
            cache: RefCell::new(HashMap::new()) 
        }
    }
}

impl<E: Evaluator> Evaluator for CacheEvaluator<E> {
    fn evaluate(&self, chess_board: &ChessBoard) -> OrderedFloat<f64> {

        let hash_board = BoardHash::new(chess_board, self.zobrist_table);
        let mut cache = self.cache.borrow_mut();

        if let Some((_, &eval)) = cache.get_key_value(&hash_board) {
            return eval;
        }
        
        let eval = self.evaluator.evaluate(chess_board);
        cache.insert(hash_board, eval);

        eval
    }
}

