mod beesolver;

use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    use beesolver::{Puzzle, WordResult};

    println!("Hello, world!");
    let puzzle = Puzzle::new('d', "ogselm")?;

    let out = match puzzle.result_for("dogs") {
        WordResult::Invalid => "no",
        WordResult::Valid => "yep",
        WordResult::Pangram => "yep pangram",
    };
    println!("Result: {}", out);
    
    Ok(())
}
