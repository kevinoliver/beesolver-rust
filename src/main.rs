use std::error::Error;

mod beesolver;

// todo mod dictionary
// todo mod solver
// todo mod result

fn main() -> Result<(), Box<dyn Error>> {
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

