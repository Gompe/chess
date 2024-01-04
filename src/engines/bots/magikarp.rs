use std::time::Duration;

use crate::engines::engine_traits::*;
use crate::engines::evaluators::*;

use crate::engines::timed_searchers::minmax::MinMax;

use super::Pokemon;

pub fn magikarp() -> Pokemon {

    let evaluator = PestoEvaluator::new();

    let engine = SearcherEngine::new(
        evaluator,
        TimedSearcherWrapper::new(
            Box::new(MinMax::new(2)),
            Duration::from_secs(1)
        )
    );

    Pokemon::new(Box::new(engine))
}
