use std::time::Duration;

use crate::engines::engine_traits::*;
use crate::engines::evaluators::*;

use crate::engines::timed_searchers::iterative_deepening::IterativeDeepening;

use super::Pokemon;

pub fn tepig() -> Pokemon {

    let evaluator = CaptureEvaluator::new(PestoEvaluator::new());

    let engine = SearcherEngine::new(
        evaluator,
        TimedSearcherWrapper::new(
            Box::new(IterativeDeepening::new()),
            Duration::from_secs(1)
        )
    );

    Pokemon::new(Box::new(engine))
}
