mod chess_server;
mod engines;
mod bitboard;

use std::time::Instant;

use engines::evaluators::LinearEvaluator;
use engines::evaluators::PositionalEvaluator;
use engines::evaluators::MaterialEvaluator;
use engines::evaluators::CacheEvaluator;
use engines::evaluators::CaptureEvaluator;

use engines::searchers::MinMaxSearcher;
use engines::searchers::AlphaBetaSearcher;
use engines::searchers::IterativeDeepening;
use engines::searchers::RepetitionAwareSearcher;

use engines::random_engine::RandomEngine;
use engines::engine_traits::SearcherEngine;


use chess_server::game::GameManager;

use crate::chess_server::types::Color;
use crate::engines::engine_traits::Evaluator;
use crate::engines::evaluators::DynamicEvaluator;
use crate::engines::evaluators::KingSafetyEvaluator;
use crate::engines::evaluators::StructureEvaluator;



fn main() {

    // let eval_white = CacheEvaluator::new(
    //     LinearEvaluator::new(
    //         MaterialEvaluator::new(),
    //         PositionalEvaluator::new(),
    //         [1., 0.1]
    //     )
    // );

    // let eval_white = CacheEvaluator::new(LinearEvaluator::new(
    //         MaterialEvaluator::new(),
    //         PositionalEvaluator::new(),
    //         [1.0, 0.1]
    // ));

    let eval_black = CaptureEvaluator::new(MaterialEvaluator::new());

    let eval_white = CacheEvaluator::new(
        CaptureEvaluator::new(
            LinearEvaluator::new(
                MaterialEvaluator::new(),
                PositionalEvaluator::new(),
                [1.0, 0.01],
            )
        )
    );

    // let eval_white = MaterialEvaluator::new();

    let player_white = SearcherEngine::new(
        eval_white, 
        RepetitionAwareSearcher::new(5)
    );


    let player_black = SearcherEngine::new(
        eval_black, 
        MinMaxSearcher::new(1)
    );

    // let player_black = RandomEngine::new();

    let mut game_manager = GameManager::new(
        &player_white, &player_black
    );


    let mut total_duration_white: u128 = 0;
    let mut total_duration_black: u128= 0;

    let max_iter = 100;
    let mut num_iter = 1;


    let eval_callbacks: Vec<Box<dyn Evaluator>> = vec![
        Box::new(DynamicEvaluator::new()),
        Box::new(KingSafetyEvaluator::new()),
        Box::new(MaterialEvaluator::new()),
        Box::new(PositionalEvaluator::new()),
    ];

    while game_manager.is_game_ongoing() && num_iter < max_iter {
        
        game_manager.show();
        println!("Evaluations:");
        for evaluator in &eval_callbacks {
            println!("Score: {} \t Evaluator: {}", evaluator.evaluate(&game_manager.get_board()), evaluator.get_name());
        }

        let start_timestamp = Instant::now();
        game_manager.make_move();

        let timedelta = Instant::now() - start_timestamp;

        println!("Time: {}", timedelta.as_millis());
        println!("\n");

        let separator: String = std::iter::repeat('-').take(60).collect();
        println!("{}", separator);

        match game_manager.get_turn() {
            Color::White => total_duration_black += timedelta.as_nanos(),
            Color::Black => total_duration_white += timedelta.as_nanos(),
        };

        num_iter += 1;
    }

    game_manager.show();

    println!();
    println!("Total Time White (millis) {}", total_duration_white / 1_000_000);
    println!("Total Time Black (millis) {}", total_duration_black / 1_000_000);

}
