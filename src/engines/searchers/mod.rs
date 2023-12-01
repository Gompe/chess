pub mod min_max_search;
pub mod alpha_beta_search;
pub mod iterative_deepening;
pub mod repetition_aware_searcher;
pub mod deep_search;

pub use min_max_search::MinMaxSearcher;
pub use alpha_beta_search::AlphaBetaSearcher;
pub use iterative_deepening::IterativeDeepening;
pub use repetition_aware_searcher::RepetitionAwareSearcher;
pub use deep_search::DeepSearch;