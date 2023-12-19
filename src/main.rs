mod chess_server;
mod engines;



use crate::chess_server::chess_types::{ChessStatus};
use crate::chess_server::io::io_player::IOPlayer;
use crate::chess_server::io::utils::board_to_string;
use crate::engines::bots::pikachu;
use crate::engines::engine_traits::Evaluator;
use crate::engines::evaluators::{TrivialEvaluator};


use chess_server::chess_types::{
    Color, ChessBoard
};


use engines::evaluators::{
    LinearEvaluator,
    PositionalEvaluator,
    PressureEvaluator,
    MaterialEvaluator,
    CacheEvaluator,
    CaptureEvaluator,
    DynamicEvaluator,
    KingSafetyEvaluator,
};


use engines::searchers::{
    DeepSearch,
};






use chess_server::game::{Player};


use std::io::{stdin, Write};
use csv::{WriterBuilder};
use std::fs::File;

use log::{info, LevelFilter};


#[allow(dead_code)]
fn main() {

    
    let mut s = String::new();

    stdin().read_line(&mut s).expect("Crashed first stdin");
    
    let mut file = File::create(s.clone() + ".txt").unwrap();

    let rand_filename = chess_server::io::utils::random_string(8);

    let mut wtr = WriterBuilder::new()
                    .from_path(format!("data/{}.csv", rand_filename))
                    .unwrap();




    // Initialize the logger with the file as the output
    simple_logging::log_to_file(s.clone() + ".txt", LevelFilter::Info).unwrap();


    file.write_all(s.as_bytes()).unwrap();

    let player_white = pikachu();
    let player_black = pikachu();

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
    let target_eval = CacheEvaluator::new(
        CaptureEvaluator::new(
            LinearEvaluator::new(
                MaterialEvaluator::new(),
                PressureEvaluator::new(),
                [1.0, 0.01],
            )
        ),
    );

    while chess_board.get_game_status() == ChessStatus::Ongoing {

        let row = chess_server::io::utils::Row {
            board_string: board_to_string(&chess_board),
            dynamic_eval: m_dynamic_eval.evaluate(&chess_board).0,
            king_safety_eval: m_king_safety_eval.evaluate(&chess_board).0,
            material_eval: m_material_eval.evaluate(&chess_board).0,
            positional_eval: m_positional_eval.evaluate(&chess_board).0,
            pressure_eval: m_pressure_eval.evaluate(&chess_board).0,
            capture_eval: m_capture_eval.evaluate(&chess_board).0,
            target_eval: deep_search_6.search_ext(&chess_board, &target_eval).0.0
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
}
