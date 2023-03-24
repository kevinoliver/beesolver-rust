use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub enum WordResult {
    Invalid,
    Valid,
    Pangram,
}

pub struct Puzzle {
    required_letter: char,
    allowed_letters: HashSet<char> 
}

// todo puzzle unit tests
// todo this should be a separate file right? 
impl Puzzle {
    // todo rename to `from`
    // todo should the error be of type `&str`?
    pub fn new(required_letter: char, other_letters: &str) -> Result<Puzzle, String> {
        if other_letters.len() != 6 {
            return Err(format!("Must have 6 other letters, found {}", other_letters.len()))
        }
    
        let mut allowed_letters = HashSet::new();
        for ch in other_letters.chars() {
            if !allowed_letters.insert(ch) {
                return Err(format!("other letters cannot have any duplicates: '{ch}'"))
            }
        }
        if !allowed_letters.insert(required_letter) {
            return Err(format!("other letters cannot contain the required letter: '{required_letter}'"))
        }
    
        Ok(Puzzle { required_letter, allowed_letters })
    }

    pub fn result_for(&self, candidate: &str) -> WordResult {
        let mut found_required = false;
        let mut all_are_allowed = true;
        for candidate_letter in candidate.chars() {
            if candidate_letter == self.required_letter {
                found_required = true;
            }
            if !self.allowed_letters.contains(&candidate_letter) {
                all_are_allowed = false;
                break;
            }
        }
        if found_required && all_are_allowed {
            let mut unique_letters = HashSet::new();
            for ch in candidate.chars() {
                unique_letters.insert(ch);
            }
            if unique_letters.len() == 7 {
                WordResult::Pangram
            } else {
                WordResult::Valid
            }
        } else {
            WordResult::Invalid
        }
    }
    
}

#[cfg(test)]
mod tests {
    use crate::beesolver::{Puzzle, WordResult};

    #[test]
    fn puzzle_new_other_letters_must_have_6_letters() {
        assert!(Puzzle::new('z', "12345").is_err());
        assert!(Puzzle::new('z', "1234567").is_err());
    }

    #[test]
    fn puzzle_new_other_letters_cannot_have_duplicates() {
        assert!(Puzzle::new('z', "123455").is_err());
    }

    #[test]
    fn puzzle_new_other_letters_cannot_have_required_letter() {
        assert!(Puzzle::new('z', "12345z").is_err());
    }

    #[test]
    fn puzzle_result_for_valid_words() {
        let puzzle = Puzzle::new('d', "ogselm").unwrap();
        assert_eq!(WordResult::Valid, puzzle.result_for("dogs"));
        assert_eq!(WordResult::Valid, puzzle.result_for("doom"));
        assert_eq!(WordResult::Valid, puzzle.result_for("does"));
        assert_eq!(WordResult::Valid, puzzle.result_for("moods"));
    }

    #[test]
    fn puzzle_result_for_pangrams() {
        let puzzle = Puzzle::new('d', "ogselm").unwrap();
        assert_eq!(WordResult::Pangram, puzzle.result_for("dogselm"));
        assert_eq!(WordResult::Pangram, puzzle.result_for("dogselmm"));
        assert_eq!(WordResult::Pangram, puzzle.result_for("dogselmdogselm"));
    }

    #[test]
    fn puzzle_result_for_words_missing_required_letter() {
        let puzzle = Puzzle::new('d', "ogselm").unwrap();
        assert_eq!(WordResult::Invalid, puzzle.result_for("logs"));
        assert_eq!(WordResult::Invalid, puzzle.result_for("ogselm"));
    }

    #[test]
    fn puzzle_result_for_words_with_unallowed_letters() {
        let puzzle = Puzzle::new('d', "ogselm").unwrap();
        assert_eq!(WordResult::Invalid, puzzle.result_for("dogz"));
    }

}
