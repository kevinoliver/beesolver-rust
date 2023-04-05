pub mod config;
pub mod dictionary;
pub mod puzzle;
pub mod solver;

use std::{error::Error, time::Instant};

use crate::{config::Config, dictionary::Dictionary};
use solver::{Metadata, Solution, Solver};

pub fn run(config: &Config) -> Result<(Solution, Metadata), Box<dyn Error>> {
    use puzzle::Puzzle;

    config.validate()?;

    let puzzle = Puzzle::from(config.required_letter(), config.other_letters())?;
    let start = Instant::now();
    let dict = match config.dict() {
        Some(path) => Dictionary::load_path(path)?,
        None => Dictionary::load()?,
    };
    let dict_size = dict.num_words();
    let dict_name = dict.name();
    let loading_dictionary = start.elapsed();
    let solver = Solver::new(dict, puzzle);

    let start = Instant::now();
    let solution = solver.solve();
    let solving = start.elapsed();

    Ok((
        solution,
        Metadata {
            dictionary_size: dict_size,
            dictionary_name: dict_name,
            loading_dictionary,
            solving,
        },
    ))
}
