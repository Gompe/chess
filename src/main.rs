mod chess_server;
mod engines;

use std::time::Instant;

use crate::engines::engine_traits::Evaluator;
use crate::engines::evaluators::{ClampEvaluator, RolloutEvaluator, StochasticRollout};
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


use chess_server::game::GameManager;
use ordered_float::OrderedFloat;
use crate::chess_server::chess_types::Color;



fn main() {

    let eval_policy_white = CacheEvaluator::new(
        ClampEvaluator::new(
            CaptureEvaluator::new(
                LinearEvaluator::new(
                    MaterialEvaluator::new(),
                    PressureEvaluator::new(),
                    [1.0, 0.05],
                ),
            ),
            OrderedFloat(6.)
        )
    );

    let eval_white = ClampEvaluator::new(
        CaptureEvaluator::new(
            LinearEvaluator::new(
                MaterialEvaluator::new(),
                PressureEvaluator::new(),
                [1.0, 0.05],
            ),
        ),
        OrderedFloat(3.00)
    );

    // let eval_white = CacheEvaluator::new(
    //     ClampEvaluator::new(
    //         CaptureEvaluator::new(
    //             LinearEvaluator::new(
    //                 MaterialEvaluator::new(),
    //                 PressureEvaluator::new(),
    //                 [1.0, 0.05],
    //             ),
    //         ),
    //         OrderedFloat(3.00)
    //     )
    // );


    let eval_white = StochasticRollout::new(
        SoftmaxPolicy::new(eval_white.clone(), 0.1),
        eval_white.clone(),
        20,
        5
    );

    let cloned_eval = eval_white.clone();

    let eval_black = CacheEvaluator::new(
        // ThresholdEvaluator::new(
            CaptureEvaluator::new(
                LinearEvaluator::new(
                    MaterialEvaluator::new(),
                    PressureEvaluator::new(),
                    [1.0, 0.05],
                )
            ),
        // OrderedFloat(5.00)
        // )
    );


    let player_white = SearcherEngine::new(
        eval_white, 
        MonteCarloTreeSearch::new(
            SoftmaxPolicy::new(eval_policy_white, 2.),
            40,
            300, 
            10.
        )
    );


    let player_black = SearcherEngine::new(
        eval_black, 
        DeepSearch::new(5)
    );

    // let player_black = RandomEngine::new();


    let mut game_manager = GameManager::new(
        &player_white, 
        &player_black,
    );


    let mut total_duration_white: u128 = 0;
    let mut total_duration_black: u128 = 0;

    let max_iter = 200;
    let mut num_iter = 1;


    let eval_callbacks: Vec<Box<dyn Evaluator>> = vec![
        Box::new(DynamicEvaluator::new()),
        Box::new(KingSafetyEvaluator::new()),
        Box::new(MaterialEvaluator::new()),
        Box::new(PositionalEvaluator::new()),
        Box::new(PressureEvaluator::new()),
        Box::new(cloned_eval)
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
