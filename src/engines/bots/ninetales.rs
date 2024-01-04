use ordered_float::OrderedFloat;

use crate::chess_server::chess_types::*;

use crate::engines::engine_traits::*;
use crate::engines::evaluators::*;
use crate::engines::if_else_engine::IfElseEngine;
use crate::engines::policies::*;
use crate::engines::searchers::*;

use super::Pokemon;

pub fn ninetales() -> Pokemon {
    // Stochastic Rollout Evaluator
    let base_eval = ClampEvaluator::new(
        CaptureEvaluator::new(PestoEvaluator::new()),
        OrderedFloat(3.00),
    );

    let rollout_eval = StochasticRollout::new(
        SoftmaxPolicy::new(base_eval.clone(), 0.25),
        base_eval.clone(),
        6,
        5,
    );

    let policy_evaluation = CacheEvaluator::new(ClampEvaluator::new(
        CaptureEvaluator::new(PestoEvaluator::new()),
        OrderedFloat(6.),
    ));
    let player_white = SearcherEngine::new(
        rollout_eval,
        MonteCarloTreeSearch::new(
            SoftmaxPolicy::new(policy_evaluation, 0.1),
            20,
            100, // 1000
            2.,
        ),
    );

    let eval_endgame = CacheEvaluator::new(CaptureEvaluator::new(PestoEvaluator::new()));

    let endgame_engine = SearcherEngine::new(eval_endgame, DeepSearch::new(9));

    let is_pre_endgame = |chess_board: &ChessBoard| {
        let mut count = 0;
        for index in 0..64 {
            if chess_board
                .get_square_content(&Square::from_index(index).unwrap())
                .is_some()
            {
                count += 1;
            }
        }

        count >= 12
    };

    let engine = IfElseEngine::new(player_white, endgame_engine, is_pre_endgame);

    Pokemon::new(Box::new(engine))
}
