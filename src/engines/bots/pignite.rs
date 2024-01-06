use std::time::Duration;

use crate::engines::engine_traits::*;
use crate::engines::evaluators::*;

use crate::engines::timed_searchers::clunky_searcher::ClunkySearcher;
use crate::engines::timed_searchers::clunky_searcher_v2::ClunkySearcherV2;
use crate::engines::timed_searchers::clunky_searcher_v3::ClunkySearcherV3;
use crate::engines::timed_searchers::clunky_searcher_v4::ClunkySearcherV4;
use crate::engines::timed_searchers::clunky_searcher_v5::ClunkySearcherV5;
use crate::engines::timed_searchers::clunky_searcher_v6::ClunkySearcherV6;

use super::Pokemon;

pub fn pignite() -> Pokemon {

    let evaluator = CaptureEvaluator::new(PestoEvaluator::new());

    let engine = SearcherEngine::new(
        evaluator,
        TimedSearcherWrapper::new(
            Box::new(ClunkySearcherV6::new()),
            Duration::from_secs(1)
        )
    );

    Pokemon::new(Box::new(engine))
}
