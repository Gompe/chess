mod chess_server;
mod engines;

use std::time::Instant;

use crate::chess_server::chess_types::{chess_board, ChessStatus};
use crate::engines::engine_traits::Evaluator;
use crate::engines::evaluators::{ClampEvaluator, RolloutEvaluator, StochasticRollout, NegateEvaluator, TrivialEvaluator};
use crate::engines::if_else_engine::IfElseEngine;
use crate::engines::searchers::deep_search;
use chess_server::chess_types::{
    Move, Piece, Square, Color, ChessBoard, WHITE_PAWN, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_ROOK, WHITE_QUEEN, BLACK_PAWN, BLACK_BISHOP, BLACK_KNIGHT, BLACK_ROOK, BLACK_QUEEN, BLACK_KING, piece, square
};
use engines::engine_traits::SearcherEngine;


use engines::evaluators::{
    LinearEvaluator,
    PositionalEvaluator,
    PressureEvaluator,
    MaterialEvaluator,
    CacheEvaluator,
    CaptureEvaluator,
    DynamicEvaluator,
    KingSafetyEvaluator,
    ThresholdEvaluator,
    StructureEvaluator,
};


use engines::searchers::{
    MonteCarloTreeSearch,
    MinMaxSearcher,
    AlphaBetaSearcher,
    IterativeDeepening,
    RepetitionAwareSearcher,
    DeepSearch,
};


use engines::policies::{
    SoftmaxPolicy
};

use engines::random_engine::RandomEngine;


use chess_server::game::{GameManager, Player};
use ordered_float::OrderedFloat;

use std::io::{stdin, stdout, Write};


fn parse_move(move_str: &str) -> Move {

    let move_str: Vec<char> = move_str.trim().chars().collect();

    assert!((move_str.len() == 4) || (move_str.len() == 5));

    let parse_col = |ch: char| {
        match ch {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => unreachable!()
        }
    };

    let parse_row = |ch: char| {
        match ch {
            '1' => 7,
            '2' => 6,
            '3' => 5,
            '4' => 4,
            '5' => 3,
            '6' => 2,
            '7' => 1,
            '8' => 0,
            _ => unreachable!()
        }
    };

    let parse_piece = |ch: char| {
        match ch {
            'b' => Piece::Bishop,
            'k' => Piece::Knight,
            'r' => Piece::Rook,
            'q' => Piece::Queen,
            _ => unreachable!()
        }
    };

    let current_square = Square::from_coordinates(
        parse_row(move_str[1]), parse_col(move_str[0])
    ).unwrap();

    let next_square = Square::from_coordinates(
        parse_row(move_str[3]), parse_col(move_str[2])
    ).unwrap();

    let mv = {
        if move_str.len() == 4 {
            Move::new_normal_move(current_square, next_square)
        } else {
            let piece = parse_piece(move_str[4]);
            Move::new_promotion_move(current_square, next_square, piece)
        }
    };

    mv
}


struct IOPlayer;

impl IOPlayer {
    pub fn new() -> IOPlayer {
        IOPlayer{}
    }
}

impl Player for IOPlayer {
    fn select_move(&self, chess_board: &ChessBoard) -> Move {
        
        let allowed_moves = chess_board.get_allowed_moves(chess_board.get_turn_color());

        loop {
            let mut s = String::new();
            stdin().read_line(&mut s).expect("Crashed waiting for move");

            let mv = parse_move(&s);
            
            if allowed_moves.contains(&mv) {
                return mv
            } else {
                println!("Invalid move: {}", mv);
            }
        }
    }
}


use std::fs::File;
use log::{error, info, LevelFilter};

use csv::{Writer, WriterBuilder};

use rand::{thread_rng, Rng, random};
use rand::distributions::Alphanumeric;

use serde::Serialize;

fn random_string(n: usize) -> String {
    let string = thread_rng().sample_iter(&Alphanumeric)
                .take(n)
                .collect();

    String::from_utf8(string).unwrap().to_lowercase()
}


#[derive(Serialize)]
struct Row {
    board_string: String,
    dynamic_eval: f64,
    king_safety_eval: f64,
    material_eval: f64,
    positional_eval: f64,
    pressure_eval: f64,
    capture_eval: f64,
    target_eval: f64
}

fn board_to_string(chess_board: &ChessBoard) -> String {
    let mut string = String::with_capacity(64 * 12);

    let piece_order = [
        WHITE_PAWN,
        WHITE_BISHOP,
        WHITE_KNIGHT,
        WHITE_ROOK,
        WHITE_QUEEN,
        WHITE_ROOK,
        BLACK_PAWN,
        BLACK_BISHOP,
        BLACK_KNIGHT,
        BLACK_ROOK,
        BLACK_QUEEN,
        BLACK_KING
    ];

    for piece in piece_order {
        for square in 0..64 {
            let square = Square::from_index(square).unwrap();
            if chess_board.get_square_content(&square) == Some(piece) {
                string += "1";
            } else {
                string += "0";
            }
        }
    }

    string
}

#[allow(dead_code)]
fn main() {

    
    let mut s = String::new();

    stdin().read_line(&mut s).expect("Crashed first stdin");
    
    let mut file = File::create(s.clone() + ".txt").unwrap();

    let rand_filename = random_string(8);

    let mut wtr = WriterBuilder::new()
                    .from_path(format!("data/{}.csv", rand_filename))
                    .unwrap();




    // Initialize the logger with the file as the output
    simple_logging::log_to_file(s.clone() + ".txt", LevelFilter::Info).unwrap();


    file.write_all(s.as_bytes()).unwrap();


    let eval_policy_white = CacheEvaluator::new(
        ClampEvaluator::new(
            CaptureEvaluator::new(
                LinearEvaluator::new(
                    MaterialEvaluator::new(),
                    PressureEvaluator::new(),
                    [1.0, 0.01],
                ),
            ),
            OrderedFloat(6.)
        )
    );

    let eval_white = ClampEvaluator::new(
        CaptureEvaluator::new(
            LinearEvaluator::new(
                LinearEvaluator::new(
                    MaterialEvaluator::new(),
                    PressureEvaluator::new(),
                    [1.0, 0.01],
                ),
                KingSafetyEvaluator::new(),
                [1.0, 0.05]
            ),
        ),
        OrderedFloat(3.00)
    );

    let eval_white = StochasticRollout::new(
        SoftmaxPolicy::new(eval_white.clone(), 0.25),
        eval_white.clone(),
        6,
        5
    );

    let player_white = SearcherEngine::new(
        eval_white.clone(), 
        MonteCarloTreeSearch::new(
            SoftmaxPolicy::new(eval_policy_white, 0.1),
            20,
            100, // 1000
            2.
        )
    );



    let eval_black = CacheEvaluator::new(
        CaptureEvaluator::new(
            LinearEvaluator::new(
                MaterialEvaluator::new(),
                PressureEvaluator::new(),
                [1.0, 0.01],
            )
        ),
    );



    // let player_black = SearcherEngine::new(
    //     eval_black, 
    //     DeepSearch::new(8)
    // );



    let is_start = |chess_board: &ChessBoard| {
        let mut count = 0;
        for index in 0..64 {
            if let Some(_) = chess_board.get_square_content(&Square::from_index(index).unwrap()) {
                count += 1;
            }
        }

        count >= 24
    };

    let is_game = |chess_board: &ChessBoard| {
        let mut count = 0;
        for index in 0..64 {
            if let Some(_) = chess_board.get_square_content(&Square::from_index(index).unwrap()) {
                count += 1;
            }
        }

        count >= 12
    };

    let player_1 = SearcherEngine::new(
        eval_black.clone(), 
        DeepSearch::new(7)
    );

    let player_2 = SearcherEngine::new(
        eval_black.clone(), 
        DeepSearch::new(8)
    );

    let player_3 = SearcherEngine::new(
        eval_black.clone(), 
        DeepSearch::new(9)
    );

    
    let player_white = IfElseEngine::new(
        player_white,
        player_3.clone(),
        is_game
    );
    
    let player_black = player_white.clone();

    // let player_black = IfElseEngine::new(
    //     player_1, 
    //     IfElseEngine::new(
    //         player_2,
    //         player_3,
    //         is_game
    //     ), 
    //     is_start
    // );

    

    // let player_white = player_black.clone();

    let player_io = IOPlayer::new();
    // let engine_player = player_white;

    // let engine_player = RandomEngine::new();

    let engine_color = match s.trim() {
        "white" => Color::White,
        "black" => Color::Black,
        _ => unreachable!()
    };

    let engine_player: &dyn Player = match engine_color {
        Color::White => &player_white,
        Color::Black => &player_black
    };

    let (player_white , player_black) : (&dyn Player, &dyn Player) = {
        match engine_color {
            Color::White => {
                info!("Engine is playing white");
                (engine_player, &player_io)
            },
            Color::Black => {
                info!("Engine is playing black");
                (&player_io, engine_player)
            }
        }
    };


    let mut chess_board = ChessBoard::starting_position();

    info!("{}", chess_board.board_string());

    let m_dynamic_eval = DynamicEvaluator::new();
    let m_king_safety_eval = KingSafetyEvaluator::new();
    let m_material_eval = MaterialEvaluator::new();
    let m_positional_eval = PositionalEvaluator::new();
    let m_pressure_eval = PressureEvaluator::new();
    let m_capture_eval = CaptureEvaluator::new(TrivialEvaluator::new());

    let deep_search_6 = DeepSearch::new(6);

    while chess_board.get_game_status() == ChessStatus::Ongoing {

        let row = Row {
            board_string: board_to_string(&chess_board),
            dynamic_eval: m_dynamic_eval.evaluate(&chess_board).0,
            king_safety_eval: m_king_safety_eval.evaluate(&chess_board).0,
            material_eval: m_material_eval.evaluate(&chess_board).0,
            positional_eval: m_positional_eval.evaluate(&chess_board).0,
            pressure_eval: m_pressure_eval.evaluate(&chess_board).0,
            capture_eval: m_capture_eval.evaluate(&chess_board).0,
            target_eval: deep_search_6.search_ext(&chess_board, &eval_black).0.0
        };

        wtr.serialize(row).unwrap();
        wtr.flush().unwrap();

        if chess_board.get_turn_color() == Color::White {
            let mv = player_white.select_move(&chess_board);
            info!("white move: {}\n", mv);

            if engine_color == Color::White {
                println!("{}", mv);
            }

            chess_board = chess_board.next_state(&mv);


            info!("{}", chess_board.board_string());

        } else {
            let mv = player_black.select_move(&chess_board);
            info!("black move: {}", mv);
            
            if engine_color == Color::Black {
                println!("{}", mv);
            }

            chess_board = chess_board.next_state(&mv);

            info!("{}", chess_board.board_string());
        }
    };

    match chess_board.get_game_status() {
        ChessStatus::WhiteWon => info!("game over: white won"),
        ChessStatus::BlackWon => info!("game over: black won"),
        ChessStatus::Draw => info!("game over: draw"),
        _ => unreachable!()
    }

    return;
}
