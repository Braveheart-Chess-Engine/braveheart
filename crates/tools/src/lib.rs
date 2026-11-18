//! Developer tools that sit outside the engine binary: the perft suite
//! runner, the performance gate, the PR bench comment, the openings
//! generator, the SPRT, match and rating reports, and the evaluation's scale.
//! Data generation and relabelling moved to braveheart-selfplay, and the
//! trainer's engine check (`nnue-eval`) to braveheart-trainer.
#![warn(missing_docs)]

pub mod gate;
pub mod history;
pub mod matchup;
pub mod openings;
pub mod perft;
pub mod rating;
pub mod report;
pub mod scale;
pub mod speed;
pub mod sprt;
pub mod stats;
