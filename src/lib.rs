pub mod puzzle;
pub mod dictionary;
pub mod solver;

use std::{error::Error, cmp::Ordering,};

use crate::{dictionary::Dictionary, solver::Solver};

#[derive(Debug)]
pub enum WordResult {
    Invalid,
    Valid(String),
    Pangram(String),
}

use WordResult::*;

impl WordResult {
    pub fn word(&self) -> Option<String> {
        match self {
            Invalid => None,
            Valid(w) => Some(w.to_string()),
            Pangram(w) => Some(w.to_string()),
        }
    }

    pub fn is_pangram(&self) -> bool {
        match self {
            Pangram(_) => false,
            _ => true,
        }
    }
}

impl PartialEq for WordResult {
    fn eq(&self, other: &Self) -> bool {
        self.word() == other.word()
    }
}

impl Ord for WordResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.word().cmp(&other.word())
    }
}

impl PartialOrd for WordResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.word().partial_cmp(&other.word())
    }
}

impl Eq for WordResult { }

use argh::FromArgs;
use solver::Solution;

#[derive(FromArgs)]
/// A Spelling Bee solver
pub struct Config {

    #[argh(positional)]
    /// the letter required in all words
    required_letter: char,

    #[argh(positional)]
    /// the 6 other allowed letters
    other_letters: String,

    #[argh(option)]
    /// path to a custom dictionary file
    dict: Option<String>,

    /// when on, only the stats for the solution are output (default off)
    #[argh(switch)]
    no_words_output: bool,
    // todo make this match the beesolver-go behavior (option with default to false)
}

impl Config {

    fn validate(&self) -> Result<(), String> {
        if let Some(path) = &self.dict {
            if !std::path::Path::new(path).exists() {
                return Err(format!("dictionary not found: '{path}'"));
            }
        }
        Ok(())
    }

    pub fn required_letter(&self) -> char {
        self.required_letter
    }

    pub fn other_letters(&self) -> &String {
        &self.other_letters
    }

    pub fn no_words_output(&self) -> bool {
        self.no_words_output
    }
}

pub fn run(config: &Config) -> Result<Solution, Box<dyn Error>> {
    use puzzle::Puzzle;

    config.validate()?;

    let puzzle = Puzzle::from(config.required_letter, &config.other_letters)?;
    let dict = match &config.dict {
        Some(path) => Dictionary::load_path(path.as_str())?,
        None => Dictionary::load()?,
    };
    let solver = Solver::new(dict, puzzle);
    Ok(solver.solve())
}
