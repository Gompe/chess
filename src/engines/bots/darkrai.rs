use ordered_float::OrderedFloat;

use crate::chess_server::chess_types::*;

use crate::engines::engine_traits::*;
use crate::engines::evaluators::*;
use crate::engines::if_else_engine::IfElseEngine;
use crate::engines::policies::*;
use crate::engines::searchers::*;

use super::Pokemon;

pub fn darkrai() -> Pokemon {

    let evaluator = PestoEvaluator::new();

    let engine = SearcherEngine::new(
        evaluator,
        DeepSearch::new(6)
    );

    Pokemon::new(Box::new(engine))
}
