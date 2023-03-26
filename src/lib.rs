mod puzzle;
mod dictionary;
mod solver;

use std::{error::Error, cmp::Ordering};

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

pub fn run() -> Result<(), Box<dyn Error>> {
    use puzzle::Puzzle;

    let puzzle = Puzzle::from('d', "ogselm")?;
    let dict = Dictionary::load()?;
    let solver = Solver::new(dict, puzzle);
    let results = solver.solve();
    
    println!("Found a bunch of words! {}", results.len());
    
    Ok(())
}
