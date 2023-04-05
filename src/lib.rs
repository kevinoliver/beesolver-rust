pub mod puzzle;
pub mod dictionary;
pub mod solver;

use std::{error::Error, cmp::Ordering, time::{Duration, Instant},};

use crate::{dictionary::Dictionary, solver::Solver};

// todo not sure about these Strings
#[derive(Debug)]
pub enum WordResult {
    Invalid,
    Valid(String),
    Pangram(String),
}

use WordResult::*;

impl WordResult {
    fn word(&self) -> Option<&str> {
        match self {
            Invalid => None,
            Valid(w) => Some(w),
            Pangram(w) => Some(w),
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

    /// when off, only the stats for the solution are output (default on)
    #[argh(option, default="true")]
    words_output: bool,
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

    pub fn other_letters(&self) -> &str {
        &self.other_letters[..]
    }

    pub fn words_output(&self) -> bool {
        self.words_output
    }
}

pub struct Metadata {
    pub dictionary_size: usize,
    pub dictionary_name: String,
    pub loading_dictionary: Duration,
    pub solving: Duration,
}

pub fn run(config: &Config) -> Result<(Solution, Metadata), Box<dyn Error>> {
    use puzzle::Puzzle;

    config.validate()?;

    let puzzle = Puzzle::from(config.required_letter, &config.other_letters)?;
    let start = Instant::now();
    let dict = match &config.dict {
        Some(path) => Dictionary::load_path(path.as_str())?,
        None => Dictionary::load()?,
    };
    let dict_size = dict.num_words();
    let dict_name = dict.name();
    let loading_dictionary = start.elapsed();
    let solver = Solver::new(dict, puzzle);

    let start = Instant::now();
    let solution = solver.solve();
    let solving = start.elapsed();

    Ok((solution, Metadata { 
        dictionary_size: dict_size,
        dictionary_name: dict_name,
        loading_dictionary, 
        solving 
    }))
}
