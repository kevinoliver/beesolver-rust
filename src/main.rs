use std::process;

use beesolver::Config;
use beesolver::solver::Solution;

fn main() {
    let config: beesolver::Config = argh::from_env();
    match beesolver::run(&config) {
        Ok(solution) => 
            print_solution(&config, solution),
        Err(err) => {
            println!("{err}");
            process::exit(1);
        },
    }
}

fn print_solution(config: &Config, solution: Solution) {
    use beesolver::WordResult;

    println!("🐝");
    println!("Hello and welcome to Spelling Bee Solver");
    println!("🐝🐝");
    println!("🐝🐝🐝");
    println!("Required Letter:    {}", config.required_letter());
    println!("Other Letters:      {}", config.other_letters());
    // todo wire up the dictionary name and size
    // println!("Dictionary:         {}", todo!());
    // println!("Dictionary words:   {}", todo!());
    println!("🐝🐝🐝🐝");
    println!("Solved!");
    println!();
    println!("  Words: {}", solution.num_words());
    println!("  Pangrams: {}", solution.num_pangrams());
    // todo load times
//   Time loading dictionary: xyz ms
//   Time solving: xyz ms
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
