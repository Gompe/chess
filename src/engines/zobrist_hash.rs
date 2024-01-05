use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;

use std::hash::{Hash, Hasher};

use crate::backend::{ChessBoard, Color, ColorPiece};
use crate::backend::{
    BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, WHITE_BISHOP,
    WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK,
};

const ZOBRIST_SEED: u64 = 0x7f4a_8e2d_2a19_a0c3;
const TABLE_SIZE: usize = 1 + 12 * 64;

fn init_zobrist(local_state: u64) -> [u64; TABLE_SIZE] {
    let mut local_state: u64 = local_state;
    let mut output = [0; TABLE_SIZE];

    let mut hasher = DefaultHasher::new();

    for index in 0..TABLE_SIZE {
        local_state.hash(&mut hasher);
        local_state = hasher.finish();

        output[index] = local_state;
    }

    output
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct BoardHash {
    value: u64,
}

impl BoardHash {
    fn new(chess_board: &ChessBoard, zobrist_table: [u64; TABLE_SIZE]) -> BoardHash {
        let mut state: u64 = 0;

        let make_indexer = |index: u8, content: ColorPiece| {
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
                _ => unreachable!(),
            };

            1 + (index as usize) * 12 + content_indexer
        };

        if chess_board.get_turn_color() == Color::White {
            state ^= zobrist_table[0];
        }

        for (coordinate, content) in chess_board.iter_coordinates() {
            if let Some(content) = content {
                state ^= zobrist_table[make_indexer(coordinate.get_index(), content)];
            }
        }

        BoardHash { value: state }
    }
}

#[derive(Clone)]
pub struct ZobristHashMap<V> {
    zobrist_table: [u64; TABLE_SIZE],
    cache: HashMap<BoardHash, V>,
}

impl<V> ZobristHashMap<V> {
    pub fn new() -> ZobristHashMap<V> {
        ZobristHashMap {
            zobrist_table: init_zobrist(ZOBRIST_SEED),
            cache: HashMap::new(),
        }
    }

    pub fn get_key_value(&self, key: &ChessBoard) -> Option<&V> {
        let hash_board = BoardHash::new(key, self.zobrist_table);
        match self.cache.get_key_value(&hash_board) {
            Some((_, v)) => Some(v),
            None => None,
        }
    }

    pub fn get_mut(&mut self, key: &ChessBoard) -> Option<&mut V> {
        let hash_board = BoardHash::new(key, self.zobrist_table);
        self.cache.get_mut(&hash_board)
    }

    pub fn insert(&mut self, key: &ChessBoard, value: V) {
        let hash_board = BoardHash::new(key, self.zobrist_table);
        self.cache.insert(hash_board, value);
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn clear(&mut self) {
        self.cache.clear()
    }
}
