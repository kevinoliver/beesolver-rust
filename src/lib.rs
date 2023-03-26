mod puzzle;
mod dictionary;

use std::error::Error;

use crate::dictionary::Dictionary;

#[derive(PartialEq, Debug)]
pub enum WordResult {
    Invalid,
    Valid,
    Pangram,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    use puzzle::Puzzle;

    let puzzle = Puzzle::new('d', "ogselm")?;
    let dict = Dictionary::load()?;

    let out = match puzzle.result_for("dogs") {
        WordResult::Invalid => "no",
        WordResult::Valid => "yep",
        WordResult::Pangram => "yep pangram",
    };
    println!("Result: {}", out);
    
    Ok(())
}
