use std::slice::Iter;

use crate::{WordResult, dictionary::Dictionary, puzzle::Puzzle};

pub struct Solution {
    results: Vec<WordResult>,
    num_pangrams: i32,
}

impl Solution {
    pub fn num_pangrams(&self) -> i32 {
        self.num_pangrams
    }

    pub fn num_words(&self) -> usize {
        self.results.len()
    }

    pub fn results(&self) -> Iter<WordResult> {
        self.results.iter()
    }
}

pub struct Solver {
    dict: Dictionary,
    puzzle: Puzzle,
}

impl Solver {

    pub fn new(dict: Dictionary, puzzle: Puzzle) -> Solver {
        Solver { dict, puzzle }
    }

    pub fn solve(&self) -> Solution {
        let mut results: Vec<WordResult> = Vec::new();
        let mut pangrams = 0;

        for candidate in self.dict.words() {
            let res = self.puzzle.result_for(candidate);
            match res {
                WordResult::Invalid => (),
                WordResult::Valid(_) => results.push(res),
                WordResult::Pangram(_) => {
                    pangrams += 1;
                    results.push(res);
                },
            }
        }
        results.sort();
        Solution { 
            results,
            num_pangrams: pangrams,
        }
    }

}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{puzzle::Puzzle, dictionary::Dictionary, WordResult};

    use super::Solver;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let puzzle = Puzzle::from('d', "ogselm")?;
        let dict = Dictionary::load_path("./src/test/solver_dictionary.txt")?;
        let solver = Solver::new(dict, puzzle);
        let solution = solver.solve();

        assert_eq!(3, solution.num_words());
        assert_eq!(1, solution.num_pangrams());

        let results: Vec<&WordResult> = solution.results().collect();
        assert_eq!(WordResult::Valid(String::from("dogs")), *results[0]);
        assert_eq!(WordResult::Valid(String::from("doom")), *results[1]);
        assert_eq!(WordResult::Pangram(String::from("ogselmd")), *results[2]);

        Ok(())
    }

}