use core::time;
use std::time::Instant;

use engines::evaluators::LinearEvaluator;
use engines::evaluators::PositionalEvaluator;
use engines::evaluators::MaterialEvaluator;
use engines::evaluators::CacheEvaluator;

use engines::random_engine::RandomEngine;
use engines::engine_traits::SearcherEngine;
use engines::min_max_search::MinMaxSearcher;
use engines::iterative_deepening::IterativeDeepening;

use chess_server::game::GameManager;

use crate::chess_server::types::Color;
use crate::engines::alpha_beta_search::AlphaBetaSearcher;


mod chess_server;
mod engines;

fn main() {

    // let eval_white = CacheEvaluator::new(
    //     LinearEvaluator::new(
    //         MaterialEvaluator::new(),
    //         PositionalEvaluator::new(),
    //         [1., 0.1]
    //     )
    // );

    let eval_white = CacheEvaluator::new(LinearEvaluator::new(
            MaterialEvaluator::new(),
            PositionalEvaluator::new(),
            [1., 0.1]
    ));

    let eval_black = CacheEvaluator::new(LinearEvaluator::new(
            MaterialEvaluator::new(),
            PositionalEvaluator::new(),
            [1., 0.1]
    ));

    // let eval_white = MaterialEvaluator::new();

    let player_white = SearcherEngine::new(
        eval_white, 
        IterativeDeepening::new(5)
    );


    let player_black = SearcherEngine::new(
        eval_black, 
        MinMaxSearcher::new(3)
    );

    let mut game_manager = GameManager::new(
        &player_white, &player_black
    );


    let mut total_duration_white: u128 = 0;
    let mut total_duration_black: u128= 0;

    let max_iter = 100;
    let mut num_iter = 1;

    while game_manager.is_game_ongoing() && num_iter < max_iter {
        
        game_manager.show();

        let start_timestamp = Instant::now();
        game_manager.make_move();

        let timedelta = Instant::now() - start_timestamp;

        println!("Time: {}", timedelta.as_millis());
        println!("\n");

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
