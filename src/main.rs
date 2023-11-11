use engines::linear_evaluator::LinearEvaluator;
use engines::positional_evaluator::PositionalEvaluator;
use engines::random_engine::RandomEngine;
use engines::engine_traits::SearcherEngine;
use engines::material_evaluator::MaterialEvaluator;
use engines::min_max_search::MinMaxSearcher;

use chess_server::game::GameManager;


mod chess_server;
mod engines;

fn main() {

    let eval_white = LinearEvaluator::new(
        MaterialEvaluator::new(),
        PositionalEvaluator::new(),
        [1., 0.1]
    );

    // let eval_white = MaterialEvaluator::new();

    let player_white = SearcherEngine::new(
        eval_white, 
        MinMaxSearcher::new(3)
    );


    let player_black = SearcherEngine::new(
        MaterialEvaluator::new(), 
        MinMaxSearcher::new(2)
    );

    let mut game_manager = GameManager::new(
        &player_white, &player_black
    );

    
    let max_iter = 100;
    let mut num_iter = 1;
    while game_manager.is_game_ongoing() && num_iter < max_iter {
        game_manager.show();
        game_manager.make_move();
        println!("\n");

        num_iter += 1;
    }

    game_manager.show();
}
