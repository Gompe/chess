use std::time::Duration;

use crate::engines::engine_traits::*;
use crate::engines::evaluators::*;

use crate::engines::timed_searchers::alpha_beta::AlphaBeta;

use super::Pokemon;

pub fn weedle() -> Pokemon {

    let evaluator = PestoEvaluator::new();

    let engine = SearcherEngine::new(
        evaluator,
        TimedSearcherWrapper::new(
            Box::new(AlphaBeta::new(4)),
            Duration::from_secs(1)
        )
    );

    Pokemon::new(Box::new(engine))
}
