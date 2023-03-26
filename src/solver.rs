use crate::{WordResult, dictionary::Dictionary, puzzle::Puzzle};

pub struct Solver {
    dict: Dictionary,
    puzzle: Puzzle,
}

impl Solver {

    pub fn new(dict: Dictionary, puzzle: Puzzle) -> Solver {
        Solver { dict, puzzle }
    }

    pub fn solve(&self) -> Vec<WordResult> {
        let mut results: Vec<WordResult> = Vec::new();

        for candidate in self.dict.words() {
            match self.puzzle.result_for(candidate) {
                WordResult::Invalid => (),
                res => results.push(res),
            }
        }

        results.sort();
        results
    }

}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::{puzzle::Puzzle, dictionary::Dictionary, WordResult};

    use super::Solver;

    #[test]
    fn test_solve() -> Result<(), Box<dyn Error>> {
        let puzzle = Puzzle::new('d', "ogselm")?;
        let dict = Dictionary::load_path("./src/test/solver_dictionary.txt")?;
        let solver = Solver::new(dict, puzzle);
        let results = solver.solve();

        assert_eq!(3, results.len());
        assert_eq!(WordResult::Valid(String::from("dogs")), results[0]);
        assert_eq!(WordResult::Valid(String::from("doom")), results[1]);
        assert_eq!(WordResult::Pangram(String::from("ogselmd")), results[2]);

        Ok(())
    }

}