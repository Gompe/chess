use ordered_float::OrderedFloat;

use crate::engines::engine_traits::*;
use crate::engines::evaluators::*;
use crate::engines::policies::*;
use crate::engines::searchers::*;

use super::Pokemon;

pub fn corpish() -> Pokemon {
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
    let engine = SearcherEngine::new(
        rollout_eval,
        MonteCarloTreeSearch::new(
            SoftmaxPolicy::new(policy_evaluation, 0.1),
            20,
            100, // 1000
            2.,
        ),
    );

    Pokemon::new(Box::new(engine))
}
