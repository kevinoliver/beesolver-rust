use std::process;

use beesolver::{Config, Metadata};
use beesolver::solver::Solution;

fn main() {
    let config: beesolver::Config = argh::from_env();
    match beesolver::run(&config) {
        Ok((solution, metadata)) => 
            print_solution(&config, solution, metadata),
        Err(err) => {
            println!("{err}");
            process::exit(1);
        },
    }
}

fn print_solution(config: &Config, solution: Solution, metadata: Metadata) {
    use beesolver::WordResult;

    println!("🐝");
    println!("Hello and welcome to Spelling Bee Solver");
    println!("🐝🐝");
    println!("🐝🐝🐝");
    println!("Required Letter:    {}", config.required_letter());
    println!("Other Letters:      {}", config.other_letters());
    println!("Dictionary:         {}", metadata.dictionary_name);
    println!("Dictionary words:   {}", metadata.dictionary_size);
    println!("🐝🐝🐝🐝");
    println!("Solved!");
    println!();
    println!("  Words: {}", solution.num_words());
    println!("  Pangrams: {}", solution.num_pangrams());
    println!("  Time loading dictionary: {} ms", metadata.loading_dictionary.as_millis());
    println!("  Time solving: {} ms", metadata.solving.as_millis());
    println!("🐝🐝🐝🐝🐝");
    if config.words_output() {
        for res in solution.results() {
            match res {
                WordResult::Invalid => (),
                WordResult::Valid(word) => println!("{word}"),
                WordResult::Pangram(word) => println!("{word} 🍳"),
            } 
        }
    }
}
